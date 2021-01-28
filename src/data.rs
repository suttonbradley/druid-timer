use druid::{Data, TimerToken};
use std::time;

#[derive(Clone, Copy, Data, PartialEq)]
pub enum TimerState {
    Running,
    Paused,
    Expired,
}

/// Holds data for TimerWidget.
// `last_started` is the `SystemTime` when the timer was last started/unpaused.
// `last_remaining` is the `Duration` remaining on the timer when the timer was last started/unpaused.
// The TimerWidget that owns this TimerData will display (last_remaining - (now - last_started)).
#[derive(Clone, Data)]
pub struct TimerData {
    last_started: time::SystemTime,
    last_remaining: time::Duration,
    state: TimerState,
    // TODO make private
    // TODO probably should not ignore?
    #[data(ignore)]
    pub timer_id: TimerToken,
}

impl TimerData {
    pub fn new(
        last_started: time::SystemTime,
        last_remaining: time::Duration,
        state: TimerState,
    ) -> Self {
        TimerData {
            last_started: last_started,
            last_remaining: last_remaining,
            state: state,
            timer_id: TimerToken::INVALID,
        }
    }

    pub fn to_string(&self) -> String {
        let secs_remaining = if self.state == TimerState::Paused {
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

    pub fn check_timed_out(&mut self) {
        if self.state != TimerState::Expired
            && self.last_started + self.last_remaining <= time::SystemTime::now()
        {
            self.state = TimerState::Expired;
        }
    }

    pub fn get_state(&self) -> TimerState {
        self.state
    }

    pub fn resume(&mut self) {
        if self.state == TimerState::Paused {
            self.last_started = time::SystemTime::now();
            self.state = TimerState::Running;
        }
    }

    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.last_remaining -= time::SystemTime::now()
                .duration_since(self.last_started)
                .unwrap();
            self.state = TimerState::Paused;
        }
    }
}
