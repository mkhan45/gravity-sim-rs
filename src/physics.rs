use ggez::nalgebra as na;
use std::f32::consts::PI;
use crate::body::Body;
use std::collections::HashSet;

use rayon::prelude::*;


type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

const G: f32 = 6.674;

pub fn collide(body1: &Body, body2: &Body) -> Body{ //inelastic collision that conserves momentum
    let body1_momentum = Vector2::new(body1.velocity.x * body1.mass, body1.velocity.y * body1.mass);
    let body2_momentum = Vector2::new(body2.velocity.x * body2.mass, body2.velocity.y * body2.mass);

    let total_momentum = body1_momentum + body2_momentum;

    let volume_1 = 4.0/3.0 * PI * body1.radius.powi(3);
    let volume_2 = 4.0/3.0 * PI * body2.radius.powi(3);

    let total_mass = body1.mass + body2.mass;

    let total_volume = volume_1 + volume_2;

    let new_rad = ( ((3.0/4.0)*total_volume)/PI ).powf(1.0/3.0); //add volumes

    Body::new(
        if body1.radius > body2.radius {Point2::new(body1.pos.x, body1.pos.y)} else {Point2::new(body2.pos.x, body2.pos.y)}, //take position of bigger body
        total_mass,
        body1.charge + body2.charge,
        new_rad,
        total_momentum/total_mass,
    )
}

pub fn distance(a: &Point2, b: &Point2) -> f32{
    ((b.x - a.x).powi(2) + (b.y-a.y).powi(2)).sqrt()
}

pub fn angle(a: &Point2, b: &Point2) -> f32{
    let restricted_dom = ((b.y - a.y)/(b.x - a.x)).atan(); //.atan() returns from -pi/2 to +pi/2

    if b.x >= a.x {restricted_dom + PI}
    else {restricted_dom}
}

pub fn update_velocities_and_collide(bodies: &Vec<Body>, method: &Integrator, step_size: &f32) -> Vec<Body>{
        let bodies_clone = bodies.clone();
        let mut bodies = bodies.clone();

        bodies.par_iter_mut() //parallel, so I can only change stuff in the iterator
            .for_each(|current_body|{ //in this case I can only change current_body
                current_body.current_accel = Vector2::new(0.0, 0.0);

                &bodies_clone.iter() //could maybe make this parallel by folding into a tuple (accel, collision)
                    .enumerate()
                    .for_each(|(other_i, other_body)|{ //other_body is an old version of it from before the loop
                        let r = distance(&other_body.pos, &current_body.pos);

                        if r <= other_body.radius + current_body.radius{
                            current_body.collision = Some(other_i);
                        }else{
                            let a_mag = (G*&other_body.mass)/(r.powi(2)); //acceleration = Gm_2/r^2
                            let angle = angle(&other_body.pos, &current_body.pos);

                            current_body.current_accel += Vector2::new(angle.cos() * a_mag, angle.sin() * a_mag);
                        }
                    });

                current_body.update_trail();
                match method{
                    &Integrator::Euler => current_body.update_euler(step_size),
                    &Integrator::Verlet => current_body.update_verlet(step_size),
                };
            });
        
        
        let mut collided: HashSet<usize> = HashSet::new();

        //because there are duplicate collisions we need a set to keep track
        (0..bodies.len()).for_each(|i|{ 
            match bodies[i].collision {
                None => {},
                Some(index) => {
                    if !collided.contains(&index) && i != index{
                        bodies.push(collide(&bodies[i], &bodies[index]));
                        collided.insert(index);
                        collided.insert(i);
                    }
                }
            }
        });

        //remove collided

        if collided.len() != 0{
            bodies.par_iter()
                .enumerate()
                .filter_map(|(index, body)|{
                    if collided.contains(&index){
                        None
                    }else {
                        Some(body.to_owned())
                    }
                }).collect()
        }else{
            bodies
        }
}

#[derive(Debug, Copy, Clone)]
pub enum Integrator{
    Euler,
    Verlet,
}
