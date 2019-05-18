use specs::prelude::*;

use crate::components::*;

pub struct MoveSys;

use crate::components::*;

impl<'a> System<'a> for MoveSys{
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData){
        (&mut pos, &vel).par_join().for_each(|(pos, vel)|{
            pos.x += vel.x;
            pos.y += vel.y;
        });
    }
}
