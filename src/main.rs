extern crate ggez;
use ggez::*;
use ggez::graphics;
use ggez::nalgebra as na;
use std::f32::consts::PI;
use std::collections::HashSet;

struct MainState {
    bodies: Vec<Body>,
    screen_width: u32,
    screen_height: u32,
    current_rad: f32,
    mouse_down: bool,
    start_point: Point2,
    zoom: f32,
}

const G: f32 = 6.674;

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let width = ctx.conf.window_mode.width as f32;
        let height = ctx.conf.window_mode.height as f32;
        let bodies = vec![
            Body::new(
                Point2::new(width/2.0, height/2.0),
                5000.0,
                30.0,
                Vector2::new(0.0, 0.0)),

            Body::new(
                Point2::new(width/2.0 + 350.0, height/2.0),
                1.0,
                5.0,
                Vector2::new(-3.0, -6.5)),
        ];
        let s = MainState {
            bodies,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
            current_rad: 0.0,
            mouse_down: false,
            start_point: Point2::new(0.0, 0.0),
            zoom: 1.0,
        };
        Ok(s)
    }

    fn update_velocities_and_collide(&mut self){
        let mut collision_blacklist = HashSet::new();
        let mut collision_bodies = Vec::new();

        for current_body_i in 0..self.bodies.len(){
            for other_body_i in 0..self.bodies.len(){
                if other_body_i != current_body_i {
                    let other_body = &self.bodies[other_body_i].clone();
                    let current_body = &mut self.bodies[current_body_i];

                    let r = distance(&other_body.pos, &current_body.pos);
                    let a_mag = (G*other_body.mass)/(r.powf(2.0)); //acceleration = Gm_2/r^2
                    let angle = angle(&other_body.pos, &current_body.pos);
                    
                    if r <= other_body.radius + current_body.radius && !collision_blacklist.contains(&current_body_i){
                        println!("Collision");
                        collision_blacklist.insert(current_body_i);
                        collision_blacklist.insert(other_body_i);
                        collision_bodies.push(collide(&current_body, &other_body));
                    }

                    self.bodies[current_body_i].velocity.x += angle.cos() * a_mag;
                    self.bodies[current_body_i].velocity.y += angle.sin() * a_mag;
                }
            }
        }

        self.bodies = self.bodies.iter()
            .enumerate()
            .filter_map(|(index, body)| {
                if collision_blacklist.contains(&index) {
                    None
                } else {
                    Some(body.clone())
                }
            }).collect();
        
        self.bodies.append(&mut collision_bodies);
    }
}

fn collide(body1: &Body, body2: &Body) -> Body{
    let body1_momentum = Point2::new(body1.velocity.x, body1.velocity.y);
    let body2_momentum = Point2::new(body2.velocity.x, body2.velocity.y);

    let body1_momentum = Point2::new(body1_momentum.x * body1.mass, body1_momentum.y * body1.mass);
    let body2_momentum = Point2::new(body2_momentum.x * body2.mass, body2_momentum.y * body2.mass);

    let total_momentum = Vector2::new(body1_momentum.x + body2_momentum.x, body1_momentum.y + body2_momentum.y);

    let total_mass = body1.mass + body2.mass;

    Body::new(
        if body1.radius > body2.radius {Point2::new(body1.pos.x, body1.pos.y)} else {Point2::new(body2.pos.x, body2.pos.y)},
        body1.mass + body2.mass,
        body1.radius + body2.radius,
        Vector2::new(total_momentum.x/total_mass, total_momentum.y/total_mass),
    )
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

#[derive(Clone)]
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
    
}




impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_velocities_and_collide();
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
                Point2::new(self.bodies[i].pos.x * &self.zoom, self.bodies[i].pos.y * &self.zoom),
                self.bodies[i].radius * &self.zoom,
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
        self.start_point = Point2::new(x as f32 * (1.0/&self.zoom), y as f32 * (1.0/&self.zoom));
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {
        let zoomed_x = x as f32 * (1.0/&self.zoom);
        let zoomed_y = y as f32 * (1.0/&self.zoom);

        self.bodies.push(Body::new(
                Point2::new(zoomed_x as f32, zoomed_y as f32),
                self.current_rad.powf(2.0) * 1.0 * (1.0/&self.zoom),
                self.current_rad * (1.0/&self.zoom),
                Vector2::new((zoomed_x as f32 - self.start_point.x)/10.0, (zoomed_y as f32 - self.start_point.y)/10.0 ),
                ));

        println!("Mass: {}", self.current_rad * 1.0 * (1.0/&self.zoom));

        self.current_rad = 0.0;
        self.mouse_down = false;
    }


    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: i32, _y: i32) {
        println!("Zoom: {}", self.zoom);
        self.zoom *= 1.0 + (_y as f32 * 0.1); 
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

    let windowmode = ggez::conf::WindowMode{
        width: 1000,
        height: 800,
        borderless: true,
        fullscreen_type: ggez::conf::FullscreenType::Off,
        vsync:true,
        min_width: 0,
        max_width: 0,
        min_height: 0,
        max_height: 0,
    };

    let mut c = conf::Conf::new();
    c.window_mode = windowmode;
    c.window_setup = windowsetup;

    let ctx = &mut Context::load_from_conf("Nbody Sim", "Fish", c).unwrap();
    ggez::graphics::set_background_color(ctx, ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0));

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
