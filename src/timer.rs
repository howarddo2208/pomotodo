use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

enum TimerType {
    Pomodoro,
    Break,
}

fn run_timer(duration: f32, timer_type: TimerType) -> Result<(), Box<dyn std::error::Error>> {
    let seconds = (duration * 60.0) as u64;

    let pb = ProgressBar::new(seconds);
    match timer_type {
        TimerType::Pomodoro => {
            pb.set_style(ProgressStyle::with_template(
                "🍅 [{elapsed}] [{wide_bar:.cyan/blue}] ({eta})",
            )?);
        }
        TimerType::Break => {
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
    run_timer(duration, TimerType::Pomodoro)?;
    Ok(())
}

pub fn run_break(duration: f32) -> Result<(), Box<dyn std::error::Error>> {
    run_timer(duration, TimerType::Break)?;
    Ok(())
}

