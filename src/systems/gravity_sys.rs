use specs::prelude::*;

use crate::components::*;

const MULTIPLIER: f32 = 5.0;
const CHARGE_MULTIPLIER: f32 = 5.0;

pub struct GraviSys;

impl<'a> System<'a> for GraviSys{
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Movement>, ReadStorage<'a, Mass>, ReadStorage<'a, PreviewFlag>);

    fn run(&mut self, (pos, mut movement, mass, flags): Self::SystemData){
        (&pos, &mut movement).par_join().for_each(|(current_pos, current_movement)|{
            current_movement.accel = (0.0, 0.0);
            for (other_pos, other_mass, ()) in (&pos, &mass, !&flags).join(){
                let distance = distance(current_pos, other_pos);

                if  distance != 0.0{ //also makes sure not the same body
                    let x_comp = other_pos.x - current_pos.x;
                    let y_comp = other_pos.y - current_pos.y;

                    let magnitude = (other_mass.0 / distance.powi(2)) * MULTIPLIER;
                    let x_mag = x_comp/distance * magnitude;
                    let y_mag = y_comp/distance * magnitude;

                    current_movement.accel.0 += x_mag;
                    current_movement.accel.1 += y_mag;
                }
            }
        });
    }
}

pub struct ChargeSys;

impl<'a> System<'a> for ChargeSys{
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Movement>, ReadStorage<'a, Mass>, ReadStorage<'a, Charge>, ReadStorage<'a, PreviewFlag>);

    fn run(&mut self, (positions, mut movements, masses, charges, flags): Self::SystemData){
        (&positions, &mut movements, &charges).par_join().for_each(|(current_pos, current_movement, current_charge)|{
            for (other_pos, other_charge, ()) in (&positions, &charges, !&flags).join(){
                let distance = distance(current_pos, other_pos);

                if distance != 0.0{
                    let x_comp = other_pos.x - current_pos.x;
                    let y_comp = other_pos.x - current_pos.x;

                    let magnitude = (other_charge.0 * current_charge.0) / distance.powi(2) * CHARGE_MULTIPLIER;
                    let x_mag = x_comp/distance * magnitude;
                    let y_mag = y_comp/distance * magnitude;

                    current_movement.accel.0 += x_mag;
                    current_movement.accel.1 += y_mag;
                }
            }
        })
    }
}

fn distance(a: &Pos, b: &Pos) -> f32{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
