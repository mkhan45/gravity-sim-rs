use specs::prelude::*;

use crate::components::*;

pub struct MoveSys;

use crate::components::*;

impl<'a> System<'a> for MoveSys{
    type SystemData = (WriteStorage<'a, Pos>, WriteStorage<'a, Movement>);

    fn run(&mut self, (mut pos, mut movement): Self::SystemData){
        (&mut pos, &mut movement).par_join().for_each(|(pos, movements)|{

            pos.x += movements.vel.0 + movements.accel.0/2.0;
            pos.y += movements.vel.1 + movements.accel.1/2.0;
            movements.vel.0 += (movements.accel.0 + movements.past_accel.0)/2.0;
            movements.vel.1 += (movements.accel.1 + movements.past_accel.1)/2.0;

            movements.past_accel = movements.accel;
        });
    }
}
