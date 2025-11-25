use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn get_time() -> String 
{
    let now: SystemTime = SystemTime::now();
    let datetime: Duration = now.duration_since(UNIX_EPOCH)
        .unwrap();

    let total_secs: u64 = datetime.as_secs();
    let hours: u64 = (total_secs / 3600) % 24;
    let minutes: u64 = (total_secs / 60) % 60;
    let seconds: u64 = total_secs % 60;

    return format!("[{:02}:{:02}:{:02}]", 
        hours, 
        minutes, 
        seconds
    );
}
