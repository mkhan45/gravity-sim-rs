use specs::prelude::*;

use ggez::{
    nalgebra as na, input, GameResult, Context, graphics,
    input::{
        mouse::MouseButton, 
        keyboard::{KeyCode, KeyMods},
    },
    graphics::{DrawParam},
    event::{EventHandler},
    mint::Point2,
};

use crate::components::*;

use std::f32::consts::PI;

pub struct MainState<'a, 'b>{
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b>{
    pub fn new(world: World, dispatcher: Dispatcher<'a, 'b>) -> Self{
        MainState{
            world,
            dispatcher,
        }
    }
}

impl<'a, 'b> EventHandler for MainState<'a, 'b>{
    fn update(&mut self, ctx: &mut Context) -> GameResult{
        self.world.maintain();
        self.dispatcher.dispatch(&mut self.world.res);

        if ggez::timer::ticks(ctx) % 60 == 0{
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        if input::mouse::button_pressed(ctx, MouseButton::Left){
        }

        {
            let mut screen = graphics::screen_coordinates(ctx);
            let scale = screen.w/1000.0;
            if input::keyboard::is_key_pressed(ctx, KeyCode::Up){
                screen.translate([0.0, -10.0 * scale]);
            }else if input::keyboard::is_key_pressed(ctx, KeyCode::Down){
                screen.translate([0.0, 10.0 * scale]);
            }

            if input::keyboard::is_key_pressed(ctx, KeyCode::Left){
                screen.translate([-10.0 * scale, 0.0]);
            }else if input::keyboard::is_key_pressed(ctx, KeyCode::Right){
                screen.translate([10.0 * scale, 0.0]);
            }

            graphics::set_screen_coordinates(ctx, screen).expect("error moving screen");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult{
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let positions = self.world.read_storage::<Pos>();
        let radii = self.world.read_storage::<Radius>();

        for (position, radius) in (&positions, &radii).join(){
            let outline = graphics::Mesh::new_circle( //draw outline
                ctx,
                graphics::DrawMode::fill(),
                [position.x, position.y],
                radius.0,
                0.25,
                graphics::Color::new(1.0, 1.0, 1.0, 1.0))
                .expect("error building outline");

            graphics::draw(ctx, &outline, DrawParam::new()).expect("error drawing outline");
        }

        graphics::present(ctx).expect("error rendering");
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32){
        match button{
            MouseButton::Left => {
                self.world.create_entity()
                    .with(Pos{x, y})
                    .with(Vel{x: 0.0, y: 0.0})
                    .with(Radius(15.0))
                    .with(Mass(5.0))
                    .build();
            },

            _ => {},
        }
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32){
        let mut screen = graphics::screen_coordinates(ctx);

        let prev_zoom = screen.w/1000.0;

        screen.scale(1.0 + (y * -0.08), 1.0 + (y * -0.08));

        let delta_zoom = (screen.w/1000.0 - prev_zoom) * -1.0;
        println!("{}", delta_zoom);

        if delta_zoom < 0.0{
            screen.translate([(screen.point().x - input::mouse::position(ctx).x) * delta_zoom, (screen.point().y - input::mouse::position(ctx).y) * delta_zoom]);
        }else{
            screen.translate([(screen.point().x + input::mouse::position(ctx).x) * delta_zoom, (screen.point().y + input::mouse::position(ctx).y) * delta_zoom]);
        }

        graphics::set_screen_coordinates(ctx, screen).expect("error scaling screen");
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool){
        match keycode{
            KeyCode::G => grid(&graphics::screen_coordinates(ctx).point(), &15.0, &0.001, &(graphics::screen_coordinates(ctx).w/1000.0), &mut self.world),
            _ => {},
        }
    }
}

fn grid(start: &Point2<f32>, radius: &f32, density: &f32, zoom: &f32, world: &mut World){
    //create a 10x10 grid of bodies
    (1..=10).for_each(|y|{
        (1..=10).for_each(|x| {
            let point = ((x as f32 * radius * 50.0) - (start.x * (1.0/zoom)), (y as f32 * radius * 50.0) - (start.y * (1.0/zoom)));
            let mass = PI * 4.0/3.0 * radius.powi(3) * density;

            world.create_entity()
                .with(Pos{x: point.0, y: point.1})
                .with(Vel{x: 0.0, y: 0.0})
                .with(Mass(mass))
                .with(Radius(*radius))
                .build();
        });
    });
}
