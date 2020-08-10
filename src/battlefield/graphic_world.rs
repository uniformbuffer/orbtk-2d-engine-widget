use orbtk::{prelude::*, render::concurrent::RenderContext2D};
use crate::battlefield::WorldSize;
use crate::battlefield::IsometryF64;
use crate::battlefield::BaseLayer;

use std::collections::HashMap;

use super::CameraCenter;

enum GraphicWorldAction
{
    AddEntity(Entity,String),
    RemoveEntity(Entity),

    MoveCamera(f64,f64),

    AddLayer(Entity),        //Implemented
    RemoveLayerById(String),        //Implemented
    RemoveLayerByEntity(Entity),    //Implemented
}

#[derive(Default, AsAny)]
pub struct GraphicWorldState {
    actions: Vec<GraphicWorldAction>,
}

impl GraphicWorldState
{
    pub fn action(&mut self,action: GraphicWorldAction) {self.actions.push(action);}

    pub fn add_entity(&mut self, entity: Entity, layer: String) {self.actions.push(GraphicWorldAction::AddEntity(entity,layer));}

    pub fn move_camera(&mut self, position: (f64,f64)) {self.actions.push(GraphicWorldAction::MoveCamera(position.0,position.1));}

    pub fn add_layer(&mut self, layer: Entity) {self.actions.push(GraphicWorldAction::AddLayer(layer));}
    pub fn remove_layer_by_id(&mut self, id: String) {self.actions.push(GraphicWorldAction::RemoveLayerById(id));}
    pub fn remove_layer_by_entity(&mut self, entity: Entity) {self.actions.push(GraphicWorldAction::RemoveLayerByEntity(entity));}


    pub fn process_actions(&mut self,registry: &mut Registry, ctx: &mut Context)
    {
        let actions: Vec<GraphicWorldAction> = self.actions.drain(..).collect();
        for action in actions
        {
            match action
            {
                GraphicWorldAction::AddEntity(entity,layer_name)=>
                {
                    println!("Adding entity to layer {}",&layer_name);
                    let layer = ctx.widget().clone::<u32>(&layer_name);
                    ctx.append_child_entity_to(entity,Entity(layer));
                }
                GraphicWorldAction::MoveCamera(x,y)=>
                {

                }
                /*
                GraphicWorldAction::Addlayer(name,layout)=>
                {
                    self.layouts.insert(name,layout.clone());
                    ctx.append_child_entity_to(layout,ctx.entity);
                }
                */
                _=>{}
            }
        }
    }
}

impl State for GraphicWorldState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context)
    {
        //let base_layer = BaseLayer::new().build(&mut ctx.build_context());

        //self.actions.push()

        self.process_actions(registry,ctx);
    }
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context)
    {
        self.process_actions(registry,ctx);
    }
}

widget!(
    /**
    GraphicWorld is used to draw entities. It manage only the graphical representation of an entity.
    */
    GraphicWorld<GraphicWorldState>
    {
        world_size: WorldSize,

        camera_center: CameraCenter
    }
);

impl GraphicWorld
{
    pub fn layer(mut self, entity: Entity)->Self
    {
        self.state.add_layer(entity);
        self
    }
}

impl Template for GraphicWorld {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("GraphicWorld")
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(CameraLayout::new())
    }
}

use std::{cell::RefCell, collections::BTreeMap};

/// Place widgets absolute on the screen.
#[derive(Default)]
pub struct CameraLayout {
    desired_size: RefCell<DirtySize>,
}

impl CameraLayout {
    pub fn new() -> Self {
        CameraLayout::default()
    }
}

