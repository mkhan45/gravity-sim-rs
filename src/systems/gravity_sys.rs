use specs::prelude::*;

use crate::components::*;

const MULTIPLIER: f32 = 25.0;

pub struct GraviSys;

impl<'a> System<'a> for GraviSys{
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Vel>, ReadStorage<'a, Mass>);

    fn run(&mut self, (pos, mut vel, mass): Self::SystemData){
        for (current_pos, current_vel, current_mass) in (&pos, &mut vel, &mass).join(){
            for (other_pos, other_mass) in (&pos, &mass).join(){
                let distance = distance(current_pos, other_pos);

                if distance > 0.0001{
                    let x_comp = other_pos.x - current_pos.x;
                    let y_comp = other_pos.y - current_pos.y;

                    let magnitude = ((current_mass.0 * other_mass.0) / distance.powi(2)) * MULTIPLIER;
                    let x_mag = x_comp/distance * magnitude;
                    let y_mag = y_comp/distance * magnitude;

                    current_vel.x += x_mag;
                    current_vel.y += y_mag;
                }
            }
        }
    }
}

fn distance(a: &Pos, b: &Pos) -> f32{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
