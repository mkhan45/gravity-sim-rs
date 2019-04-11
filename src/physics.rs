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
        let mut bodies = bodies.clone();
        let mut collision_blacklist = HashSet::new();
        let mut collision_bodies = Vec::new();
        microprofile::scope!("Update velocities/collide", "Calculations");

        for current_body_i in 0..bodies.len(){
            bodies[current_body_i].current_accel = Vector2::new(0.0, 0.0);

            for other_body_i in 0..bodies.len(){
                if other_body_i != current_body_i {
                    let other_body = &bodies[other_body_i].clone();
                    let current_body = &mut bodies[current_body_i];

                    let r = distance(&other_body.pos, &current_body.pos);
                    let a_mag = (G*other_body.mass)/(r.powi(2)); //acceleration = Gm_2/r^2
                    let angle = angle(&other_body.pos, &current_body.pos);

                    //if two bodies collide, add them to remove list and create new body that's a combination of both
                    if r <= other_body.radius + current_body.radius && !collision_blacklist.contains(&current_body_i){
                        collision_blacklist.insert(current_body_i);
                        collision_blacklist.insert(other_body_i);
                        collision_bodies.push(collide(&current_body, &other_body));
                    }

                    current_body.current_accel += Vector2::new(angle.cos() * a_mag, angle.sin() * a_mag);
                }
            }
            
            bodies[current_body_i].update_trail();
            match method {
                &Integrator::Euler => bodies[current_body_i].update_euler(step_size),
                &Integrator::Verlet => bodies[current_body_i].update_verlet(step_size),
            };
        }

        bodies = bodies.par_iter() //remove all bodies in collision_blacklist
            .enumerate()
            .filter_map(|(index, body)| {
                if collision_blacklist.contains(&index) {
                    None
                } else {
                    Some(body.clone())
                }
            }).collect();
        
        bodies.append(&mut collision_bodies);
        return bodies;
}

#[derive(Debug, Clone)]
pub enum Integrator{
    Euler,
    Verlet,
}
