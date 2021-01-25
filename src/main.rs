use druid::{
    widget::{prelude::*, CrossAxisAlignment, Flex, Label, MainAxisAlignment},
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
            // subtract time since last start from last_remaining
            (self.last_remaining
                - (time::SystemTime::now()
                    .duration_since(self.last_started)
                    .unwrap()))
            .as_secs()
        };

        format!("{:02}:{:02}", secs_remaining / 60, secs_remaining % 60)
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
            self.last_remaining = time::SystemTime::now()
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
            // Closure updates label when t_data changes
            label: Label::dynamic(|t_data: &TimerData, _| t_data.to_string())
                .with_text_color(Color::WHITE),
        }
    }
}

impl Widget<TimerData> for TimerWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut TimerData, env: &Env) {
        match event {
            Event::WindowConnected => {
                // Start the timer when the application launches
                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) => {
                // If this is the event for the current timer, set up a new timer
                if *id == self.timer_id {
                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
                println!("TIMER EVENT");
                println!(
                    "{} | {}",
                    data.last_remaining.as_secs(),
                    time::SystemTime::now()
                        .duration_since(data.last_started)
                        .unwrap()
                        .as_secs()
                );
            }
            _ => {}
        }

        self.label.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &TimerData,
        env: &Env,
    ) {
        self.label.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &TimerData, data: &TimerData, env: &Env) {
        self.label.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &TimerData,
        env: &Env,
    ) -> Size {
        let size = Size::new(50.0, 50.0);
        self.label.layout(ctx, bc, data, env);
        bc.constrain(size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &TimerData, env: &Env) {
        let size = Size::new(ctx.size().height / 2.0, ctx.size().width / 2.0);
        let rect = size.to_rounded_rect(10.0);
        ctx.fill(rect, &Color::BLACK);

        self.label.paint(ctx, data, env);
    }
}

struct RootWidget {
    children: Flex<TimerData>,
}

impl RootWidget {
    fn new() -> RootWidget {
        RootWidget {
            children: Flex::column()
                .with_flex_spacer(100.0)
                .with_child(TimerWidget::new())
                .with_flex_spacer(100.0)
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .main_axis_alignment(MainAxisAlignment::Center),
        }
    }
}

impl Widget<TimerData> for RootWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut TimerData, env: &Env) {
        self.children.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &TimerData,
        env: &Env,
    ) {
        self.children.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &TimerData, data: &TimerData, env: &Env) {
        self.children.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &TimerData,
        env: &Env,
    ) -> Size {
        let raw_size = Size::new(100.0, 100.0);
        let size = bc.constrain(raw_size);

        self.children.layout(ctx, bc, data, env);

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &TimerData, env: &Env) {
        // Fill window with white rectangle
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::BLACK);

        self.children.paint(ctx, data, env);
    }
}

pub fn main() {
    // Create WindowDesc with T as RootWidget's type (T must implement Data)
    let window = WindowDesc::new(|| RootWidget::new()).title(LocalizedString::new("Druid Timer"));

    // Create AppLauncher using window. AppLauncher type T is same as window's type, which is RootWidget's type
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(TimerData {
            last_started: time::SystemTime::now(),
            last_remaining: time::Duration::from_secs(62),
            running: false,
        })
        .expect("launch failed");
}
