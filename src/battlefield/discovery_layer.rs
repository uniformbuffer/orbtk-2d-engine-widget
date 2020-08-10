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
    DiscoveryLayer<DiscoveryLayerState>
    {

    }
);

impl Template for DiscoveryLayer {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("DiscoveryLayer")

    }
}
