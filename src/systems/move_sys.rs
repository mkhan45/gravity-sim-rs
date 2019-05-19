use specs::prelude::*;

use crate::components::*;

use ggez::mint::Point2;

type Point = Point2<f32>;

pub struct MoveSys;

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

pub struct TrailSys;

impl <'a> System<'a> for TrailSys{
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Trail>);

    fn run(&mut self, (pos, mut trails): Self::SystemData){
        (&pos, &mut trails).par_join().for_each(|(pos, trail)|{
            trail.points.push(Point::from_slice(&[pos.x, pos.y]));

            if trail.points.len() > trail.length as usize{
                trail.points.remove(0);
            }
        });
    }
}
