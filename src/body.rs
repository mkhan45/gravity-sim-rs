use ggez::nalgebra as na;
use std::collections::VecDeque;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use ggez::graphics;

#[derive(Clone)]
pub struct Body {
    pub pos: Point2,
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vector2,
    pub trail: VecDeque<Point2>,
    pub trail_length: usize,
    pub past_accel: Vector2,
    pub current_accel: Vector2,
}

impl Body {
    pub fn new(position: Point2, mass_assign: f32, rad: f32, vel: Vector2) -> Body{
        let mut trail_vec = VecDeque::new();
        trail_vec.push_back(Point2::new(position.x + rad/2.0, position.y + rad/2.0));
        trail_vec.push_back(Point2::new(position.x, position.y));
        Body {
            pos: position,
            mass: mass_assign,
            radius: rad,
            velocity: vel,
            trail: trail_vec,
            trail_length: 120,
            past_accel: Vector2::new(0.0, 0.0),
            current_accel: Vector2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self){
        microprofile::scope!("Update", "Bodies");

        self.trail.push_back(self.pos);
               
        if self.trail.len() > self.trail_length {
            for _i in 0..(self.trail.len() - self.trail_length - 1) {
                self.trail.pop_front();
            }
        }
        

        self.pos.x += self.velocity.x + (0.5 * self.current_accel.x);
        self.pos.y += self.velocity.y + (0.5 * self.current_accel.y);
        
        self.velocity.x += (self.past_accel.x + self.current_accel.x)/2.0;

        self.velocity.y += (self.past_accel.y + self.current_accel.y)/2.0;

        self.past_accel = self.current_accel;
    }

    pub fn render(&self) -> graphics::MeshBuilder{
        graphics::MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(),
                self.pos,
                self.radius,
                2.5,
                graphics::Color::new(1.0, 1.0, 1.0, 1.0))
            .clone() 
    }
}
