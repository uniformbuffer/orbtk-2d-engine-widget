use orbtk::{prelude::*, render::platform::RenderContext2D};
use orbtk::behaviors::MouseBehavior;

#[derive(Default, AsAny)]
pub struct DiscoveryLayerState {

}

impl DiscoveryLayerState
{

}

impl State for DiscoveryLayerState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context)
    {

    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context)
    {

    }
}

widget!(
    /**
    This layer is supposed to be a layer that will turn the entire map black and,
    based on the movement of controlled entity, it will make that paths visible.
    */
    DiscoveryLayer<DiscoveryLayerState>
    {

    }
);

impl Template for DiscoveryLayer {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("DiscoveryLayer")

    }
}
