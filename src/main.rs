use druid::{
    widget::{prelude::*, CrossAxisAlignment, Flex, Label, MainAxisAlignment, WidgetExt},
    TimerToken,
};
use druid::{AppLauncher, Color, Data, LocalizedString, WindowDesc};
use std::time;

static TIMER_INTERVAL: time::Duration = time::Duration::from_millis(200);

/// Holds data for TimerWidget.
// `last_started` is the `SystemTime` when the timer was last started/unpaused.
// `last_remaining` is the `Duration` remaining on the timer when the timer was last started/unpaused.
// The TimerWidget that owns this TimerData will display (last_remaining - (now - last_started)).
#[derive(Clone, Data)]
struct TimerData {
    last_started: time::SystemTime,
    last_remaining: time::Duration,
    running: bool,
}

impl TimerData {
    fn to_string(&self) -> String {
        let secs_remaining = if !self.running {
            self.last_remaining.as_secs()
        } else {
            self.last_remaining.checked_sub(
                time::SystemTime::now().duration_since(self.last_started).unwrap()
            ).unwrap_or(time::Duration::new(0, 0)).as_secs()
        };

        format!("{:02}:{:02}", secs_remaining / 60, secs_remaining % 60)
    }

    fn timed_out(&self) -> bool {
        self.last_started + self.last_remaining <= time::SystemTime::now()
    }

    #[allow(dead_code)]
    fn resume(&mut self) {
        if !self.running {
            self.last_started = time::SystemTime::now();
            self.running = true;
        }
    }

    #[allow(dead_code)]
    fn pause(&mut self) {
        if self.running {
            self.last_remaining -= time::SystemTime::now()
                .duration_since(self.last_started)
                .unwrap();
            self.running = false
        }
    }
}

struct TimerWidget {
    timer_id: TimerToken,
    label: Label<TimerData>,
}

impl TimerWidget {
    fn new() -> Self {
        TimerWidget {
            timer_id: TimerToken::INVALID,
            label: Label::new(|t_data: &TimerData, _: &Env| t_data.to_string())
                .with_text_color(Color::WHITE),
        }
    }
}

impl Widget<TimerData> for TimerWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut TimerData, env: &Env) {
        match event {
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) => {
                if *id == self.timer_id && !data.timed_out() {
                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
                ctx.request_update();
                println!("TIMER EVENT {}", data.to_string());
            }
            _ => {}
        }

        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &TimerData, env: &Env,) {
        self.label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &TimerData, data: &TimerData, env: &Env) {
        self.label.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &TimerData, env: &Env,) -> Size {
        self.label.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &TimerData, env: &Env) {
        self.label.paint(ctx, data, env);
    }
}

// use UI builder 
fn build_ui() -> impl Widget<TimerData> {
    let timer_widget = TimerWidget::new()
        .background(Color::BLACK)
        .fix_size(50., 50.);

    Flex::column()
        .with_flex_spacer(100.0)
        .with_child(timer_widget)
        .with_flex_spacer(100.0)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .background(Color::WHITE)
}

pub fn main() {
    let window = WindowDesc::new(build_ui)
        .title(LocalizedString::new("Druid Timer"));
    let initial_state = TimerData {
        last_started: time::SystemTime::now(),
        last_remaining: time::Duration::from_secs(60),
        running: true,
    };

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("launch failed");
}
