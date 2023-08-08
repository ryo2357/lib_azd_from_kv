use log::{error, info};

use super::config::Kv7500Config;
use super::connection::HotLinkConnection;

// pub enum CommandResponse {
//     Ok,
//     TimeOut,
//     Failure,
// }

pub struct AzdKvDirectClient {
    config: Kv7500Config,
    pub connection: HotLinkConnection,
    state: AzdState,
}

impl AzdKvDirectClient {
    pub async fn create(config: Kv7500Config) -> anyhow::Result<Self> {
        let mut connection = HotLinkConnection::connect(&config).await?;
        let state = AzdState::create(&mut connection).await?;

        Ok(Self {
            config,
            connection,
            state,
        })
    }

    pub async fn info_state(&mut self) -> anyhow::Result<()> {
        self.update_state().await?;
        info!("{:?}", self.state);

        Ok(())
    }

    // TODO:タイムアウトは考慮されていない
    pub async fn wait_can_command(&mut self) -> anyhow::Result<()> {
        loop {
            info!("loop in wait_can_command");
            self.update_state().await?;

            info!("{:?}", self.state);
            if self.state.is_ready && !self.state.is_alarm {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
        Ok(())
    }

    pub async fn wait_start_move(&mut self) -> anyhow::Result<()> {
        loop {
            info!("loop in wait start move");
            self.update_state().await?;

            info!("{:?}", self.state);
            if self.state.is_move && !self.state.is_alarm {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
        Ok(())
    }

    pub async fn check_finish_move(&mut self) -> anyhow::Result<()> {
        loop {
            info!("loop in check finish move");
            self.update_state().await?;

            info!("{:?}", self.state);
            if self.state.is_finish_move && !self.state.is_alarm {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
        Ok(())
    }
    pub async fn direct_move(&mut self, point: i32, speed: i32) -> anyhow::Result<()> {
        self.wait_can_command().await?;

        // トリガーのオフ（念のため）
        let command = make_command_direct_move(false, point, speed);
        info!("send command :{:?}", command);
        let response = self.connection.send_command(command).await?;
        if response != "OK\r\n" {
            error!("response is {:?}", response);
            anyhow::bail!("設定コマンド送信失敗")
        }

        // self.wait_can_command().await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // トリガーオン
        let command = make_command_direct_move(true, point, speed);
        info!("send command :{:?}", command);
        let response = self.connection.send_command(command).await?;
        if response != "OK\r\n" {
            error!("response is {:?}", response);
            anyhow::bail!("動作コマンド送信失敗")
        }

        // move のチェック
        self.wait_start_move().await?;

        let command = make_command_direct_move(false, point, speed);
        info!("send command :{:?}", command);
        let response = self.connection.send_command(command).await?;
        if response != "OK\r\n" {
            error!("response is {:?}", response);
            anyhow::bail!("設定コマンド送信失敗")
        }

        Ok(())
    }

    pub async fn throw_command_direct_move(
        &mut self,
        point: i32,
        speed: i32,
    ) -> anyhow::Result<()> {
        self.wait_can_command().await?;

        // トリガーのオフ（念のため）
        let command = make_command_direct_move(false, point, speed);
        info!("send command :{:?}", command);
        let response = self.connection.send_command(command).await?;
        if response != "OK\r\n" {
            error!("response is {:?}", response);
            anyhow::bail!("設定コマンド送信失敗")
        }

        // self.wait_can_command().await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // トリガーオン
        let command = make_command_direct_move(true, point, speed);
        info!("send command :{:?}", command);
        let response = self.connection.send_command(command).await?;
        if response != "OK\r\n" {
            error!("response is {:?}", response);
            anyhow::bail!("動作コマンド送信失敗")
        }

        Ok(())
    }
    async fn update_state(&mut self) -> anyhow::Result<()> {
        let state = AzdState::create(&mut self.connection).await?;
        self.state = state;

        Ok(())
    }

    pub async fn throw_command_direct_move_trigger_off(
        &mut self,
        point: i32,
        speed: i32,
    ) -> anyhow::Result<()> {
        self.wait_can_command().await?;

        let command = make_command_direct_move(false, point, speed);
        info!("send command :{:?}", command);
        let response = self.connection.send_command(command).await?;
        if response != "OK\r\n" {
            error!("response is {:?}", response);
            anyhow::bail!("設定コマンド送信失敗")
        }

        Ok(())
    }
}

fn make_command_direct_move(trigger: bool, point: i32, speed: i32) -> String {
    let trigger = match trigger {
        true => 0b0000_0001_0000_0000u16,
        false => 0b0000_0000_0000_0000u16,
    };

    let driving_method = 1u16; //絶対位置決め

    // let detect_position = 5000i32; // 目標位置
    //1000 ⇒ 10㎜
    let detect_position = point;
    let detect_position = detect_position.to_le_bytes();
    let detect_position_lower = u16::from_le_bytes([detect_position[0], detect_position[1]]);
    let detect_position_upper = u16::from_le_bytes([detect_position[2], detect_position[3]]);

    // 1000 ⇒ 1mm/sec

    let detect_speed = speed; // 移動速度
    let detect_speed = detect_speed.to_le_bytes();
    let detect_speed_lower = u16::from_le_bytes([detect_speed[0], detect_speed[1]]);
    let detect_speed_upper = u16::from_le_bytes([detect_speed[2], detect_speed[3]]);

    let acceleration_rate = 1_000_000i32;
    let acceleration_rate = acceleration_rate.to_le_bytes();
    let acceleration_rate_lower = u16::from_le_bytes([acceleration_rate[0], acceleration_rate[1]]);
    let acceleration_rate_upper = u16::from_le_bytes([acceleration_rate[2], acceleration_rate[3]]);

    let deceleration_rate = 1_000_000i32;
    let deceleration_rate = deceleration_rate.to_le_bytes();
    let deceleration_rate_lower = u16::from_le_bytes([deceleration_rate[0], deceleration_rate[1]]);
    let deceleration_rate_upper = u16::from_le_bytes([deceleration_rate[2], deceleration_rate[3]]);

    let driving_current = 1_000u16;

    let command: String = format!(
        "WRS W1E.H 11 {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X}\r",
        trigger,
        driving_method,
        detect_position_lower,
        detect_position_upper,
        detect_speed_lower,
        detect_speed_upper,
        acceleration_rate_lower,
        acceleration_rate_upper,
        deceleration_rate_lower,
        deceleration_rate_upper,
        driving_current
    );
    command
}
#[derive(Debug)]
struct AzdState {
    is_ready: bool,
    is_alarm: bool,
    is_move: bool,
    is_finish_move: bool,
    detection_position: i32,
    detection_velocity: i32,
}
impl AzdState {
    async fn create(connection: &mut HotLinkConnection) -> anyhow::Result<Self> {
        let response = connection.send_command("RDS W00.H 8\r".to_string()).await?;

        info!("get response");
        let state = AzdState::parse_response(response)?;
        Ok(state)
    }

    fn parse_response(response: String) -> anyhow::Result<Self> {
        let iter = response.split_whitespace();
        let mut vec: Vec<u16> = Vec::new();
        for i in iter {
            let device: u16 = u16::from_str_radix(i, 16)?;
            // info!("{:?}", device);
            vec.push(device);
        }
        info!("固定I/O::{:016b}", vec[2]);
        let is_ready = vec[2] & (1 << 5) != 0;
        let is_alarm = vec[2] & (1 << 7) != 0;

        let is_move = vec[2] & (1 << 1) != 0;
        let is_finish_move = vec[2] & (1 << 2) != 0;

        let detection_position = i32_from_2u16(vec[4], vec[5]);
        let detection_velocity = i32_from_2u16(vec[6], vec[7]);

        Ok(Self {
            is_ready,
            is_alarm,
            is_move,
            is_finish_move,
            detection_position,
            detection_velocity,
        })
    }
}

fn i32_from_2u16(lower: u16, upper: u16) -> i32 {
    // 上位下位の概念がいまいち分からんがオリエンタルモーターのUMに合わせた形に
    let upper = upper.to_le_bytes();
    let lower = lower.to_le_bytes();
    i32::from_le_bytes([lower[0], lower[1], upper[0], upper[1]])
}
