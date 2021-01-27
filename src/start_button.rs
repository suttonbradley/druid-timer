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
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut TimerData, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &TimerData,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &TimerData,
        _data: &TimerData,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &TimerData,
        _env: &Env,
    ) -> Size {
        bc.constrain(Size::new(50., 50.))
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &TimerData, _env: &Env) {}
}
