use orbtk::prelude::*;
use crate::battlefield::BattlefieldEvent;

enum BaseLayerAction
{
    NewEvents
}

#[derive(Default, AsAny)]
pub struct BaseLayerState {
    actions: Vec<BaseLayerAction>,
}

impl BaseLayerState
{
    pub fn action(&mut self,action: BaseLayerAction)
    {
        self.actions.push(action);
    }
    pub fn handle_event(&mut self,event: &BattlefieldEvent)
    {
    }
}

impl State for BaseLayerState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context)
    {
        ctx.widget().set::<String16>("name",String16::from("base_layer"));
    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context)
    {
        let actions: Vec<BaseLayerAction> = self.actions.drain(..).collect();
        for action in actions
        {
            match action
            {
                BaseLayerAction::NewEvents=>
                {
                    let current_widget = ctx.widget();
                    let events = current_widget.get::<Vec<BattlefieldEvent>>("battlefield_events");
                    for event in events
                    {
                        self.handle_event(event);
                    }
                }
                _=>{}
            }
        }
    }
}

type BattlefieldEvents = Vec<BattlefieldEvent>;

widget!(
    /**
    Base layer is supposed to be a base layer where entity are added.
    The idea is that there could be multiple base_layer that store (and display) specific kind of object, so that hiding the layer will hide all the object of that kind.
    For example is possible to use a base layer for display obstacles and a base layer for display characters.
    */
    BaseLayer<BaseLayerState>
    {
        battlefield_events: BattlefieldEvents
    }
);

impl Template for BaseLayer {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let current_widget = ctx.get_widget(id);
        self.name("BaseLayer")
        .on_changed(|states, entity, property_name| {
            match property_name
            {
                "battlefield_events"=>
                {
                    states.get_mut::<BaseLayerState>(entity).action(BaseLayerAction::NewEvents);
                }
                _=>{}
            }
        })
    }
}
