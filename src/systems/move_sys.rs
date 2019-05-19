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
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Trail>, Entities<'a>, ReadStorage<'a, PreviewFlag>);

    fn run(&mut self, (pos, mut trails, entities, preview_flags): Self::SystemData){
        (&pos, &mut trails, &entities).par_join().for_each(|(pos, trail, ent)|{
            trail.points.push(Point::from_slice(&[pos.x, pos.y]));

            let preview_flag = preview_flags.get(ent);

            if trail.points.len() > trail.length as usize{
                if let None = preview_flag{
                    trail.points.remove(0);
                }
            }
        });
    }
}
