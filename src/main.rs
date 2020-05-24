use druid::kurbo::BezPath;
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, LocalizedString, Point, WindowDesc, Vec2};
use druid::kurbo::Arc;
use std::f64::consts::FRAC_PI_2;

struct CustomWidget;

impl Widget<String> for CustomWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut String, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &String,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        let size = ctx.size();
        let half_size = size * 0.5;
        let center = Point::new(half_size.width, half_size.height);

        let arc = Arc {
            center,
            radii: Vec2::new(100.0, 100.0),
            start_angle: FRAC_PI_2,
            sweep_angle: FRAC_PI_2,
            x_rotation: 0.0,
        };

        let path = BezPath::from_vec(arc.append_iter(1e-12).collect());
        let color = Color::WHITE;
        ctx.stroke(path, &color, 6.0);
    }
}

pub fn main() {
    let window = WindowDesc::new(|| CustomWidget {}).title(
        LocalizedString::new("custom-widget-demo-window-title")
            .with_placeholder("Test Arc"),
    );
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch("Druid + Piet".to_string())
        .expect("launch failed");
}
