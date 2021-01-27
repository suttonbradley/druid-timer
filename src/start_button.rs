use crate::data::TimerData;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};

pub struct StartButton {}

impl StartButton {
    pub fn new() -> Self {
        StartButton {}
    }
}

impl Widget<TimerData> for StartButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut TimerData, env: &Env) {}

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &TimerData,
        env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &TimerData, data: &TimerData, env: &Env) {}

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &TimerData,
        env: &Env,
    ) -> Size {
        bc.constrain(Size::new(50., 50.))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &TimerData, env: &Env) {}
}
