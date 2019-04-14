#[macro_use] extern crate microprofile;
extern crate ggez;
use ggez::GameResult;
use ggez::event;

mod presets;

mod MainState;

mod physics;
mod body;
mod input_helper;


const G: f32 = 6.674;




pub fn main() -> GameResult{
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("N-body gravity sim", "Fish")
        .window_setup(ggez::conf::WindowSetup::default().title("N-body gravity sim"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0, 800.0))
        .build().expect("error building context");
    let state = &mut MainState::MainState::new().clone();

    microprofile::init();
    microprofile::set_enable_all_groups(true);
    event::run(ctx, event_loop, state)
}
