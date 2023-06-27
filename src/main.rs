use clap::Parser;
use notify_rust::Notification;
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

/// Pomodoro timer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times pomodoro cycle
    #[arg(short, long, default_value_t = 4)]
    count: u8,

    /// Number of minutes per pomodoro cycle
    #[arg(short, long, default_value_t = 25.0)]
    duration: f32,

    /// Number of minutes per short break
    #[arg(short, long, default_value_t = 5.0)]
    short_break: f32,

    /// Number of minutes per long break
    #[arg(short, long, default_value_t = 15.0)]
    long_break: f32,
}

struct Notifier {
    notification: Notification,
}

impl Notifier {
    pub fn new() -> Notifier {
        Notifier {
            notification: Notification::new(),
        }
    }

    fn play_sound(&mut self, frequency: f32) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = SineWave::new(frequency)
            .take_duration(Duration::from_secs_f32(0.25))
            .amplify(0.10);
        sink.append(source);
        sink.sleep_until_end();
    }

    pub fn break_start(&mut self) {
        self.notification
            .summary("Pomodoro complete")
            .body("Let's take a break")
            .show()
            .unwrap();

        self.play_sound(440.0);
    }

    pub fn break_end(&mut self) {
        self.notification
            .summary("Break end")
            .body("Let's get back to work")
            .show()
            .unwrap();

        self.play_sound(700.0);
    }
}

mod pomodoro;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // destructure the args
    let Args {
        count,
        duration,
        short_break,
        long_break,
    } = args;

    let mut notifier = Notifier::new();

    for i in 1..=count {
        pomodoro::run_pomodoro(duration)?;
        notifier.break_start();

        if i % 4 == 0 {
            pomodoro::run_break(long_break)?;
        } else {
            pomodoro::run_break(short_break)?;
        }
        notifier.break_end();
    }

    Ok(())
}
