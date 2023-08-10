#![allow(dead_code)]
use mylogger::info;

fn main() -> anyhow::Result<()> {
    mylogger::init();
    // bit_verify();
    // convert_verify_4();
    func_verify();
    Ok(())
}

fn bit_verify() {
    let device = 1120u16;

    // 10001100000
    info!("{:b}", device);

    // 0000010001100000
    // 0000 0100 0110 0000
    // 5:ready
    // 6:info
    // 7:alarm
    // 10:area1
    // 13:move
    info!("{:0>16b}", device);
    if device & (1 << 5) != 0 {
        info!("ready is on");
    }

    if device & (1 << 6) != 0 {
        info!("info is on");
    }
    if device & (1 << 7) != 0 {
        info!("alarm is on");
    }
}

fn convert_verify() {
    // let device = 1120u16;
    // info!("u16:{:?}", device);
    // let array = device.to_le_bytes();
    // info!("[u8]:{:?}", array);
    // let device_i16 = i16::from_le_bytes(array);
    // info!("i16:{:?}", device_i16);

    // let device = -1120i16;
    // info!("i16:{:?}", device);
    // let array = device.to_le_bytes();
    // info!("[u8]:{:?}", array);
    // let device_u16 = u16::from_le_bytes(array);
    // info!("u16:{:?}", device_u16);

    let array: [u8; 2] = [160, 251];
    info!("[u8]:{:?}", array);
    let device_u16 = u16::from_le_bytes(array);
    info!("u16:{:?}", device_u16);
    let array = device_u16.to_le_bytes();
    info!("[u8]:{:?}", array);
    let device_i16 = i16::from_le_bytes(array);
    info!("i16:{:?}", device_i16);
}

fn convert_verify_2() {
    // let true_num = -5000i32;
    let true_num = 5000i32;
    info!("i32:{:?}", true_num);

    let array = true_num.to_le_bytes();

    info!("[u8]:{:?}", array);

    let device_u16_upper = u16::from_le_bytes([array[0], array[1]]);
    let device_u16_lowwr = u16::from_le_bytes([array[2], array[3]]);
    info!("u16_upper:{:?}", device_u16_upper);
    info!("u16_lower:{:?}", device_u16_lowwr);

    let convert_i32 = i32_from_2u16(device_u16_upper, device_u16_lowwr);
    info!("convert i32:{:?}", convert_i32);
}

fn i32_from_2u16(upper: u16, lower: u16) -> i32 {
    let upper = upper.to_le_bytes();
    let lower = lower.to_le_bytes();
    i32::from_le_bytes([upper[0], upper[1], lower[0], lower[1]])
}

fn convert_verify_3() {
    let command = "WRS W1F.H 9 0001\r".to_string();
    info!("command: {:?}", command);

    let command = String::from("WRS W1F.H 9 0001\r");
    info!("command String::from: {:?}", command);

    let injection = String::from("0001");

    let command = format!("WRS W1F.H 9 {}\r", injection);
    info!("command from format!: {:?}", command);

    let injection: u16 = 0b0000_0001;
    let injection: String = format!("{:04x}", injection);
    let command = format!("WRS W1F.H 9 {}\r", injection);
    info!("command from format!: {:?}", command);

    let command = format!(
        "WRS W1F.H 9 {:04x} {:04x} {:04x} {:04x}\r",
        0b1100_0100_0100_0001u16, 36u16, 0x55u16, 0x5Au16
    );
    info!("command from format!: {:?}", command);

    let command = format!(
        "WRS W1F.H 9 {:04X} {:04X} {:04X} {:04X}\r",
        0b1100_0100_0100_0001u16, 36u16, 0x55u16, 0x5Au16
    );
    info!("command from format!: {:?}", command);

    let command = format!(
        "WRS W1F.H 9 {:04X} {:04X} {:04X} {:04X}\r",
        0b1100_0100_0100_0001u16, 36u16, 0x55u16, 0x5Au16
    );
    info!("command from format!: {:?}", command);
}

fn convert_verify_4() {
    let test_num = 5000i32;
    let test_num2 = -5000i32;

    let array = test_num.to_le_bytes();
    info!("5000: {:?}", array);

    let array = test_num2.to_le_bytes();
    info!("-5000: {:?}", array);

    let array = test_num.to_le_bytes();
    let lower = u16::from_le_bytes([array[0], array[1]]);
    let upper = u16::from_le_bytes([array[2], array[3]]);
    info!("5000: {:04X} {:04X}", lower, upper);

    let array = test_num2.to_le_bytes();
    let lower = u16::from_le_bytes([array[0], array[1]]);
    let upper = u16::from_le_bytes([array[2], array[3]]);
    info!("-5000: {:04X} {:04X}", lower, upper);
}

fn func_verify() {
    let command = make_move_command(true);
    info!("command: {:?}", command);

    let command = make_move_command(false);
    info!("command: {:?}", command);

    let command = make_return_command(true);
    info!("command: {:?}", command);

    let command = make_return_command(false);
    info!("command: {:?}", command);
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

    let detect_speed = 1000i32; // 目標位置
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
