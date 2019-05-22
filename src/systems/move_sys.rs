use specs::prelude::*;

use crate::components::*;
use crate::resources::*;

use ggez::mint::Point2;

type Point = Point2<f32>;

pub struct MoveSys;

impl<'a> System<'a> for MoveSys{
    type SystemData = (WriteStorage<'a, Pos>, WriteStorage<'a, Movement>, Read<'a, TimeStep>);

    fn run(&mut self, (mut pos, mut movement, time_step): Self::SystemData){
        (&mut pos, &mut movement).par_join().for_each(|(pos, movements)|{
            let d_t = time_step.0;
            movements.vel.0 += (movements.accel.0 + movements.past_accel.0)/2.0 * d_t;
            movements.vel.1 += (movements.accel.1 + movements.past_accel.1)/2.0 * d_t;
            pos.x += movements.vel.0 * d_t + movements.accel.0/2.0 * d_t.powi(2);
            pos.y += movements.vel.1 * d_t + movements.accel.1/2.0 * d_t.powi(2);

            movements.past_accel = movements.accel;
        });
    }
}
