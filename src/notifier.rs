use std::{ time::Duration};
use notify_rust::Notification;
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};

pub struct Notifier {
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
