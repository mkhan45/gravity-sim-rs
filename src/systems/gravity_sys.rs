use specs::prelude::*;

use crate::components::*;

use std::f32::consts::PI;

const MULTIPLIER: f32 = 50.0;

pub struct GraviSys;

impl<'a> System<'a> for GraviSys{
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Movement>, ReadStorage<'a, Mass>);

    fn run(&mut self, (pos, mut movement, mass): Self::SystemData){
        (&pos, &mut movement).par_join().for_each(|(current_pos, current_movement)|{
            current_movement.accel = (0.0, 0.0);
            for (other_pos, other_mass) in (&pos, &mass).join(){
                let distance = distance(current_pos, other_pos);

                if  distance != 0.0{ //also makes sure not the same body
                    let x_comp = other_pos.x - current_pos.x;
                    let y_comp = other_pos.y - current_pos.y;

                    let magnitude = (other_mass.0 / distance.powi(2)) * MULTIPLIER * 1.0;
                    let x_mag = x_comp/distance * magnitude;
                    let y_mag = y_comp/distance * magnitude;

                    current_movement.accel.0 += x_mag;
                    current_movement.accel.1 += y_mag;
                }
            }
        });
    }
}

fn distance(a: &Pos, b: &Pos) -> f32{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
