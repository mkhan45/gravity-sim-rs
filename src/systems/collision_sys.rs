use specs::prelude::*;

use crate::components::*;

pub struct CollisionSys;

impl<'a> System<'a> for CollisionSys{
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Vel>, ReadStorage<'a, Mass>);

    fn run(&mut self, (pos, mut vel, mass): Self::SystemData){
    }
}

fn distance(a: &Pos, b: &Pos) -> f32{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
