use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration};

pub fn run_game() -> Result<T, Error>{
    let mut i = 0;
    let interval = Duration::from_secs(1);

    print!("Starting game: \n");
    loop{
        io::stdout().flush()?;
        print!("\r{} ", i);
        i += 1;
        sleep(interval);
    }
}