use log::info;

use super::config::Kv7500Config;
use super::connection::HotLinkConnection;

// pub enum CommandResponse {
//     Ok,
//     TimeOut,
//     Failure,
// }

pub struct AzdClient {
    config: Kv7500Config,
    pub connection: HotLinkConnection,
    state: AzdState,
}

impl AzdClient {
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
    async fn test_behavior(&mut self) -> anyhow::Result<()> {
        self.wait_ready().await?;

        // ライトリクエストON
        let response = self
            .connection
            .send_command("RDS W00.H 28\r".to_string())
            .await?;

        Ok(())
    }

    // TODO:タイムアウトは考慮されていない
    pub async fn wait_ready(&mut self) -> anyhow::Result<()> {
        self.update_state().await?;
        while !self.state.is_ready {
            info!("loop in wait ready");
            self.update_state();
        }
        Ok(())
    }

    // TODO:タイムアウトは考慮されていない
    pub async fn wait_finish_move(&mut self) -> anyhow::Result<()> {
        self.update_state().await?;
        while self.state.is_move {
            info!("loop in wait_finish_move");
            self.update_state();
        }
        Ok(())
    }

    async fn update_state(&mut self) -> anyhow::Result<()> {
        let state = AzdState::create(&mut self.connection).await?;
        self.state = state;

        Ok(())
    }

    fn make_return_command(trigger: bool) -> String {
        let trigger = match trigger {
            true => 0b0000_0001_0000_0000u16,
            false => 0b0000_0000_0000_0000u16,
        };

        let driving_method = 1u16; //絶対位置決め

        let detect_position = 0i32; // 目標位置
        let detect_position = detect_position.to_le_bytes();
        let detect_position_lower = u16::from_le_bytes([detect_position[0], detect_position[1]]);
        let detect_position_upper = u16::from_le_bytes([detect_position[2], detect_position[3]]);

        let detect_speed = 1000i32; // 移動速度
        let detect_speed = detect_speed.to_le_bytes();
        let detect_speed_lower = u16::from_le_bytes([detect_speed[0], detect_speed[1]]);
        let detect_speed_upper = u16::from_le_bytes([detect_speed[2], detect_speed[3]]);

        let command: String = format!(
            "WRS W1E.H 6 {:04X} {:04X} {:04X} {:04X} {:04X} {:04X}\r",
            trigger,
            driving_method,
            detect_position_lower,
            detect_position_upper,
            detect_speed_lower,
            detect_speed_upper
        );
        command
    }

    fn make_move_command(trigger: bool) -> String {
        let trigger = match trigger {
            true => 0b0000_0001_0000_0000u16,
            false => 0b0000_0000_0000_0000u16,
        };

        let driving_method = 1u16; //絶対位置決め

        let detect_position = 5000i32; // 目標位置
        let detect_position = detect_position.to_le_bytes();
        let detect_position_lower = u16::from_le_bytes([detect_position[0], detect_position[1]]);
        let detect_position_upper = u16::from_le_bytes([detect_position[2], detect_position[3]]);

        let detect_speed = 1000i32; // 移動速度
        let detect_speed = detect_speed.to_le_bytes();
        let detect_speed_lower = u16::from_le_bytes([detect_speed[0], detect_speed[1]]);
        let detect_speed_upper = u16::from_le_bytes([detect_speed[2], detect_speed[3]]);

        let command: String = format!(
            "WRS W1E.H 6 {:04X} {:04X} {:04X} {:04X} {:04X} {:04X}\r",
            trigger,
            driving_method,
            detect_position_lower,
            detect_position_upper,
            detect_speed_lower,
            detect_speed_upper
        );
        command
    }
}

#[derive(Debug)]
struct AzdState {
    is_ready: bool,
    is_alarm: bool,
    is_move: bool,
    is_write_end: bool,
    write_parameter_id: u16,
    detection_position: i32,
    command_position: i32,
    detection_velocity: i32,
}
impl AzdState {
    async fn create(connection: &mut HotLinkConnection) -> anyhow::Result<Self> {
        let response = connection
            .send_command("RDS W00.H 28\r".to_string())
            .await?;

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
        // info!("{:?}", vec);
        let is_ready = vec[0] & (1 << 5) != 0;
        let is_alarm = vec[0] & (1 << 7) != 0;
        let is_move = vec[0] & (1 << 13) != 0;
        let is_write_end = vec[10] & (1 << 7) != 0;
        let write_parameter_id = vec[11];
        let detection_position = i32_from_2u16(vec[4], vec[5]);
        let command_position = i32_from_2u16(vec[8], vec[9]);
        let detection_velocity = i32_from_2u16(vec[6], vec[7]);

        Ok(Self {
            is_ready,
            is_alarm,
            is_move,
            is_write_end,
            write_parameter_id,
            detection_position,
            command_position,
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
