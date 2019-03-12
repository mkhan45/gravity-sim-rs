extern crate ggez;
use ggez::*;
use ggez::graphics;
use ggez::nalgebra as na;
use std::f32::consts::PI;

struct MainState {
    bodies: Vec<Body>,
    screen_width: u32,
    screen_height: u32,
    current_rad: f32,
    mouse_down: bool,
    start_point: Point2,
}

const G: f32 = 6.674;

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let width = ctx.conf.window_mode.width as f32;
        let height = ctx.conf.window_mode.height as f32;
        let bodies = vec![
            Body::new(
                Point2::new(width/2.0, height/2.0),
                300.0,
                10.0,
                Vector2::new(0.0, 0.0)),

            Body::new(
                Point2::new(600.0, 400.0),
                1.0,
                5.0,
                Vector2::new(0.0, -2.5)),
        ];
        let s = MainState {
            bodies,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
            current_rad: 0.0,
            mouse_down: false,
            start_point: Point2::new(0.0, 0.0),
        };
        Ok(s)
    }

    fn update_velocities(&mut self){
        // for i in 0..self.bodies.len(){
        //     if(self.bodies[i].pos.y + self.bodies[i].radius * 2.0 <= self.screen_height as f32){
        //         self.bodies[i].velocity.y += 9.81 * 0.005;
        //     }else {
        //         self.bodies[i].velocity.y = 0.0;
        //     }
        // }
        
        for current_body_i in 0..self.bodies.len(){
            for other_body_i in 0..self.bodies.len(){
                if other_body_i != current_body_i {
                    let other_body = &self.bodies[other_body_i].clone();
                    let current_body = &mut self.bodies[current_body_i];

                    let r = distance(&other_body.pos, &current_body.pos);
                    let a_mag = (G*other_body.mass)/(r.powf(2.0)); //acceleration = Gm_2/r^2
                    let angle = angle(&other_body.pos, &current_body.pos);

                    self.bodies[current_body_i].velocity.x += angle.cos() * a_mag;
                    self.bodies[current_body_i].velocity.y += angle.sin() * a_mag;
                }
            }
        }
    }
}

fn distance(a: &Point2, b: &Point2) -> f32{
    ((b.x - a.x).powf(2.0) + (b.y-a.y).powf(2.0)).sqrt()
}

fn angle(a: &Point2, b: &Point2) -> f32{
    let mut restricted_dom = ((b.y - a.y)/(b.x - a.x)).atan();
    if b.x >= a.x{
        restricted_dom += PI;
    }

    restricted_dom
}

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

struct Body {
    pos: Point2,
    mass: f32,
    radius: f32,
    velocity: Vector2,
}

impl Body {
    fn new(position: Point2, mass_assign: f32, rad: f32, vel: Vector2) -> Body{
        Body {
            pos: position,
            mass: mass_assign,
            radius: rad,
            velocity: vel,
        }
    }

    fn update(&mut self){
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
    }

    fn clone(&self) -> Body{
        Body {
            pos: self.pos,
            mass: self.mass,
            radius: self.radius,
            velocity: self.velocity,
        }
    }
}




impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_velocities();
        for i in 0..self.bodies.len(){
            self.bodies[i].update();
        }

        if self.mouse_down {
            self.current_rad += 0.2;
        } 

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        

        for i in 0..self.bodies.len(){
            let body = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::Fill,
                self.bodies[i].pos,
                self.bodies[i].radius*2.0,
                2.0,
            )?;

            graphics::draw(ctx, &body, Point2::new(0.0, 0.0), 0.0);
        }

        graphics::present(ctx);
        if ggez::timer::get_ticks(ctx) % 60 == 0{
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {
        self.mouse_down = true;
        self.start_point = Point2::new(x as f32, y as f32);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {
        self.bodies.push(Body::new(
                Point2::new(x as f32, y as f32),
                self.current_rad * 3.0,
                self.current_rad,
                Vector2::new((x as f32 - self.start_point.x)/10.0, (y as f32 - self.start_point.y)/10.0)),
                );

        self.current_rad = 0.0;
        self.mouse_down = false;
    }

}

pub fn main() {
    let windowsetup = ggez::conf::WindowSetup{
        title: "N-body Gravity Simulator".to_owned(),
        icon: "".to_owned(),
        resizable: false,
        allow_highdpi: true,
        samples: ggez::conf::NumSamples::One,
    };

    let mut c = conf::Conf::new();
    c.window_mode.vsync(true);
    c.window_setup = windowsetup;

    let ctx = &mut Context::load_from_conf("Nbody Sim", "Fish", c).unwrap();
    ggez::graphics::set_background_color(ctx, ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0));

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
