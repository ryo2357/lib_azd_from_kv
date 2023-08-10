#![allow(dead_code)]
use dotenv::dotenv;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use azd_from_kv::{client, config};

use mylogger::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mylogger::init();
    // config_verify();
    // connect_verify().await?;
    // read_verify().await?;
    // write_verify().await?;
    // struct_verify().await?;
    // command_verification().await?;
    // command_verification_2().await?;
    client_verify_2().await?;

    Ok(())
}

fn config_verify() {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();

    info!("{:?}", config);
}

async fn connect_verify() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();
    let mut stream = TcpStream::connect(&config.address).await?;
    info!("connected");

    // 型番の問い合わせ　返却の55はkv7500
    let msg = "?K\r".to_string();
    stream.write_all(msg.as_bytes()).await?;

    let mut buf = Vec::with_capacity(4096);
    stream.read_buf(&mut buf).await?;

    let received = String::from_utf8(buf).unwrap();
    info!("receive:{}", received);

    Ok(())
}

async fn read_verify() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();
    let mut stream = TcpStream::connect(&config.address).await?;
    info!("connected");

    // 型番の問い合わせ
    // let msg = "RDS DM1000.U 28\r".to_string();
    // リンクレジスタも直接変更できる
    // let msg = "RDS W00.U 28\r".to_string();
    let msg = "RDS W00.H 28\r".to_string();
    stream.write_all(msg.as_bytes()).await?;

    let mut buf = Vec::with_capacity(4096);
    stream.read_buf(&mut buf).await?;

    let received = String::from_utf8(buf).unwrap();
    // 00032 00000 01120 00000 05000 00000 00000 00000 05000 00000 00008 00500 00000 00000 00000 00000 00000 00000 00000 00000 00350 00000 00363 00000 00000 00000 00001 00000
    // .H　こっちの方がパースしやすい気がする
    // 0020 0000 0460 0000 1388 0000 0000 0000 1388 0000 0006 01F4 0000 0000 0000 0000 0000 0000 0000 0000 0158 0000 0169 0000 0000 0000 0001 0000
    // 文字列に変換されて転送が行われる。.Uは
    info!("receive:{}", received);
    // info!("receive:{:?}", buf);
    // info!("receive len:{:?}", buf.len());

    Ok(())
}

async fn write_verify() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();
    let mut stream = TcpStream::connect(&config.address).await?;
    info!("connected");

    let msg = "WRS DM3000.H 2 0003 0020\r".to_string();
    stream.write_all(msg.as_bytes()).await?;

    let mut buf = Vec::with_capacity(4096);
    stream.read_buf(&mut buf).await?;

    let received = String::from_utf8(buf).unwrap();
    // 00032 00000 01120 00000 05000 00000 00000 00000 05000 00000 00008 00500 00000 00000 00000 00000 00000 00000 00000 00000 00350 00000 00363 00000 00000 00000 00001 00000
    // 文字列に変換されて転送が行われる。.Uは
    info!("receive:{}", received);
    // info!("receive:{:?}", buf);
    // info!("receive len:{:?}", buf.len());

    Ok(())
}

async fn struct_verify() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();
    info!("get config");
    let mut azd = client::AzdKvDirectClient::create(config).await?;

    azd.info_state().await?;

    Ok(())
}

async fn command_verification() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();
    info!("get config");
    let mut azd = client::AzdKvDirectClient::create(config).await?;

    // wait_until_enter();
    // azd.info_state().await?;

    // パラメータの書き込み
    wait_until_enter();
    let command = make_command_direct_move(false, 5000, 1000);
    info!("send command :{:?}", command);
    let response = azd.connection.send_command(command).await?;
    info!("response:{:?}", response);

    // 動作
    wait_until_enter();
    let command = make_command_direct_move(true, 5000, 1000);
    info!("send command :{:?}", command);
    let response = azd.connection.send_command(command).await?;
    info!("response:{:?}", response);

    // パラメータの書き込み（原点復帰）
    wait_until_enter();
    let command = make_command_direct_move(false, 0, 1000);
    info!("send command :{:?}", command);
    let response = azd.connection.send_command(command).await?;
    info!("response:{:?}", response);

    // 動作（原点復帰）
    wait_until_enter();
    let command = make_command_direct_move(true, 0, 1000);
    info!("send command :{:?}", command);
    let response = azd.connection.send_command(command).await?;
    info!("response:{:?}", response);

    Ok(())
}

async fn command_verification_2() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();
    info!("get config");
    let mut azd = client::AzdKvDirectClient::create(config).await?;

    // パラメータの書き込み
    wait_until_enter();
    let command = make_command_direct_move(false, 9000, 500);
    info!("send command :{:?}", command);
    let response = azd.connection.send_command(command).await?;
    info!("response:{:?}", response);

    // 動作
    wait_until_enter();
    let command = make_command_direct_move(true, 9000, 500);
    info!("send command :{:?}", command);
    let response = azd.connection.send_command(command).await?;
    info!("response:{:?}", response);

    // パラメータの書き込み（原点復帰）
    wait_until_enter();
    let command = make_command_direct_move(false, 0, 3000);
    info!("send command :{:?}", command);
    let response = azd.connection.send_command(command).await?;
    info!("response:{:?}", response);

    // 動作（原点復帰）
    wait_until_enter();
    let command = make_command_direct_move(true, 0, 3000);
    info!("send command :{:?}", command);
    let response = azd.connection.send_command(command).await?;
    info!("response:{:?}", response);

    Ok(())
}

use std::io::Write;
pub fn wait_until_enter() {
    print!("wait until press enter: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
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

async fn client_verify() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();
    info!("get config");
    let mut azd = client::AzdKvDirectClient::create(config).await?;

    // 動作
    wait_until_enter();
    azd.direct_move(9000, 500).await?;

    azd.check_finish_move().await?;

    azd.direct_move(0, 3000).await?;

    azd.check_finish_move().await?;

    Ok(())
}

async fn client_verify_2() -> anyhow::Result<()> {
    dotenv().ok();
    let config = config::AzdFromKvConfig::from_env().unwrap();
    info!("get config");
    let mut azd = client::AzdKvDirectClient::create(config).await?;
    info!("クライアント作成完了");

    // 動作
    azd.throw_command_direct_move(9000, 500).await?;
    info!("動作指令完了");
    azd.wait_start_move().await?;
    azd.throw_command_direct_move_trigger_off(9000, 500).await?;
    info!("トリガーオフ");
    azd.check_finish_move().await?;
    info!("動作完了");

    azd.throw_command_direct_move(0, 4000).await?;
    info!("基準位置復帰動作指令完了");
    azd.wait_start_move().await?;
    azd.throw_command_direct_move_trigger_off(0, 2000).await?;
    info!("トリガーオフ");
    azd.check_finish_move().await?;
    info!("動作完了");

    Ok(())
}
