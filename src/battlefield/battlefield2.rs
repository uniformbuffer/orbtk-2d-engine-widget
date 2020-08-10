

extern crate nalgebra as na;

pub use na::Vector2;
pub use ncollide2d::math::Isometry;

use ncollide2d::pipeline::{ContactEvent,ProximityEvent};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet,RigidBodyDesc,DefaultBodyHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

use orbtk::prelude::*;

use crate::battlefield::PhysicalShape;
use crate::battlefield::IsometryF64;
use crate::battlefield::PhysicalEntity;
use crate::battlefield::BattlefieldEvent;
use crate::battlefield::Layer;
use std::ops::Deref;

enum BattlefieldAction
{
    AddEntity(String,Entity,Isometry<f64>),
    RemoveEntity(Entity),
    MoveEntity(Entity,Isometry<f64>),
    MoveOffsetEntity(Entity,Point),

    LayerStatusChanged(String),
    MoveCamera(f64,f64)
}

#[derive(AsAny)]
pub struct BattlefieldState {
    actions: Vec<BattlefieldAction>,

    //Origin 0,0
    world_width: f64,
    world_height: f64,

    camera_center: (f64,f64),

    layers: Vec<Layer>,

    physical_entities: HashMap<Entity,PhysicalEntity>,

    mechanical_world: DefaultMechanicalWorld<f64>,
    geometrical_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    collider_set: DefaultColliderSet<f64>,
    constraint_set: DefaultJointConstraintSet<f64>,
    force_generator_set: DefaultForceGeneratorSet<f64>
}

impl BattlefieldState
{
    pub fn action(&mut self,action: BattlefieldAction) {self.actions.push(action);}
    pub fn layer_status_changed(&mut self,property_name: String){}


    pub fn world_to_camera_position(&self,world_position: Isometry<f64>)
    {
        let bounds = ctx.widget().clone::<Rectangle>("bounds");
        let camera_size = (bounds.width,bounds.height);

    }
}

impl Default for BattlefieldState
{
    fn default()->Self
    {
        Self
        {
            actions: Vec::new(),
            world_width: 100.0,
            world_height: 100.0,

            layers: Vec::new(),
            physical_entities: HashMap::new(),

            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            geometrical_world: DefaultGeometricalWorld::new(),
            body_set: DefaultBodySet::new(),
            collider_set: DefaultColliderSet::new(),
            constraint_set: DefaultJointConstraintSet::new(),
            force_generator_set: DefaultForceGeneratorSet::new()
        }
    }
}

impl State for BattlefieldState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context)
    {
        println!("Forcing first redraw");
        ctx.widget().update_dirty(true);

        if let Some(entity_id) = ctx.widget().try_clone::<u32>("base_layer") {ctx.append_child_entity_to(Entity(entity_id),ctx.entity);}
    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context)
    {
        let mut update_world = false;

        let mut physical_events = Vec::new();

        let actions: Vec<BattlefieldAction> = self.actions.drain(..).collect();
        for action in actions
        {
            match action
            {
                BattlefieldAction::AddEntity(layer_name,entity,position)=>
                {
                    println!("Adding entity");
                    let widget = ctx.get_widget(entity);
                    let physical_shape = match widget.try_clone::<PhysicalShape>("physical_shape")
                    {
                        Some(physical_shape)=>
                        {
                            println!("Entity have physical_shape");
                            let rigid_body = RigidBodyDesc::new().set_position(position).build();
                            let rigid_body_handle = self.body_set.insert(rigid_body);
                            let collider = physical_shape.to_collider(rigid_body_handle.clone());
                            let collider_handle = self.collider_set.insert(collider);

                            let physical_entity = PhysicalEntity{entity: entity,collider: collider_handle,rigid_body: rigid_body_handle};
                            self.physical_entities.insert(entity,physical_entity);
                        }
                        None=>println!("Entity does not have physical_shape")
                    };

                    println!("Adding entity to layer {}",&layer_name);
                    let layer = ctx.widget().clone::<u32>(&layer_name);
                    ctx.append_child_entity_to(entity,Entity(layer));
                    update_world = true;
                }
                BattlefieldAction::MoveEntity(entity,position)=>
                {
                    match self.physical_entities.get(&entity)
                    {
                        Some(physical_entity)=>
                        {
                            let rigid_body = self.body_set.rigid_body_mut(physical_entity.rigid_body).unwrap();
                            rigid_body.set_position(position);
                        }
                        None=>()
                    }
/*
                    if let Some(physical_handle) = ctx.get_widget(entity).try_get::<CollisionObjectSlabHandle>("physical_handle")
                    {
                        if let Some(object) = self.collision_world.get_mut(*physical_handle)
                        {
                            object.set_position(position);
                        }
                    }
                    */
                    update_world = true;
                }
                BattlefieldAction::MoveCamera(x,y)=>
                {

                }
                _=>{}
            }

            if update_world
            {
                self.mechanical_world.step(
                    &mut self.geometrical_world,
                    &mut self.body_set,
                    &mut self.collider_set,
                    &mut self.constraint_set,
                    &mut self.force_generator_set,
                );

                // Get proximity events
                for event in self.geometrical_world.proximity_events() {
                    handle_proximity_event(&self.mechanical_world, event)
                }
                // Get contact events
                for event in self.geometrical_world.contact_events() {
                    handle_contact_event(&self.mechanical_world, event)
                }

                // Get physical events
                for physical_entity in self.physical_entities.values()
                {
                    let current_isometry = ctx.get_widget(physical_entity.entity).clone::<IsometryF64>("physical_position");
                    let new_isometry = self.body_set.rigid_body(physical_entity.rigid_body).unwrap().position();

                    if current_isometry.deref() != new_isometry {physical_events.push(BattlefieldEvent::EntityMoved(physical_entity.entity,*new_isometry))}
                }

                update_world = false;
            }
        }
    }
}

widget!(
    Battlefield<BattlefieldState>
    {
        map_layer: u32,
        base_layer: u32,
        discovery_layer: u32
    }
);

impl Battlefield
{
    pub fn add_entity(mut self,layer_name: &str,entity: Entity,position: Isometry<f64>)->Self
    {
        self.state.action(BattlefieldAction::AddEntity(layer_name.to_string(),entity,position));
        self
    }

}

impl Template for Battlefield {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Battlefield")
        .on_changed(|states, entity, property_name| {
            states.get_mut::<BattlefieldState>(entity)
            .layer_status_changed(property_name.to_string());

            println!("Property {} changed on NumericBox!",property_name);
        })
    }
}

