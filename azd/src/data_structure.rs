pub type ReceiveBuff = [u8; 56];
pub type SendBuff = [u8; 40];

pub struct ReceiveData {
    pub remote_io: u16,
    pub driving_data: u16,
    pub fixed_io: u16,
    pub current_alarm: u16,
    pub detection_position: i32,
    pub detection_speed: u32,
    pub command_position: i32,
    pub torque_monitor: i16,
    pub cst_driving_current: u16,
    pub information: u32,
    pub reservation: u16,
    pub read_parameter_id: u16,
    pub rw_status: u16,
    pub write_parameter_id: u16,
    pub read_data: u32,
    pub optional_monitor_0: u32,
    pub optional_monitor_1: u32,
    pub optional_monitor_2: u32,
    pub optional_monitor_3: u32,
}
impl ReceiveData {
    pub fn from_buff(buff: ReceiveBuff) -> Self {
        let remote_io = u16::from_le_bytes([buff[0], buff[1]]);
        let driving_data = u16::from_le_bytes([buff[2], buff[3]]);
        let fixed_io = u16::from_le_bytes([buff[4], buff[5]]);
        let current_alarm = u16::from_le_bytes([buff[6], buff[7]]);
        let detection_position = i32::from_le_bytes([buff[8], buff[9], buff[10], buff[11]]);
        let detection_speed = u32::from_le_bytes([buff[12], buff[13], buff[14], buff[15]]);
        let command_position = i32::from_le_bytes([buff[16], buff[17], buff[18], buff[19]]);
        let torque_monitor = i16::from_le_bytes([buff[20], buff[21]]);
        let cst_driving_current = u16::from_le_bytes([buff[22], buff[23]]);
        let information = u32::from_le_bytes([buff[24], buff[25], buff[26], buff[27]]);
        let reservation = u16::from_le_bytes([buff[28], buff[29]]);
        let read_parameter_id = u16::from_le_bytes([buff[30], buff[31]]);
        let rw_status = u16::from_le_bytes([buff[32], buff[33]]);
        let write_parameter_id = u16::from_le_bytes([buff[34], buff[35]]);
        let read_data = u32::from_le_bytes([buff[36], buff[37], buff[38], buff[39]]);
        let optional_monitor_0 = u32::from_le_bytes([buff[40], buff[41], buff[42], buff[43]]);
        let optional_monitor_1 = u32::from_le_bytes([buff[44], buff[45], buff[46], buff[47]]);
        let optional_monitor_2 = u32::from_le_bytes([buff[48], buff[49], buff[50], buff[51]]);
        let optional_monitor_3 = u32::from_le_bytes([buff[52], buff[53], buff[54], buff[55]]);
        Self {
            remote_io,
            driving_data,
            fixed_io,
            current_alarm,
            detection_position,
            detection_speed,
            command_position,
            torque_monitor,
            cst_driving_current,
            information,
            reservation,
            read_parameter_id,
            rw_status,
            write_parameter_id,
            read_data,
            optional_monitor_0,
            optional_monitor_1,
            optional_monitor_2,
            optional_monitor_3,
        }
    }
}

pub struct SendData {
    pub remote_io: u16,
    pub driving_data: u16,
    pub fixed_io: u16,
    pub current_alarm: u16,
    // ダイレクトデータ運転はしないという認識
    pub read_parameter_id: u16,
    pub write_request: u16,
    pub write_parameter_id: u16,
    pub write_data: u32,
}

impl SendData {
    pub fn convert_buff(&mut self) -> SendBuff {
        let mut buff: SendBuff = [0; 40];
        buff[0..2].copy_from_slice(&self.read_parameter_id.to_le_bytes());
        buff[2..4].copy_from_slice(&self.driving_data.to_le_bytes());
        buff[4..6].copy_from_slice(&self.fixed_io.to_le_bytes());
        buff[6..8].copy_from_slice(&self.current_alarm.to_le_bytes());

        buff[30..32].copy_from_slice(&self.read_parameter_id.to_le_bytes());
        buff[32..34].copy_from_slice(&self.write_request.to_le_bytes());
        buff[34..36].copy_from_slice(&self.write_parameter_id.to_le_bytes());
        buff[36..40].copy_from_slice(&self.write_data.to_le_bytes());
        buff
    }
}
