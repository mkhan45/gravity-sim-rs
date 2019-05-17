extern crate ggez;
use ggez::{event, GameResult};

extern crate specs;
use specs::prelude::*;

mod systems;
mod components;

use systems::*;
use components::*;


mod main_state;
use main_state::MainState;

fn main() -> GameResult {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(MoveSys, "move_system", &[])
        .with(GraviSys, "gravity_system", &[])
        .with(CollisionSys, "collision_system", &[])
        .build();

    dispatcher.setup(&mut world.res);

    world.register::<Radius>();

    // world.create_entity()
    //     .with(Vel{x: 0.0, y: 0.0})
    //     .with(Pos{x: 0.0, y: 30.0})
    //     .with(Mass(1.0))
    //     .with(Radius(15.0))
    //     .build();

    // world.create_entity()
    //     .with(Vel{x: 0.0, y: 0.0})
    //     .with(Pos{x: 200.0, y: 100.0})
    //     .with(Mass(1.0))
    //     .with(Radius(15.0))
    //     .build();

    for i in 0..1000{
        world.create_entity()
            .with(Vel{x: 0.0, y: 0.0})
            .with(Pos{x: i as f32 * 100.0, y: 400.0})
            .with(Mass(-1.0))
            .with(Radius(15.0))
            .build();
    }

    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("N-body gravity sim", "Fish")
        .window_setup(ggez::conf::WindowSetup::default().title("N-body gravity sim"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0, 800.0))
        .build().expect("error building context");

    let main_state = &mut MainState::new(world, dispatcher);

    event::run(ctx, event_loop, main_state)
}
