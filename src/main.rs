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
        let bodies = vec![new_body(150f32, 50f32, 50f32, 50f32)];
        let s = MainState {
            bodies,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
        };
        Ok(s)
    }
}

type Point2 = na::Point2<f32>;

struct Body {
    pos: Point2,
    mass: f32,
    radius: f32,
}

fn new_body(x_pos: f32, y_pos: f32, mass_assign: f32, rad: f32) -> Body{
    Body {
        pos: Point2::new(x_pos, y_pos),
        mass: mass_assign,
        radius: rad,
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let body = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::Fill,
            self.bodies[0].pos,
            self.bodies[0].radius,
            2.0,
        )?;
        graphics::draw(ctx, &body, Point2::new(0.0, 0.0), 0.0);
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
