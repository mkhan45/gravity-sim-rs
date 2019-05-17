extern crate quicksilver;
use quicksilver::{
    Result,
    geom::{Vector, Shape},
    graphics::{Background, Color, Drawable, Font, FontStyle, Background::Img, Image},
    lifecycle::{Settings, State, Window, run, Event, Asset},
    input::{MouseButton, Key, ButtonState}
};

use std::{thread, time};

extern crate specs;
use specs::prelude::*;

mod systems;
mod components;

use systems::*;
use components::*;

fn main() {
    let mut world = World::new();


    let mut dispatcher = DispatcherBuilder::new()
        .with(MoveSys, "move_system", &[])
        .with(GraviSys, "gravity_system", &[])
        .build();

    dispatcher.setup(&mut world.res);


    world.create_entity()
        .with(Vel{x: 0.0, y: 0.0})
        .with(Pos{x: 0.0, y: 0.0})
        .with(Mass(5.0))
        .build();

    world.create_entity()
        .with(Vel{x: 0.0, y: 0.0})
        .with(Pos{x: 100.0, y: 0.0})
        .with(Mass(5.0))
        .build();

    loop{
        dispatcher.dispatch(&mut world.res);
        world.maintain();
        thread::sleep(time::Duration::from_millis((1000.0/60.0) as u64));
    }
}


struct Mass(f32);

impl Component for Mass{
    type Storage = DenseVecStorage<Self>;
}

struct GraviSys;

impl<'a> System<'a> for GraviSys{
    type SystemData = (ReadStorage<'a, Pos>, WriteStorage<'a, Vel>, ReadStorage<'a, Mass>);

    fn run(&mut self, (pos, mut vel, mass): Self::SystemData){
        for (current_pos, current_vel, current_mass) in (&pos, &mut vel, &mass).join(){
            for (other_pos, other_mass) in (&pos, &mass).join(){
                let distance = distance(current_pos, other_pos);

                if distance >= 1.0{
                    let x_comp = other_pos.x - current_pos.x;
                    let y_comp = other_pos.y - current_pos.y;

                    let magnitude = ((current_mass.0 * other_mass.0) / distance.powi(2))/10.0;
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
