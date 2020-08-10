use orbtk::prelude::*;
pub use ncollide2d::math::Isometry;
use super::{PhysicWorld,GraphicWorld,WorldSize,CameraCenter};

#[derive(PartialEq,Clone)]
enum BattlefieldAction
{
    AddEntity(Entity,String),
    RemoveEntity(Entity),

    MoveCamera(f64,f64),            //Implemented

    AddLayer(Entity),        //Implemented
    RemoveLayerById(String),        //Implemented
    RemoveLayerByEntity(Entity),    //Implemented
}

#[derive(Default, AsAny)]
pub struct BattlefieldState {
    actions: Vec<BattlefieldAction>,

    graphic_world: Entity,
    physic_world: Entity
}

impl BattlefieldState
{
    pub fn add_entity(&mut self, entity: Entity, layer: String, position: Isometry<f64>) {self.actions.push(BattlefieldAction::AddEntity(entity,layer,position));}

    pub fn move_camera(&mut self, position: (f64,f64)) {self.actions.push(BattlefieldAction::MoveCamera(position.0,position.1));}

    pub fn add_layer(&mut self, layer: Entity) {self.actions.push(BattlefieldAction::AddLayer(layer));}
    pub fn remove_layer_by_id(&mut self, id: String) {self.actions.push(BattlefieldAction::RemoveLayerById(id));}
    pub fn remove_layer_by_entity(&mut self, entity: Entity) {self.actions.push(BattlefieldAction::RemoveLayerByEntity(entity));}



    pub fn process_actions(&mut self,registry: &mut Registry, ctx: &mut Context)
    {
        let actions: Vec<BattlefieldAction> = self.actions.drain(..).collect();
        for action in actions
        {
            match action
            {
                BattlefieldAction::AddEntity(entity,layer_name)=>
                {
                    if Some(_) = ctx.get_widget(entity).try_clone::<Isometry64>("physical_position") {}
                    else
                    {
                        ctx.get_widget(entity).set("physical_position",Isometry::new(Vector::new(0.0,0.0),0.0));
                    }
                    if let Some(layer) = ctx.entity_of_child(layer_name.as_str())
                    {
                        ctx.append_child_entity_to(entity,layer);
                    }

                    if ctx.get_widget(entity).has::<PhysicalShape>("physical_shape")
                    {
                        ctx.append_child_entity_to(entity,self.physic_world);
                    }


                    //TODO Add entity to physic engine if it have a physical shape and a physical position
                    println!("Added entity to layer {}",&layer_name);
                }
                BattlefieldAction::RemoveEntity(entity)=>
                {
                    //TODO C'e' bisogno di rimuovere l'entita' dal layer,
                    //ma non trovo una funzione per trovare in che layer e' l'entita'
                    //o per rimuovere ricorsivamente
                }
                BattlefieldAction::MoveCamera(x,y)=>
                {
                    ctx.widget().set("camera_center",CameraCenter(x,y))
                }
                BattlefieldAction::AddLayer(layer)=>
                {
                    if let Some(id) = ctx.get_widget(layer).try_clone::<String16>("id")
                    {
                        ctx.append_child_entity_to(layer,self.graphic_world);
                    }
                    else {println!("Cannot add layer {:#?} because id is not setted",layer);}
                }
                BattlefieldAction::RemoveLayerById(id)=>
                {
                    if let Some(layer) = ctx.entity_of_child(id.as_str())
                    {
                        ctx.remove_child_from(layer,self.graphic_world);
                    }
                    else {println!("Cannot remove layer {}: id not found",id);}
                }
                BattlefieldAction::RemoveLayerByEntity(entity)=>
                {
                    ctx.remove_child_from(entity,self.graphic_world);
                }
                _=>{}
            }
        }
    }
}

impl State for BattlefieldState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context)
    {
        let mut graphic_world = GraphicWorld::new()
        .world_size(ctx.entity)
        .camera_center(ctx.entity);

        for mut i in 0..self.actions.len()
        {
            match self.actions[i]
            {
                BattlefieldAction::AddLayer(layer)=>
                {
                    println!("Added layer");
                    graphic_world = graphic_world.layer(layer);
                    self.actions.remove(i);
                    i = i-1;
                }
                _=>{}
            }
        }

        self.physic_world = PhysicWorld::new().world_size(ctx.entity).build(&mut ctx.build_context());
        self.graphic_world = graphic_world.build(&mut ctx.build_context());

        ctx.append_child_entity_to(self.physic_world,ctx.entity);
        ctx.append_child_entity_to(self.graphic_world,ctx.entity);
        println!("Worlds initialized");
    }
    fn update(&mut self, _: &mut Registry, ctx: &mut Context)
    {

    }
}

widget!(
    /**
    Battlefield is the entity which the user interact with.
    battlefield will abstract and synchronize functions called on physic_world and graphic_world.
    */
    Battlefield<BattlefieldState>
    {
        world_size: WorldSize,
        camera_center: CameraCenter
    }
);

impl Battlefield
{
    pub fn layer(mut self, entity: Entity)->Self
    {
        self.state.add_layer(entity);
        self
    }
}

impl Template for Battlefield {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Battlefield")

    }
}
