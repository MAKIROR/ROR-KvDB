use std::time::{SystemTime, UNIX_EPOCH};
use super::user_error::{UserError,Result};

const INIT_EPOCH: i64 = 1673701152000;
const TIME_STAMP_LEN: isize = 41;
const MACHINE_ID_LEN: isize = 5;
const DATA_CENTER_LEN: isize = 5;
const SEQUENCE_LEN: isize = 12;
const MACHINE_ID_SHIFT: i64 = SEQUENCE_LEN;
const DATA_CENTER_SHIFT: i64 = SEQUENCE_LEN + MACHINE_ID_LEN;
const TIME_STAMP_SHIFT: i64 = SEQUENCE_LEN + MACHINE_ID_LEN + DATA_CENTER_LEN;

const TIME_STAMP_MAX: i64 = -1 ^ (-1 << TIME_STAMP_LEN);
const DATA_CENTER_MAX: i64 = -1 ^ (-1 << DATA_CENTER_LEN);
const MACHINE_ID_MAX: i64 = -1 ^ (-1 << MACHINE_ID_LEN);
const SEQUENCE_MAX: i64 = -1 ^ (-1 << SEQUENCE_LEN); 

pub struct Snowflake {
    last_time_stamp: i64,
    data_center_id: i64,
    machine_id: i64,
    sequence: i64,
}
impl Snowflake {
    pub fn generate(&mut self) -> Result<String> {
        if self.machine_id < 0 || self.machine_id > 31 {
            return Err(UserError::MachineIdLengthError);
        }
        let mut time: i64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        if time < self.last_time_stamp {
            return Err(UserError::ClockBack);
        } else if time == self.last_time_stamp {
            self.sequence += 1;
            if self.sequence == 0 {
                time = Self::next_millis(&self.last_time_stamp);
            }
        } else {
            self.sequence = 0;
        }
        self.last_time_stamp = time;
        Ok(((time - INIT_EPOCH) << TIME_STAMP_SHIFT) | (self.data_center_id << DATA_CENTER_SHIFT) | (self.machine_id << MACHINE_ID_SHIFT) | self.sequence)
    }
    fn next_millis(last_time: i64) -> Result<i64> {
        let mut time: i64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        while time <= last_time {
            time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        }
        Ok(time)
    }
}





