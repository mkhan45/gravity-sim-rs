use specs::prelude::*;
use specs::world::EntitiesRes;

use crate::components::*;

use std::f32::consts::PI;

use std::collections::HashSet;

pub struct CollisionSys;

impl<'a> System<'a> for CollisionSys{
    type SystemData = (Entities<'a>, WriteStorage<'a, Pos>, WriteStorage<'a, Radius>, WriteStorage<'a, Mass>, WriteStorage<'a, Vel>);

    fn run(&mut self, (entities, mut positions, mut radii, mut masses, mut velocities): Self::SystemData){
        let mut new_bodies: Vec<(Pos, Radius, Mass, Vel)> = Vec::new();
        let mut collided: Vec<&Pos> = Vec::new();

        for (entity, pos, radius, mass, vel) in (&entities, &positions, &radii, &masses, &velocities).join(){
            for (other_entity, other_pos, other_rad, other_mass, other_vel) in (&entities, &positions, &radii, &masses, &velocities).join(){
                let distance = distance(&pos, &other_pos);

                if distance <= radius.0 + other_rad.0 && entity != other_entity{
                    entities.delete(entity).expect("no entity");

                    if !collided.contains(&other_pos){
                        let momentum_1 = (mass.0 * vel.x, mass.0 * vel.y);
                        let momentum_2 = (other_mass.0 * other_vel.x, other_mass.0 * other_vel.y);

                        let total_momentum = (momentum_1.0 + momentum_2.0, momentum_1.1 + momentum_2.1);

                        let volume_1 = 4.0/3.0 * PI * radius.0.powi(3);
                        let volume_2 = 4.0/3.0 * PI * other_rad.0.powi(3);

                        let total_volume = volume_1 + volume_2;
                        let total_mass = mass.0 + other_mass.0;

                        let new_rad = ( ((3.0/4.0)*total_volume)/PI ).powf(1.0/3.0);

                        new_bodies.push((
                                if radius.0 > other_rad.0 {Pos{x: pos.x, y: pos.y}} else {Pos{x: other_pos.x, y: other_pos.y}},
                                Radius(new_rad),
                                Mass(total_mass),
                                Vel{x: total_momentum.0/total_mass, y: total_momentum.1/total_mass}));

                        collided.push(pos);
                    }
                }
            }
        }
        new_bodies.iter()
            .for_each(|body|{
                entities.build_entity()
                    .with(body.0.clone(), &mut positions)
                    .with(body.1.clone(), &mut radii)
                    .with(body.2.clone(), &mut masses)
                    .with(body.3.clone(), &mut velocities)
                    .build();
            });
    }
}

fn distance(a: &Pos, b: &Pos) -> f32{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
