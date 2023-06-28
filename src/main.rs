use clap::Parser;

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

mod notifier;
mod timer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // destructure the args
    let Args {
        count,
        duration,
        short_break,
        long_break,
    } = args;

    let mut notifier = notifier::Notifier::new();

    for i in 1..=count {
        timer::run_pomodoro(duration)?;
        notifier.break_start();

        if i % 4 == 0 {
            timer::run_break(long_break)?;
        } else {
            timer::run_break(short_break)?;
        }
        notifier.break_end();
    }

    Ok(())
}
