pub struct PreviewSpeedSys;

use specs::prelude::*;
use crate::resources::*;
use crate::components::*;

use ggez::mint::Point2;
type Point = Point2<f32>;

const MULTIPLIER: f32 = 5.0;

impl <'a> System<'a> for PreviewSpeedSys{
    type SystemData = (WriteStorage<'a, Pos>, WriteStorage<'a, Movement>, ReadStorage<'a, Mass>, 
                       ReadStorage<'a, PreviewFlag>, WriteStorage<'a, Trail>, Read<'a, SimSpeed>,
                       Read<'a, PredictionSpeed>, Read<'a, TimeStep>, Entities<'a>,
                       ReadStorage<'a, Radius>);

    fn run(&mut self, (mut pos, mut movement, mass, flags, mut trails, sim_speed, prediction_speed, time_step, entities, radii): Self::SystemData){
        for _i in sim_speed.0..prediction_speed.0{
            (&pos, &mut movement, &flags).join().for_each(|(current_pos, current_movement, _flag)|{
                current_movement.accel = (0.0, 0.0);
                for (other_pos, other_mass, ()) in (&pos, &mass, !&flags).join(){
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

            //movement
            (&mut pos, &mut movement, &mut trails, &flags).join().for_each(|(current_pos, current_movement, trail, _flag)|{
                trail.points.push(Point::from_slice(&[current_pos.x, current_pos.y]));

                let d_t = time_step.0;
                current_movement.vel.0 += (current_movement.accel.0 + current_movement.past_accel.0)/2.0 * d_t;
                current_movement.vel.1 += (current_movement.accel.1 + current_movement.past_accel.1)/2.0 * d_t;
                current_pos.x += current_movement.vel.0 * d_t + current_movement.accel.0/2.0 * d_t.powi(2);
                current_pos.y += current_movement.vel.1 * d_t + current_movement.accel.1/2.0 * d_t.powi(2);

                current_movement.past_accel = current_movement.accel;
            });
        }
    }
}

fn distance(a: &Pos, b: &Pos) -> f32{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
