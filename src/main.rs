use std::time::{Duration, SystemTime};
use std::io::{Read, stdin};
use std::sync::mpsc::{channel, TryRecvError};

fn main() {
    let mut args = std::env::args();
    args.next(); //exe name
    let Some(seconds_after_epoch) = args.next().and_then(|s| s.parse().ok()) else {
        eprintln!("{}\n\n", include_str!("../README.md"));

        eprintln!("Need target time as seconds after epoch!");
        eprintln!("For example, the year 2000 would be 946710000");
        return;
    };

    let start = SystemTime::UNIX_EPOCH.checked_add(
        Duration::from_secs(seconds_after_epoch)
    ).unwrap();

    println!("Press enter to quit");

    let mut now;
    // hide cursor
    print!("\x1b[?25l");

    let receiver = {
        let (sender, receiver) = channel::<()>();
        std::thread::spawn(move || {
            let mut buffer = [0; 1];
            let mut stdin = stdin();
            stdin.read(&mut buffer).unwrap();
            sender.send(()).unwrap();
        });
        receiver
    };

    loop {
        now = SystemTime::now();

        let elapsed = now.duration_since(start).unwrap();
        // https://www.wolframalpha.com/input?i=orbital+period+of+earth+in+seconds
        let giga_years = elapsed / 31558149;

        let decimal = giga_years.as_secs_f64();

        let nanos = giga_years.as_nanos();
        let whole = nanos / 1_000_000_000;
        let fract = nanos % 1_000_000_000;
        
        print!("{whole:#b}.{fract:b} = {decimal:<20}\r");

        match receiver.try_recv() {
            Ok(()) => break,
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
    }

    // show cursor
    print!("\x1b[?25h");
}
