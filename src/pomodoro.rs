use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

enum Timer {
    Pomodoro,
    Break,
}

fn run_timer(duration: f32, timer: Timer) -> Result<(), Box<dyn std::error::Error>> {
    let seconds = (duration * 60.0) as u64;

    let pb = ProgressBar::new(seconds);
    match timer {
        Timer::Pomodoro => {
            pb.set_style(ProgressStyle::with_template(
                "🍅 [{elapsed}] [{wide_bar:.cyan/blue}] ({eta})",
            )?);
        }
        Timer::Break => {
            pb.set_style(ProgressStyle::with_template(
                "☕ [{elapsed}] [{wide_bar:.yellow/orange}] ({eta})",
            )?);
        }
    }

    for _ in 0..seconds {
        thread::sleep(Duration::from_secs(1));
        pb.inc(1);
    }
    pb.finish_with_message("Timer expired!");
    Ok(())
}

pub fn run_pomodoro(duration: f32) -> Result<(), Box<dyn std::error::Error>> {
    run_timer(duration, Timer::Pomodoro)?;
    Ok(())
}

pub fn run_break(duration: f32) -> Result<(), Box<dyn std::error::Error>> {
    run_timer(duration, Timer::Break)?;
    Ok(())
}

