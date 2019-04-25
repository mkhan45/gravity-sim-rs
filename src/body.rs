use ggez::nalgebra as na;
use std::collections::VecDeque;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;


#[derive(Clone)]
pub struct Body {
    pub pos: Point2,
    pub mass: f32,
    pub charge: f32,
    pub radius: f32,
    pub velocity: Vector2,
    pub trail: VecDeque<Point2>,
    pub trail_length: usize,
    pub past_accel: Vector2,
    pub current_accel: Vector2,
    pub collision: Option<usize>,
}

impl Body {
    pub fn new(position: Point2, mass_assign: f32, charge_assign: f32, rad: f32, vel: Vector2) -> Body{
        let mut trail_vec = VecDeque::new();
        trail_vec.push_back(Point2::new(position.x + rad/2.0, position.y + rad/2.0));
        trail_vec.push_back(Point2::new(position.x, position.y));

        Body {
            pos: position,
            mass: mass_assign,
            charge: charge_assign,
            radius: rad,
            velocity: vel,
            trail: trail_vec,
            trail_length: 120,
            past_accel: Vector2::new(0.0, 0.0),
            current_accel: Vector2::new(0.0, 0.0),
            collision: None,
        }
    }

    pub fn update_trail(&mut self){
        self.trail.push_back(self.pos);

        if self.trail.len() > self.trail_length {
            for _i in 0..(self.trail.len() - self.trail_length - 1) { //pop all points over trail length limit
                self.trail.pop_front();
            }
        }

    }

    pub fn update_euler(&mut self, step_size: &f32){
        self.pos += Vector2::new(self.velocity.x * step_size, self.velocity.y * step_size);
        self.velocity += self.current_accel * step_size.powi(2);
    }

    pub fn update_verlet(&mut self, step_size: &f32){ //verlet velocity
        self.velocity += ((self.current_accel + self.past_accel)/2.0) * *step_size;
        self.pos += self.velocity * *step_size + (self.current_accel/2.0) * (*step_size).powi(2);
        self.past_accel = self.current_accel;
    }
}
