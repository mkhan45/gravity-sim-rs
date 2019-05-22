use specs::prelude::*;

use ggez::{
    input, GameResult, Context, graphics,
    input::{
        mouse::MouseButton, 
        keyboard::{KeyCode, KeyMods},
    },
    graphics::{DrawParam},
    event::{EventHandler},
    mint::Point2,
};

use crate::components::*;
use crate::resources::*;

use std::f32::consts::PI;

type Point = Point2<f32>;

const DENSITY_MULTIPLIER: f32 = 0.0005;


pub struct MainState<'a, 'b>{
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    start_point: Point,
    radius: f32,
    density: f32,
    paused: bool,
}

impl<'a, 'b> MainState<'a, 'b>{
    pub fn new(world: World, dispatcher: Dispatcher<'a, 'b>) -> Self{
        MainState{
            world,
            dispatcher,
            start_point: Point::from_slice(&[0.0, 0.0]),
            radius: 15.0,
            density: 0.5,
            paused: false,
        }
    }
}

impl<'a, 'b> EventHandler for MainState<'a, 'b>{
    fn update(&mut self, ctx: &mut Context) -> GameResult{
        if !self.paused{
            let mut sim_speed = 1;

            {
                sim_speed = (self.world.read_resource::<SimSpeed>()).0;
            }

            for _i in 0..sim_speed {
                self.world.maintain();
                self.dispatcher.dispatch(&mut self.world.res);
            }

            self.world.maintain();
            self.dispatcher.dispatch(&mut self.world.res);


            if ggez::timer::ticks(ctx) % 60 == 0{
                println!("FPS: {}", ggez::timer::fps(ctx));
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
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult{
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        {
            let screen_coords = graphics::screen_coordinates(ctx);
            let scale = screen_coords.w/1000.0;

            let info = if !self.paused{
                format!(
                    "
                Offset: {x}, {y}
                Zoom {zoom}
                Radius: {radius}
                Density: {density}
                Time Step: {timestep}
                Sim Speed: {sim_speed}
                Prediction Speed: {prediction_speed}
                Press space to pause and view keybinds 
                ",

                x = screen_coords.x,
                y = screen_coords.y, 
                zoom = scale,
                radius = self.radius,
                density = self.density,
                timestep = self.world.read_resource::<TimeStep>().0,
                sim_speed = self.world.read_resource::<SimSpeed>().0,
                prediction_speed = self.world.read_resource::<PredictionSpeed>().0)
            }else {
                format!(
                    "
                Q and A to increase/decrease radius
                W and S to increase/decrease density
                1 and 2 to increase/decrease step size
                3 and 4 to increase/decrease sim speed
                5 and 6 to increase/decrease prediction speed
                ")
            };

            let text = graphics::Text::new(info);
            let params = graphics::DrawParam::new()
                .scale([scale, scale])
                .dest([screen_coords.x, screen_coords.y]);
            graphics::draw(ctx, &text, params).expect("error drawing text");
        }

        let positions = self.world.read_storage::<Pos>();
        let radii = self.world.read_storage::<Radius>();
        let trails = self.world.read_storage::<Trail>();
        let flags = self.world.read_storage::<PreviewFlag>();
        let entities = self.world.entities();

        for (trail, radius, entity) in (&trails, &radii, &entities).join(){
            if trail.points.len() > 2{
                let flag = flags.get(entity);
                let color = match flag{
                    Some(_flag) => graphics::Color::new(0.2, 1.0, 0.5, 0.35),
                    None => graphics::Color::new(0.1, 0.25, 1.0, 0.5),
                };

                let result = graphics::Mesh::new_line(
                    ctx,
                    &trail.points.as_slice(),
                    0.2 * radius.0,
                    color);

                match result{
                    Ok(line_mesh) => graphics::draw(ctx, &line_mesh, DrawParam::new()).expect("error drawing outline"),
                    Err(_e) => {},
                }
            }
        }


        for (position, radius, entity) in (&positions, &radii, &entities).join(){
            let flag = flags.get(entity);

            let color = match flag{
                Some(_flag) => graphics::Color::new(0.2, 1.0, 0.5, 0.5),
                None => graphics::Color::new(1.0, 1.0, 1.0, 1.0),
            };


            let outline = graphics::Mesh::new_circle( //draw bodies
                ctx,
                graphics::DrawMode::fill(),
                [position.x, position.y],
                radius.0,
                0.1,
                color)
                .expect("error building body mesh");

            graphics::draw(ctx, &outline, DrawParam::new()).expect("error drawing outline");
        }


        let mut mouse_pos = input::mouse::position(ctx);

        let scale = graphics::screen_coordinates(ctx).w / 1000.0;
        let scaled_x = mouse_pos.x * scale + graphics::screen_coordinates(ctx).x;
        let scaled_y = mouse_pos.y * scale + graphics::screen_coordinates(ctx).y;

        mouse_pos = Point::from_slice(&[scaled_x, scaled_y]);

        let mouse_pressed = input::mouse::button_pressed(ctx, MouseButton::Left);

        if mouse_pressed{
            if mouse_pos != self.start_point{ //draw preview vector

                let line = graphics::Mesh::new_line(
                    ctx,
                    &[self.start_point, mouse_pos][..],
                    0.25 * 10.0,
                    graphics::Color::new(1.0, 1.0, 1.0, 0.8))
                    .expect("error building preview line mesh");

                graphics::draw(ctx, &line, DrawParam::new()).expect("error drawing preview line");
            }
        }

        let preview_outline = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            if mouse_pressed {[self.start_point.x, self.start_point.y]} else {[mouse_pos.x, mouse_pos.y]},
            self.radius,
            0.1,
            graphics::Color::new(1.0, 1.0, 1.0, 0.5))
            .expect("error building preview outline");

        graphics::draw(ctx, &preview_outline, DrawParam::new()).expect("error drawing outline");

        graphics::present(ctx).expect("error rendering");
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32){
        match button{
            MouseButton::Left => {
                let scale = graphics::screen_coordinates(ctx).w / 1000.0;
                let scaled_x = x * scale + graphics::screen_coordinates(ctx).x;
                let scaled_y = y * scale + graphics::screen_coordinates(ctx).y;

                let vector_x = (scaled_x - self.start_point.x) * 0.1; 
                let vector_y = (scaled_y - self.start_point.y) * 0.1; 

                self.world.create_entity()
                    .with(Pos{x: self.start_point.x, y: self.start_point.y})
                    .with(Movement::new(vector_x, vector_y))
                    .with(Radius(self.radius))
                    .with(Mass(DENSITY_MULTIPLIER * self.density * (4.0/3.0) * PI * self.radius.powi(3)))
                    .with(Trail::new(30))
                    .build();

                println!("Mass: {}", DENSITY_MULTIPLIER * self.density * (4.0/3.0) * PI * self.radius.powi(3));

                let entities = self.world.entities();
                let flags = self.world.read_storage::<PreviewFlag>();

                for (entity, _flag) in (&entities, &flags).join(){
                    entities.delete(entity).expect("error deleting preview");
                }
            },

            _ => {},
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32){
        let scale = graphics::screen_coordinates(ctx).w / 1000.0;

        let scaled_x = x * scale + graphics::screen_coordinates(ctx).x;
        let scaled_y = y * scale + graphics::screen_coordinates(ctx).y;

        match button{
            MouseButton::Left => {

                self.start_point = Point::from_slice(&[scaled_x, scaled_y]);
            },

            MouseButton::Right => {
                let entities = self.world.entities();
                let positions = self.world.read_storage::<Pos>();
                let radii = self.world.read_storage::<Radius>();

                (&entities, &positions, &radii).par_join().for_each(|(entity, pos, radius)|{
                    if distance((pos.x, pos.y), (scaled_x, scaled_y)) < radius.0{
                        entities.delete(entity).expect("error deleting");
                    }
                });
            }

            _ => {},
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32){
        if input::mouse::button_pressed(ctx, MouseButton::Left){
            let scale = graphics::screen_coordinates(ctx).w / 1000.0;
            let scaled_x = x * scale + graphics::screen_coordinates(ctx).x;
            let scaled_y = y * scale + graphics::screen_coordinates(ctx).y;

            let vector_x = (scaled_x - self.start_point.x) * 0.1; 
            let vector_y = (scaled_y - self.start_point.y) * 0.1; 

            let mut positions = self.world.write_storage::<Pos>();
            let mut movements = self.world.write_storage::<Movement>();
            let mut radii = self.world.write_storage::<Radius>();
            let mut flags = self.world.write_storage::<PreviewFlag>();
            let mut trails = self.world.write_storage::<Trail>();
            let entities = self.world.entities();

            for (entity, _flag) in (&entities, &flags).join(){
                entities.delete(entity).expect("error deleting preview");
            }

            self.world.entities().build_entity()
                .with(Pos{x: self.start_point.x, y: self.start_point.y}, &mut positions)
                .with(Movement::new(vector_x, vector_y), &mut movements)
                .with(Radius(self.radius), &mut radii)
                .with(PreviewFlag, &mut flags)
                .with(Trail::new(1), &mut trails)
                .build();
        }
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32){
        let mut screen = graphics::screen_coordinates(ctx);

        let prev_zoom = screen.w/1000.0;

        screen.scale(1.0 + (y * -0.08), 1.0 + (y * -0.08));

        let delta_zoom = (screen.w/1000.0 - prev_zoom) * -1.0;

        let mut mouse_pos = input::mouse::position(ctx);

        let scale = screen.w / 1000.0;

        mouse_pos.x = (mouse_pos.x * scale) + screen.x;
        mouse_pos.y = (mouse_pos.y * scale) + screen.y;

        let mut focus = [mouse_pos.x, mouse_pos.y];
        focus[0] *= delta_zoom;
        focus[1] *= delta_zoom;

        screen.translate(focus);

        graphics::set_screen_coordinates(ctx, screen).expect("error scaling screen");
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool){
        match keycode{
            KeyCode::G => grid(&graphics::screen_coordinates(ctx).point(), &self.radius, &self.density, &(graphics::screen_coordinates(ctx).w/1000.0), &mut self.world),
            KeyCode::Space => self.paused = !self.paused,
            _ => {},
        };

        self.radius += match keycode{
            KeyCode::Q => 1.0,
            KeyCode::A => -1.0,
            _ => 0.0,
        };

        self.density += match keycode{
            KeyCode::W => 0.5,
            KeyCode::S => -0.5,
            _ => 0.0,
        };

        let mut delta_t = self.world.write_resource::<TimeStep>();
        (*delta_t).0 += match keycode{
            KeyCode::Key1 => -0.05,
            KeyCode::Key2 => 0.05,
            _ => 0.0,
        };

        let mut sim_speed = self.world.write_resource::<SimSpeed>();
        (*sim_speed).0 = match keycode{
            KeyCode::Key3 => if sim_speed.0 > 1 {sim_speed.0 - 1} else {sim_speed.0},
            KeyCode::Key4 => sim_speed.0 + 1,
            _ => sim_speed.0,
        };

        let mut prediction_speed = self.world.write_resource::<PredictionSpeed>();
        (*prediction_speed).0 = match keycode{
            KeyCode::Key5 => if prediction_speed.0 > 1 {prediction_speed.0 - 1} else {prediction_speed.0},
            KeyCode::Key6 => prediction_speed.0 + 1,
            _ => if prediction_speed.0 < sim_speed.0 {sim_speed.0} else {prediction_speed.0},
        };
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
                .with(Movement::new(0.0, 0.0))
                .with(Mass(mass))
                .with(Radius(*radius))
                .with(Trail::new(10))
                .build();
        });
    });
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32{
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}
