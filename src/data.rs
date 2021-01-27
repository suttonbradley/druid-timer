use druid::Data;
use std::time;

/// Holds data for TimerWidget.
// `last_started` is the `SystemTime` when the timer was last started/unpaused.
// `last_remaining` is the `Duration` remaining on the timer when the timer was last started/unpaused.
// The TimerWidget that owns this TimerData will display (last_remaining - (now - last_started)).
#[derive(Clone, Data)]
pub struct TimerData {
    last_started: time::SystemTime,
    last_remaining: time::Duration,
    running: bool,
}

impl TimerData {
    pub fn new(
        last_started: time::SystemTime,
        last_remaining: time::Duration,
        running: bool,
    ) -> Self {
        TimerData {
            last_started,
            last_remaining,
            running,
        }
    }

    pub fn to_string(&self) -> String {
        let secs_remaining = if !self.running {
            self.last_remaining.as_secs()
        } else {
            self.last_remaining
                .checked_sub(
                    time::SystemTime::now()
                        .duration_since(self.last_started)
                        .unwrap(),
                )
                .unwrap_or(time::Duration::new(0, 0))
                .as_secs()
        };

        format!("{:02}:{:02}", secs_remaining / 60, secs_remaining % 60)
    }

    pub fn timed_out(&self) -> bool {
        self.last_started + self.last_remaining <= time::SystemTime::now()
    }

    pub fn resume(&mut self) {
        if !self.running {
            self.last_started = time::SystemTime::now();
            self.running = true;
        }
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        if self.running {
            self.last_remaining -= time::SystemTime::now()
                .duration_since(self.last_started)
                .unwrap();
            self.running = false
        }
    }
}