impl Layout for CameraLayout {
    fn measure(
        &self,
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &Theme,
    ) -> DirtySize {
        let window = ecm.entity_store().root();

        if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", window) {
            self.desired_size
                .borrow_mut()
                .set_size(bounds.width(), bounds.height());
        }

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];
            if let Some(child_layout) = layouts.get(&child) {
                let dirty = child_layout
                    .measure(render_context_2_d, child, ecm, layouts, theme)
                    .dirty()
                    || self.desired_size.borrow().dirty();

                self.desired_size.borrow_mut().set_dirty(dirty);
            }
        }

        *self.desired_size.borrow()
    }

    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        _parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &Theme,
    ) -> (f64, f64) {
        if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return (0.0, 0.0);
        }

        if let Some(bounds) = component_try_mut::<Rectangle>(ecm, entity, "bounds") {
            bounds.set_width(self.desired_size.borrow().width());
            bounds.set_height(self.desired_size.borrow().height());
        }

        mark_as_dirty("bounds", entity, ecm);

        //This is what the camera see actually
        let camera_view =
        {
            let camera_center = component::<CameraCenter>(ecm, entity, "camera_center");
            let bounds = component::<Rectangle>(ecm, entity, "bounds");
            let top_left_corner = Point::new(camera_center.0 - (bounds.width()/2.0),camera_center.1 - (bounds.height()/2.0));
            Rectangle::new(top_left_corner,bounds.width(),bounds.height())
        };

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];
            if let Some(child_layout) = layouts.get(&child) {
                if let Some(physical_position) = try_component::<IsometryF64>(ecm, child, "physical_position")
                {
                    //If the child have a phisical position, it must be inside the camera view
                    let point = Point::new(physical_position.translation.vector.x,physical_position.translation.vector.y);
                    if camera_view.contains(point)
                    {
                        //The drawing zone start from (0,0) to (bounds.width,bounds.height), so the physical position
                        //need to be adjusted with the drawing posision.
                        //To do this, i simply translate the physical position to align with the (0,0) position.
                        let adjusted_position =
                        (
                            physical_position.translation.vector.x - camera_view.x(),
                            physical_position.translation.vector.y - camera_view.y()
                        );
                        child_layout.arrange(
                            render_context_2_d,
                            (
                                self.desired_size.borrow().width(),
                                self.desired_size.borrow().height(),
                            ),
                            child,
                            ecm,
                            layouts,
                            theme,
                        );

                        if let Some(child_bounds) = component_try_mut::<Rectangle>(ecm, child, "bounds")
                        {
                            child_bounds.set_x(adjusted_position.0);
                            child_bounds.set_y(adjusted_position.1);
                        }
                        else{println!("Warning: cannot set bounds");}
                    }
                }
                else
                {
                    //Otherwise the child is something placed on absolute position on the camera
                    child_layout.arrange(
                        render_context_2_d,
                        (
                            self.desired_size.borrow().width(),
                            self.desired_size.borrow().height(),
                        ),
                        child,
                        ecm,
                        layouts,
                        theme,
                    );
                }
            }
        }


        self.desired_size.borrow_mut().set_dirty(false);
        self.desired_size.borrow().size()
    }
}

impl Into<Box<dyn Layout>> for CameraLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}


fn component<C: Component + Clone>(
    ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
    entity: Entity,
    component: &str,
) -> C {
    ecm.component_store()
        .get::<C>(component, entity)
        .unwrap()
        .clone()
}

fn try_component<C: Component + Clone>(
    ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
    entity: Entity,
    component: &str,
) -> Option<C> {
    if let Ok(c) = ecm.component_store().get::<C>(component, entity) {
        return Some(c.clone());
    }

    None
}

fn component_or_default<C: Component + Clone + Default>(
    ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
    entity: Entity,
    component: &str,
) -> C {
    ecm.component_store()
        .get::<C>(component, entity)
        .map(Clone::clone)
        .unwrap_or_default()
}

fn component_try_mut<'a, C: Component>(
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    entity: Entity,
    component: &str,
) -> Option<&'a mut C> {
    ecm.component_store_mut()
        .get_mut::<C>(component, entity)
        .ok()
}
