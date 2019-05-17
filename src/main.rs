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
