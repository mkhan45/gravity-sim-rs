extern crate ggez;
use ggez::{event, GameResult};

extern crate specs;
use specs::prelude::*;

mod systems;
mod components;

use systems::*;
use components::*;

mod resources;
use resources::*;

mod main_state;
use main_state::MainState;

fn main() -> GameResult {
    let mut world = World::new();

    world.add_resource(TimeStep(0.5));
    world.add_resource(PredictionSpeed(1));
    world.add_resource(SimSpeed(1));
    world.add_resource(MaxPredictions(1));
    world.add_resource(CurrentPredictions(0));

    let mut dispatcher = DispatcherBuilder::new()
        .with(GraviSys, "gravity_system", &[])
        .with(MoveSys, "move_system", &["gravity_system"])
        .with(PreviewSpeedSys, "preview_move_sys", &["gravity_system", "move_system"])
        .with(CollisionSys, "collision_system", &[])
        .with(PreviewCollisionSys, "preview_collision_system", &[])
        .with(TrailSys, "trail_system", &[])
        .build();

    dispatcher.setup(&mut world.res);


    world.create_entity()
        .with(Movement::new(0.0, 0.0))
        .with(Pos{x: 500.0, y: 400.0})
        .with(Mass(450_000.0))
        .with(Radius(50.0))
        .with(Trail::new(30))
        .build();

    // for i in 0..1100{
    //     world.create_entity()
    //         .with(Movement::new(0.0, 0.0))
    //         .with(Pos{x: i as f32 * 100.0, y: i as f32 * 100.0})
    //         .with(Mass(-1.0))
    //         .with(Radius(15.0))
    //         .build();
    // }

    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("N-body gravity sim", "Fish")
        .window_setup(ggez::conf::WindowSetup::default().title("N-body gravity sim"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0, 800.0))
        .build().expect("error building context");

    let main_state = &mut MainState::new(world, dispatcher);

    event::run(ctx, event_loop, main_state)
}
