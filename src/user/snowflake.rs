use std::time::{SystemTime, UNIX_EPOCH};
use super::user_error::{UserError,Result};

const INIT_EPOCH: i64 = 1673701152000;
const WORKER_ID_LEN: i64 = 5;
const DATA_CENTER_LEN: i64 = 5;
const SEQUENCE_LEN: i64 = 12;
const WORKER_ID_SHIFT: i64 = SEQUENCE_LEN;
const DATA_CENTER_SHIFT: i64 = SEQUENCE_LEN + WORKER_ID_LEN;
const TIME_STAMP_SHIFT: i64 = SEQUENCE_LEN + WORKER_ID_LEN + DATA_CENTER_LEN;

const DATA_CENTER_MAX: i64 = -1 ^ (-1 << DATA_CENTER_LEN);
const WORKER_ID_MAX: i64 = -1 ^ (-1 << WORKER_ID_LEN);

pub struct Snowflake {
    last_time_stamp: i64,
    data_center_id: i64,
    worker_id: i64,
    sequence: i64,
}
impl Snowflake {
    pub fn new(worker_id: i64, data_center_id: i64) -> Result<Self> {
        if worker_id < 0 || worker_id > WORKER_ID_MAX {
            return Err(UserError::WorkerIdLengthError);
        }
        if worker_id < 0 || worker_id > DATA_CENTER_MAX {
            return Err(UserError::DataCenterLengthError);
        }
        Ok(Snowflake {
            last_time_stamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis().try_into()?,
            data_center_id,
            worker_id,
            sequence: 0,
        })
    }
    pub fn generate(&mut self) -> Result<String> {
        if self.worker_id < 0 || self.worker_id > WORKER_ID_MAX {
            return Err(UserError::WorkerIdLengthError);
        }
        if self.worker_id < 0 || self.worker_id > DATA_CENTER_MAX {
            return Err(UserError::DataCenterLengthError);
        }
        let mut time: i64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis().try_into()?;
        if time < self.last_time_stamp {
            return Err(UserError::ClockBack);
        } else if time == self.last_time_stamp {
            self.sequence += 1;
            if self.sequence == 0 {
                time = next_millis(&self.last_time_stamp)?;
            }
        } else {
            self.sequence = 0;
        }
        self.last_time_stamp = time;
        Ok((((time - INIT_EPOCH) << TIME_STAMP_SHIFT) | (self.data_center_id << DATA_CENTER_SHIFT) | (self.worker_id << WORKER_ID_SHIFT) | self.sequence).to_string())
    }
}
fn next_millis(last_time: &i64) -> Result<i64> {
    let mut time: i64 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis().try_into()?;
    while time <= *last_time {
        time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis().try_into()?;
    }
    Ok(time)
}





