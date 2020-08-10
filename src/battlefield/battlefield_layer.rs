use orbtk::{prelude::*, render::platform::RenderContext2D};


enum BattlefieldLayerAction
{
    AddEntity(Entity,Option<Point>),
    RemoveEntity(Entity),
    MoveEntity(Entity,Point),
    MoveOffsetEntity(Entity,Point)
}

#[derive(Default, AsAny)]
pub struct BattlefieldLayerState {
    actions: Vec<BattlefieldLayerAction>,
}

impl BattlefieldLayerState
{
    pub fn add_entity(&mut self,entity: impl Into<Entity>, point: Option<impl Into<Point>>)
    {
        self.actions.push(BattlefieldLayerAction::AddEntity(entity.into(),if let Some(p) = point {Some(p.into())}else{None}));
    }
    pub fn remove_entity(&mut self,entity: impl Into<Entity>)
    {
        self.actions.push(BattlefieldLayerAction::RemoveEntity(entity.into()));
    }
    pub fn move_entity(&mut self,entity: impl Into<Entity>, point: impl Into<Point>)
    {
        self.actions.push(BattlefieldLayerAction::MoveEntity(entity.into(),point.into()));
    }
    pub fn move_offset_entity(&mut self,entity: impl Into<Entity>, point: impl Into<Point>)
    {
        self.actions.push(BattlefieldLayerAction::MoveOffsetEntity(entity.into(),point.into()));
    }
}

impl State for BattlefieldLayerState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context)
    {

    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context)
    {
        let actions: Vec<BattlefieldLayerAction> = self.actions.drain(..).collect();
        for action in actions
        {
            match action
            {
                BattlefieldLayerAction::AddEntity(entity,point)=>
                {
                    if let Some(bound) = ctx.get_widget(entity).try_get_mut::<Rectangle>("bounds")
                    {
                        if let Some(p) = point {
                            bound.set_x(p.x());
                            bound.set_y(p.y());
                        }
                        else
                        {
                            bound.set_x(0.0);
                            bound.set_y(0.0);
                        }
                    }

                    ctx.append_child_entity_to(entity, ctx.entity);
                }
                BattlefieldLayerAction::RemoveEntity(entity)=>
                {
                    ctx.remove_child_from(entity, ctx.entity);
                }
                BattlefieldLayerAction::MoveEntity(entity,center)=>
                {
                    if let Some(bound) = ctx.get_widget(entity).try_get_mut::<Rectangle>("bounds")
                    {
                        let new_origin = Point::new(center.x()-(bound.width()/2.0),center.y()+(bound.height()/2.0));
                        bound.set_x(new_origin.x());
                        bound.set_y(new_origin.y());
                    }
                }
                BattlefieldLayerAction::MoveOffsetEntity(entity,point)=>
                {
                    let mut container = ctx.get_widget(entity);
                    let mut bounds = container.get_mut::<Rectangle>("bounds");
                    bounds.set_x(bounds.x()+point.x());
                    bounds.set_y(bounds.y()+point.y());
                }
            }
        }
    }
}

widget!(
    BattlefieldLayer<BattlefieldLayerState>
    {
        world_width: f64,
        world_height: f64,
    }
);

impl BattlefieldLayer
{
    pub fn add_entity(mut self,entity: impl Into<Entity>, point: Option<impl Into<Point>>)->Self
    {
        self.state.actions.push(BattlefieldLayerAction::AddEntity(entity.into(),if let Some(p) = point {Some(p.into())}else{None}));
        self
    }
    pub fn remove_entity(mut self,entity: impl Into<Entity>)->Self
    {
        self.state.actions.push(BattlefieldLayerAction::RemoveEntity(entity.into()));
        self
    }
    pub fn move_entity(mut self,entity: impl Into<Entity>, point: impl Into<Point>)->Self
    {
        self.state.actions.push(BattlefieldLayerAction::MoveEntity(entity.into(),point.into()));
        self
    }
    pub fn move_offset_entity(mut self,entity: impl Into<Entity>, point: impl Into<Point>)->Self
    {
        self.state.actions.push(BattlefieldLayerAction::MoveOffsetEntity(entity.into(),point.into()));
        self
    }
}

impl Template for BattlefieldLayer {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("BattlefieldLayer")

    }
    fn layout(&self) -> Box<dyn Layout> {
        Box::new(AbsoluteLayout::new())
    }
}
