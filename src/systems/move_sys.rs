use specs::prelude::*;

use crate::components::*;

pub struct MoveSys;

use crate::components::*;

impl<'a> System<'a> for MoveSys{
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData){
        for (pos, vel) in (&mut pos, &vel).join(){
            pos.x += vel.x;
            pos.y += vel.y;
            println!("{:?}", pos);
        }
    }
}
