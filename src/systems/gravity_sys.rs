use specs::prelude::*;

use crate::components::*;

use std::f32::consts::PI;

const MULTIPLIER: f32 = 50.0;

pub struct GraviSys;

impl<'a> System<'a> for GraviSys{
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Vel>, ReadStorage<'a, Mass>);

    fn run(&mut self, (pos, mut vel, mass): Self::SystemData){
        for (current_pos, current_vel) in (&pos, &mut vel).join(){
            for (other_pos, other_mass) in (&pos, &mass).join(){
                let distance = distance(current_pos, other_pos);

                if  distance != 0.0{ //also makes sure not the same body
                    let x_comp = other_pos.x - current_pos.x;
                    let y_comp = other_pos.y - current_pos.y;

                    let magnitude = (other_mass.0 / distance.powi(2)) * MULTIPLIER * 1.0;
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

fn angle(a: &Pos, b: &Pos) -> f32{
    let restricted_dom = ((b.y - a.y)/(b.x - a.x)).atan(); //.atan() returns from -pi/2 to +pi/2

    if b.x >= a.x {restricted_dom + PI}
    else {restricted_dom}
}
