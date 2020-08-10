extern crate nalgebra as na;

pub use na::Vector2;
pub use ncollide2d::math::Isometry;

use ncollide2d::pipeline::{ContactEvent,ProximityEvent};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet,RigidBodyDesc,DefaultBodyHandle,DefaultColliderHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

use orbtk::prelude::*;

use crate::battlefield::PhysicalShape;
use crate::battlefield::IsometryF64;
use crate::battlefield::BattlefieldEvent;
use crate::battlefield::Layer;
use crate::battlefield::WorldSize;
use std::ops::Deref;


#[derive(Clone)]
struct PhysicalEntity
{
    entity: Entity,
    collider: DefaultColliderHandle,
    rigid_body: DefaultBodyHandle
}

enum PhysicWorldAction
{
    AddEntity(String,Entity,Isometry<f64>),
    RemoveEntity(Entity),
    MoveEntity(Entity,Isometry<f64>),
    MoveOffsetEntity(Entity,Point)
}

#[derive(AsAny)]
pub struct PhysicWorldState {
    actions: Vec<PhysicWorldAction>,

    world_size: WorldSize,

    physical_entities: HashMap<Entity,PhysicalEntity>,

    mechanical_world: DefaultMechanicalWorld<f64>,
    geometrical_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    collider_set: DefaultColliderSet<f64>,
    constraint_set: DefaultJointConstraintSet<f64>,
    force_generator_set: DefaultForceGeneratorSet<f64>
}

impl PhysicWorldState
{
    pub fn action(&mut self,action: PhysicWorldAction) {self.actions.push(action);}
}

impl Default for PhysicWorldState
{
    fn default()->Self
    {
        Self
        {
            actions: Vec::new(),
            world_size: WorldSize(200.0,200.0),

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

impl State for PhysicWorldState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context)
    {

    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context)
    {
        let mut update_world = false;

        //let mut physical_events = Vec::new();

        let actions: Vec<PhysicWorldAction> = self.actions.drain(..).collect();
        for action in actions
        {
            match action
            {
                PhysicWorldAction::AddEntity(layer_name,entity,position)=>
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
                    update_world = true;
                }
                PhysicWorldAction::MoveEntity(entity,position)=>
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
                    update_world = true;
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
                    //let current_isometry = ctx.get_widget(physical_entity.entity).clone::<IsometryF64>("physical_position");
                    let new_isometry = self.body_set.rigid_body(physical_entity.rigid_body).unwrap().position().clone();
                    ctx.get_widget(physical_entity.entity).set("physical_position",new_isometry);
                }

                update_world = false;
            }
        }
    }
}


widget!(
    PhysicWorld<PhysicWorldState>
    {
        world_size: WorldSize
    }
);

impl Template for PhysicWorld {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("PhysicWorld")
        .world_size(WorldSize(200.0,200.0))
    }
}



fn handle_proximity_event(
    world: &DefaultMechanicalWorld<f64>,
    event: &ProximityEvent<DefaultBodyHandle>,
) {
    println!("Proximity detected");
    /*
    // The collision object with a None velocity is the coloured area.
    let area_name;
    let co1 = world.collision_object(event.collider1).unwrap();
    let co2 = world.collision_object(event.collider2).unwrap();

    if co1.data().velocity.is_none() {
        area_name = co1.data().name;
    } else {
        area_name = co2.data().name;
    }

    if event.new_status == Proximity::Intersecting {
        println!("The ball enters the {} area.", area_name);
    } else if event.new_status == Proximity::Disjoint {
        println!("The ball leaves the {} area.", area_name);
    }
    */
}

fn handle_contact_event(
    world: &DefaultMechanicalWorld<f64>,
    event: &ContactEvent<DefaultBodyHandle>)
{
    println!("Contact detected");
    if let &ContactEvent::Started(collider1, collider2) = event {
        /*
        // NOTE: real-life applications would avoid this systematic allocation.
        let pair = world.contact_pair(collider1, collider2).unwrap();
        let mut collector = Vec::new();
        pair.contacts(&mut collector);

        let co1 = world.collision_object(collider1).unwrap();
        let co2 = world.collision_object(collider2).unwrap();

        // The ball is the one with a non-None velocity.
        if let Some(ref vel) = co1.data().velocity {
            let normal = collector[0].deepest_contact().unwrap().contact.normal;
            vel.set(vel.get() - 2.0 * na::dot(&vel.get(), &normal) * *normal);
        }
        if let Some(ref vel) = co2.data().velocity {
            let normal = -collector[0].deepest_contact().unwrap().contact.normal;
            vel.set(vel.get() - 2.0 * na::dot(&vel.get(), &normal) * *normal);
        }
        */
    }
}
