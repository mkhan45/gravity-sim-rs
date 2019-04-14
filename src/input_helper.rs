use rayon::prelude::*;
use ggez::input;
use ggez::input::keyboard;
use ggez::nalgebra as na;

use crate::body::Body;
use crate::physics::Integrator;
use crate::presets;

use std::collections::HashSet;

use crate::MainState::*;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

pub fn process_input(keys: &HashSet<input::keyboard::KeyCode>, game: &mut MainState){
    keys.iter() //I think this is better than just a bunch of if statements
        .for_each(|keycode|{
            game.offset.y += match keycode{
                input::keyboard::KeyCode::Up => 10.0,
                input::keyboard::KeyCode::Down => -10.0,
                _ => 0.0,
            };

            game.offset.x += match keycode{
                input::keyboard::KeyCode::Left => 10.0,
                input::keyboard::KeyCode::Right => -10.0,
                _ => 0.0,
            };

            game.density += match keycode{
                input::keyboard::KeyCode::W => 0.05,
                input::keyboard::KeyCode::S => -0.05,
                _ => 0.0,
            };

            game.radius += match keycode{
                input::keyboard::KeyCode::Q => 1.0,
                input::keyboard::KeyCode::A => -1.0,
                _ => 0.0,
            };

            game.trail_length = match keycode{
                input::keyboard::KeyCode::E => game.trail_length + 1,
                input::keyboard::KeyCode::D => if game.trail_length != 0 {game.trail_length - 1} else {0},
                _ => game.trail_length,
            };

            game.predict_speed = match keycode {
                input::keyboard::KeyCode::X => game.predict_speed + 1,
                input::keyboard::KeyCode::Z => if game.predict_speed != 0 {game.predict_speed - 1} else {0},
                _ => game.predict_speed,
            };

            game.fast_forward = match keycode {
                input::keyboard::KeyCode::Key1 => if game.fast_forward == 1 {1} else {game.fast_forward - 1},
                input::keyboard::KeyCode::Key2 => game.fast_forward + 1,
                _ => game.fast_forward,
            };

            game.step_size += match keycode {
                input::keyboard::KeyCode::Key3 => -0.1,
                input::keyboard::KeyCode::Key4 => 0.1,
                _ => 0.0,
            };

            match keycode{ //misc keys
                input::keyboard::KeyCode::Space => game.paused = !game.paused,

                input::keyboard::KeyCode::G => game.bodies.append(&mut presets::grid(&game.offset, &game.radius, &game.density, &game.zoom)),

                input::keyboard::KeyCode::R => {
                    game.bodies = vec![
                        Body::new(
                            Point2::new(500.0, 400.0),
                            300000.0,
                            100.0,
                            Vector2::new(0.0, 0.0)),
                    ];
                    game.zoom = 1.0;
                    game.offset = Point2::new(0.0, 0.0);
                    game.fast_forward = 1;
                }

                input::keyboard::KeyCode::I => {
                    game.integrator = match game.integrator {
                        Integrator::Euler => Integrator::Verlet,
                        Integrator::Verlet => Integrator::Euler,
                    };
                }

                input::keyboard::KeyCode::H => game.help_menu = !game.help_menu,

                _ => {},
            };
        });


    if game.radius < 1.0 {game.radius = 1.0};

    //rounds to 3 digits
    game.radius = (game.radius * 1000.0).round()/1000.0;
    game.density = (game.density * 1000.0).round()/1000.0;
    game.step_size = (game.step_size * 1000.0).round()/1000.0;
}
