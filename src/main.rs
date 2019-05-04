extern crate quicksilver;
use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background, Background::Col, Color, Drawable},
    lifecycle::{Settings, State, Window, run},
    input::{MouseButton, Key, ButtonState}
};

use nalgebra as na;

mod body;
use body::Body;

mod physics;
use physics::*;

use std::collections::VecDeque;

use rayon::prelude::*;

const G: f32 = 6.674;

#[derive(Clone)]
struct MainState {
    bodies: Vec<Body>,
    start_point: Point2,
    zoom: f32,
    offset: Point2,
    density: f32,
    radius: f32,
    mouse_pos: Point2,
    trail_length: usize,
    mouse_pressed: bool, paused: bool, predict_body: Body,
    predict_speed: usize,
    integrator: Integrator,
    help_menu: bool,
    fast_forward: usize,
    step_size: f32,
}

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

impl State for MainState {
    fn new() -> Result<Self> {
        let bodies = vec![ //initialize with one massive body in center
            Body::new(
                Point2::new(500.0, 400.0), //position
                300000.0, //mass
                100.0,  //radius
                Vector2::new(0.0, 0.0)), //velocity
        ];

        Ok(MainState {
            bodies,
            start_point: Point2::new(0.0, 0.0),
            zoom: 1.0,
            offset: Point2::new(0.0, 0.0),
            density: 0.05,
            radius: 10.0,
            mouse_pos: Point2::new(0.0, 0.0),
            trail_length: 30,
            mouse_pressed: false,
            paused: false,
            predict_body: Body::new(Point2::new(0.0, 0.0), 1.0, 1.0, Vector2::new(0.0, 0.0)),
            predict_speed: 1,
            integrator: Integrator::Verlet,
            help_menu: false,
            fast_forward: 1,
            step_size: 1.0,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if !self.paused{ //physics sim
            (0..self.fast_forward).for_each(|_i|{
                self.bodies = update_velocities_and_collide(&self.bodies, &self.integrator, &self.step_size);

                (0..self.bodies.len()).for_each(|i|{
                    self.bodies[i].trail_length = self.trail_length;
                })
            })
        }



        let x = window.mouse().pos().x;
        let y = window.mouse().pos().y;
        self.mouse_pos = Point2::new(x, y);

        if window.mouse()[MouseButton::Left].is_down(){
            if self.mouse_pressed == false{ //on_press() basically
                self.start_point = Point2::new(x, y);
            }


            self.mouse_pressed = true;
        }else {
            if self.mouse_pressed == true { //on_release() kind of
                self.bodies.push(Body::new(
                        inv_scale(self.start_point, &self.offset, &self.zoom),
                        self.radius.powi(3) * self.density,
                        self.radius,
                        (self.mouse_pos - self.start_point) * 0.5));
            }

            self.mouse_pressed = false;
        }


        if window.keyboard()[Key::Right].is_down(){
            self.offset.x += 10.0/self.zoom;
        }else if window.keyboard()[Key::Left].is_down(){
            self.offset.x -= 10.0/self.zoom;
        }

        if window.keyboard()[Key::Up].is_down(){
            self.offset.y -= 10.0/self.zoom;
        }else if window.keyboard()[Key::Down].is_down(){
            self.offset.y += 10.0/self.zoom;
        }

        if window.keyboard()[Key::LShift].is_down(){
            let prev_zoom = self.zoom;
            self.zoom *= 0.95;
            let delta_zoom = self.zoom - prev_zoom;

            let focus = Vector2::new(self.mouse_pos.x + self.offset.x, self.mouse_pos.y + self.offset.y) * delta_zoom;
            self.offset += focus;
        }else if window.keyboard()[Key::LControl].is_down(){
            let prev_zoom = self.zoom;
            self.zoom *= 1.0/0.95;
            let delta_zoom = self.zoom - prev_zoom;

            let focus = Vector2::new(self.mouse_pos.x - self.offset.x, self.mouse_pos.y - self.offset.y) * delta_zoom;
            self.offset += focus;
        }

        if window.keyboard()[Key::G] == ButtonState::Pressed{
            let start = scale(self.offset, &Point2::new(0.0, 0.0), &(-1.0 * self.zoom));
            self.bodies.append(&mut grid(&start, &self.radius, &self.density, &self.zoom));
        }

        if window.keyboard()[Key::D] == ButtonState::Pressed{
            let mouse = inv_scale(self.mouse_pos, &self.offset, &self.zoom);
            self.bodies = self.bodies.iter() //iterate through meshes and delete any under mouse
                .filter_map(|body| {
                    if distance(&mouse, &body.pos) > body.radius {
                        Some(body.clone())
                    }else {
                        None
                    }
                }).collect();
        }

        ////simulate prediction
        //if self.mouse_pressed{
        //    for _i in 0..self.predict_speed { //reimplementation of update_bodies_and_collide() but for only predict body
        //        self.predict_body.current_accel = self.bodies.iter()
        //            .fold(Vector2::new(0.0, 0.0), |acc: Vector2, body|{
        //                let r = distance(&body.pos, &self.predict_body.pos);
        //                let a_mag = (G*body.mass)/(r.powi(2));
        //                let angle = angle(&body.pos, &self.predict_body.pos);
        //                acc + Vector2::new(a_mag * angle.cos(), a_mag * angle.sin())
        //            });

        //        self.predict_body.trail_length += 1; //infinite trail length
        //        self.predict_body.update_trail();

        //        match self.integrator{
        //            Integrator::Euler => self.predict_body.update_euler(&self.step_size),
        //            Integrator::Verlet => self.predict_body.update_verlet(&self.step_size),
        //        };
        //    }
        //}

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

            {
                ////top left ui text
                //let info = format!(
                //    "
                //    Offset: {x}, {y}
                //    Zoom: {zoom}
                //    Density: {density}
                //    Radius: {radius}
                //    Trail length: {trail_length}
                //    Prediction Speed: {prediction_speed}
                //    Integrator: {method}
                //    Sim Speed: {sim_speed}
                //    Step Size: {step_size}
                //    Press H for keybinds
                //    ",
                //    x = self.offset.x,
                //    y = self.offset.y, 
                //    zoom = self.zoom,
                //    density = self.density,
                //    radius = self.radius,
                //    trail_length = self.trail_length,
                //    prediction_speed = self.predict_speed,
                //    method = format!("{:?}", self.integrator),
                //    sim_speed = self.fast_forward,
                //    step_size = self.step_size);

                //let text = graphics::Text::new(info);
                //graphics::draw(ctx, &text, graphics::DrawParam::new()).expect("error drawing text");
            }

            for i in 0..self.bodies.len(){ //draw trail and bodies

                let curve = draw_line(&self.bodies[i].trail, self.offset, self.zoom);

                for segment in curve{
                    window.draw(&segment, Background::Col(Color::from_rgba(95, 136, 255, 0.5)));
                }

                let pos = scale(self.bodies[i].pos, &self.offset, &self.zoom);
                let circle = Circle::new(
                    pos,
                    self.bodies[i].radius * self.zoom);

                window.draw(&circle, Background::Col(Color::WHITE));
            }

            // if self.mouse_pressed && self.predict_speed != 0{ // draw prediction
            //     if self.predict_body.trail.len() > 2{
            //         let trail = graphics::Mesh::new_line(
            //             ctx,
            //             &self.predict_body.trail.as_slices().0,
            //             0.25 * self.predict_body.radius,
            //             graphics::Color::new(0.0, 1.0, 0.1, 0.4));

            //         match trail {
            //             Ok(line) => graphics::draw(ctx, &line, params).expect("error drawing trail"),
            //             Err(_error) => {},
            //         };
            //     }

            //     let body = graphics::Mesh::new_circle( //draw prediction body
            //         ctx,
            //         graphics::DrawMode::fill(),
            //         self.predict_body.pos,
            //         self.predict_body.radius,
            //         2.0,
            //         graphics::Color::new(0.0, 1.0, 0.0, 0.8)).expect("error building prediction body");

            //     graphics::draw(ctx, &body, params).expect("error drawing prediction body");
            // }

            if self.mouse_pos != self.start_point && self.mouse_pressed{ //draw preview vector
                let line = Line::new(
                    self.start_point,
                    self.mouse_pos);


                window.draw(&line, Background::Col(Color::WHITE));
            }

            let outline = Circle::new(
                if !self.mouse_pressed {self.mouse_pos} else {self.start_point},
                self.radius * self.zoom);

            window.draw(&outline, Background::Col(Color::WHITE.with_alpha(0.8)));

        Ok(())
    }
}

pub fn main(){
    run::<MainState>("N-body Gravity Sim", Vector::new(1000, 800), Settings {
        draw_rate: 1.0,  //draw as fast as possible basically
        update_rate: 1000. / 30., 
        vsync: true,
        ..Settings::default()
    });

    println!("test");
}

fn grid(start: &Point2, radius: &f32, density: &f32, zoom: &f32) -> Vec<Body> {
    //create a 10x10 grid of bodies
    let mut new_bodies: Vec<Body> = Vec::new();

    (1..=10).for_each(|y|{
        (1..=10).for_each(|x| {
            let point = Point2::new((x as f32 * radius * 50.0) - (start.x * (1.0/zoom)), (y as f32 * radius * 50.0) - (start.y * (1.0/zoom)));
            new_bodies.push(Body::new(
                    point,
                    radius.powi(3) * density,
                    *radius,
                    Vector2::new(0.0, 0.0)));
        });
    });

    new_bodies
}

fn scale(mut point: Point2, offset: &Point2, scale: &f32) -> Point2{
    point *= *scale;
    point.x -= offset.x * *scale;
    point.y -= offset.y * *scale;
    point
}

fn inv_scale(mut point: Point2, offset: &Point2, scale: &f32) -> Point2{
    point /= *scale;
    point.x += offset.x;
    point.y += offset.y;
    point
}

fn draw_line(points: &VecDeque<Point2>, offset: Point2, zoom: f32) -> Vec<Line>{
        let mut curve: Vec<Line> = Vec::new();


        (0..points.len() - 1).for_each(|i|{
            let first = scale(points[i], &offset, &zoom);
            let second = scale(points[i + 1], &offset, &zoom);
            curve.push(Line::new(first, second));
        });

        curve
}
