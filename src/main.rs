use chrono::Local;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    print!("{}[2J", 27 as char);
    start_clock();
}

fn get_current_time() -> String {
    let date: chrono::DateTime<Local> = Local::now();
    let formatted: String = format!("{}", date.format("%H:%M:%S"));
    return formatted;
}

fn start_clock() {
    let now = SystemTime::now();
    loop {
        sleep(Duration::new(1, 0));
        match now.elapsed() {
            Ok(elapsed) => {
                println!(
                    "current time: {0}  elapsed: {1}",
                    get_current_time(),
                    elapsed.as_secs()
                );
                print!("{}[2J", 27 as char);
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
    }
}
