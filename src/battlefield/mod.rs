use orbtk::prelude::*;

pub mod shape_widget;
pub use shape_widget::{ShapeWidget};

pub mod base_layer;
pub use base_layer::BaseLayer;


pub mod physic_world;
use physic_world::PhysicWorld;

pub mod graphic_world;
use graphic_world::GraphicWorld;

pub mod battlefield;
pub use battlefield::Battlefield;

use nphysics2d::object::BodyPartHandle;
use nphysics2d::object::DefaultBodyHandle;
use nphysics2d::object::DefaultColliderHandle;

use ncollide2d::shape::ShapeHandle;
use ncollide2d::shape::Ball;

use nphysics2d::object::ColliderDesc;
use nphysics2d::object::Collider;

use nphysics2d::math::Isometry;
use nphysics2d::math::Vector;

#[derive(Debug,Clone,PartialEq)]
pub enum PhysicalShape
{
    Ball2D(Ball<f64>)
}

impl Default for PhysicalShape
{
    fn default()->Self {Self::Ball2D(Ball::new(5.0))}
}


impl PhysicalShape
{
    pub fn to_collider(self,parent_handle: DefaultBodyHandle)->Collider<f64,DefaultBodyHandle>
    {
        match self
        {
            Self::Ball2D(ball)=>
            {
                let shape = ShapeHandle::new(ball);
                ColliderDesc::new(shape)
                .build(BodyPartHandle(parent_handle,0))
            }
        }

    }
}

into_property_source!(PhysicalShape);

#[derive(Debug,PartialEq,Clone)]
pub struct IsometryF64(Isometry<f64>);

impl Default for IsometryF64
{
    fn default()->Self
    {
        Self(Isometry::new(Vector::new(0.0,0.0),0.0))
    }
}

use std::ops::{Deref, DerefMut};

impl Deref for IsometryF64 {
    type Target = Isometry<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IsometryF64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}



#[derive(Clone,PartialEq,Debug)]
pub enum BattlefieldEvent
{
    EntityMoved(Entity,Isometry<f64>)
}

into_property_source!(BattlefieldEvent);

#[derive(Clone,PartialEq,Debug)]
pub enum LayerType
{
    MapLayer,
    BaseLayer,
    DiscoveryLayer
}


#[derive(Clone,PartialEq,Debug)]
pub struct Layer
{
    name: String,
    entity: Entity
}


#[derive(Clone,PartialEq,Debug)]
pub enum BattlefieldView
{
    Top,
    Side
}
impl Default for BattlefieldView
{
    fn default()->Self {Self::Top}
}

#[derive(Debug,Default,Clone,PartialEq)]
pub struct WorldSize(f64,f64);
into_property_source!(WorldSize);

#[derive(Debug,Default,Clone,PartialEq)]
pub struct CameraCenter(f64,f64);
into_property_source!(CameraCenter);
