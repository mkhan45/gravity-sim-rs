use specs::prelude::*;

use crate::components::*;

use std::f32::consts::PI;

pub struct CollisionSys;

impl<'a> System<'a> for CollisionSys{
    type SystemData = (Entities<'a>, WriteStorage<'a, Pos>, WriteStorage<'a, Radius>, 
                       WriteStorage<'a, Mass>, WriteStorage<'a, Movement>, WriteStorage<'a, Trail>);

    fn run(&mut self, (entities, mut positions, mut radii, mut masses, mut movements, mut trails): Self::SystemData){
        let mut new_bodies: Vec<(Pos, Radius, Mass, Movement, Trail)> = Vec::new();
        let mut collided: Vec<&Pos> = Vec::new();

        for (entity, pos, radius, mass, movement, trail) in (&entities, &positions, &radii, &masses, &movements, &trails).join(){
            for (other_entity, other_pos, other_rad, other_mass, other_movements, other_trail) in (&entities, &positions, &radii, &masses, &movements, &trails).join(){
                let distance = distance(&pos, &other_pos);
                let vel = movement.vel;

                if distance <= radius.0 + other_rad.0 && entity != other_entity{
                    entities.delete(entity).expect("no entity");

                    let other_vel = other_movements.vel;

                    if !collided.contains(&other_pos) && !collided.contains(&pos){
                        let momentum_1 = (mass.0 * vel.0, mass.0 * vel.1);
                        let momentum_2 = (other_mass.0 * other_vel.0, other_mass.0 * other_vel.1);

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
                                Movement::new(total_momentum.0/total_mass, total_momentum.1/total_mass),
                                if mass.0 > other_mass.0 {trail.clone()} else {other_trail.clone()}));

                        collided.push(pos);
                        collided.push(other_pos);
                    }
                }
            }
        }

        // (&positions, &radii, &entities, &flags).join()
        //     .for_each(|(position, radius, entity, flag)|{
        //         (&positions, &radii, !&flags).par_join()
        //             .for_each(|(other_pos, other_rad, ())|{
        //                 let distance = distance(&position, &other_pos);

        //                 if distance < radius.0 + other_rad.0{
        //                     entities.delete(entity).expect("error deleting collided preview");
        //                 }
        //             });
        //     });

        new_bodies.iter()
            .for_each(|body|{
                entities.build_entity()
                    .with(body.0.clone(), &mut positions)
                    .with(body.1.clone(), &mut radii)
                    .with(body.2.clone(), &mut masses)
                    .with(body.3.clone(), &mut movements)
                    .with(body.4.clone(), &mut trails)
                    .build();
            });
    }
}


pub struct PreviewCollisionSys;

impl<'a> System<'a> for PreviewCollisionSys{
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>, ReadStorage<'a, Radius>, ReadStorage<'a, PreviewFlag>);

    fn run(&mut self, (entities, positions, radii, flags): Self::SystemData){
        (&entities, &positions, &radii, &flags).join()
            .for_each(|(entity, position, radius, _flag)|{
                (&positions, &radii, !&flags).par_join()
                    .for_each(|(other_position, other_rad, ())|{
                        let distance = distance(&position, &other_position);

                        if distance < other_rad.0 + radius.0{
                            entities.delete(entity).expect("error deleting preview on collision");
                        }
                    });
            });
    }
}

fn distance(a: &Pos, b: &Pos) -> f32{
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
