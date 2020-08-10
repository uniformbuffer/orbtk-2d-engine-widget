use orbtk::{prelude::*, render::concurrent::RenderContext2D};

use super::PhysicalShape;
use super::IsometryF64;

#[derive(Default,AsAny)]
pub struct ShapeWidgetState {

}

impl ShapeWidgetState
{

}

impl State for ShapeWidgetState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context)
    {

    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context)
    {

    }
}

widget!(
    ShapeWidget<ShapeWidgetState>
    {
        physical_shape: PhysicalShape,
        physical_position: IsometryF64,
        background: Brush
    }
);

impl Template for ShapeWidget {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ShapeWidget")

    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(ShapeWidgetRenderObject)
    }
}

use std::f64::consts::PI;

pub struct ShapeWidgetRenderObject;

impl Into<Box<dyn RenderObject>> for ShapeWidgetRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

fn render_circle(
    render_context_2_d: &mut RenderContext2D,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radius: f64,
) {
    render_context_2_d.arc(x + width / 2.0, y + height / 2.0, radius, 0., 2. * PI);
    render_context_2_d.close_path();
}
impl RenderObject for ShapeWidgetRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
        let background = ctx.widget().clone::<Brush>("background");
        let bounds = ctx.widget().clone::<Rectangle>("bounds");

        match ctx.widget().clone::<PhysicalShape>("physical_shape")
        {
            PhysicalShape::Ball2D(ball)=>
            {
                let radius = ball.radius();
                render_circle(
                    ctx.render_context_2_d(),
                    global_position.x() + bounds.x(),
                    global_position.y() + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    radius,
                );
                ctx.render_context_2_d().set_fill_style(background);
                ctx.render_context_2_d().fill();
            }
            _=>panic!("Not implemented")
        }
    }
}
