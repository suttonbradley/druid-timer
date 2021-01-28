mod data;
mod start_button;

use crate::data::{TimerData, TimerState};
use druid::widget::{
    prelude::*, Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, WidgetExt,
};
use druid::{AppLauncher, Color, LocalizedString, WindowDesc};
use std::time;

static TIMER_INTERVAL: time::Duration = time::Duration::from_millis(200);

struct TimerWidget {
    label: Label<TimerData>,
}

impl TimerWidget {
    fn new() -> Self {
        TimerWidget {
            label: Label::new(|t_data: &TimerData, _: &Env| t_data.to_string())
                .with_text_color(Color::WHITE),
        }
    }
}

impl Widget<TimerData> for TimerWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut TimerData, env: &Env) {
        match event {
            Event::WindowConnected => {
                data.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) => {
                if *id == data.timer_id {
                    // check for timer expiration
                    data.check_timed_out();

                    match data.get_state() {
                        // request new timer if running
                        TimerState::Running => {
                            data.timer_id = ctx.request_timer(TIMER_INTERVAL);
                        }
                        // else depend on button click to request new timer (for efficiency)
                        _ => {
                            data.timer_id = ctx.request_timer(TIMER_INTERVAL);
                        }
                    }

                    ctx.request_update();
                    println!("TIMER EVENT {}", data.to_string());
                }
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
        // println!("UPDATE<TimerWidget>\nold_data: \"{:#?}\"\ndata: \"{:#?}\"\n", old_data, data);
        self.label.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &TimerData,
        env: &Env,
    ) -> Size {
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

    let start_button =
        Button::new("Start").on_click(|_ctx: &mut EventCtx, t_data: &mut TimerData, _| match t_data
            .get_state()
        {
            TimerState::Running => {
                t_data.pause();
                println!("BUTTON: Running -> Paused");
            }
            TimerState::Paused => {
                t_data.resume();
                // t_data.timer_id = ctx.request_timer(TIMER_INTERVAL);
                println!("BUTTON: Paused -> Running");
            }
            _ => {}
        });

    Flex::column()
        .with_flex_spacer(100.0)
        .with_child(timer_widget)
        .with_child(start_button)
        .with_flex_spacer(100.0)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .background(Color::WHITE)
}

pub fn main() {
    let window = WindowDesc::new(build_ui).title(LocalizedString::new("Druid Timer"));
    let initial_state = TimerData::new(
        time::SystemTime::now(),
        time::Duration::from_secs(60),
        TimerState::Paused,
    );

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("launch failed");
}
