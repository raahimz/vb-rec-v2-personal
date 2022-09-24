use std::thread::sleep;
use std::time::Duration;

fn main() {
    fn set_timeout(callback: fn() -> (), time: u64) -> () {
        sleep(Duration::from_secs(time));
        callback();
    }
    let mut i = 0;
    while i < 100000 {
        set_timeout(
            || {
                println!("Hello World after 3 secs");
            },
            3,
        );
        i += 1;

        if i == 3 {
            println!("RECORDER_RESTART_REQUESTED");
        }
        if i == 5 {
            println!("RECORDER_STOP_REQUESTED");
        }
    }
}
