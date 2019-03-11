extern crate ggez;
use ggez::*;
use ggez::graphics;
use ggez::nalgebra as na;

struct MainState {
    bodies: Vec<Body>,
    screen_width: u32,
    screen_height: u32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let bodies = vec![
            Body::new(
                Point2::new(50.0, 200.0),
                5.0,
                10.0,
                Vector2::new(0.0, 0.0)),

            Body::new(
                Point2::new(200.0, 300.0),
                5.0,
                10.0,
                Vector2::new(0.0, -1.0)),
        ];
        let s = MainState {
            bodies,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
        };
        Ok(s)
    }

    fn update_velocities(&mut self){
        for i in 0..self.bodies.len(){
            if(self.bodies[i].pos.y + self.bodies[i].radius * 2.0 <= self.screen_height as f32){
                self.bodies[i].velocity.y += 9.81 * 0.005;
            }else {
                self.bodies[i].velocity.y = 0.0;
            }
        }
    }
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
}



impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_velocities();
        for i in 0..self.bodies.len(){
            self.bodies[i].update();
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
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    c.window_mode.vsync(true);
    let ctx = &mut Context::load_from_conf("Nbody Sim", "Fish", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
