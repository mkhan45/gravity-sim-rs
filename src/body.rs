use ggez::nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;



#[derive(Clone)]
pub struct Body {
    pub pos: Point2,
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vector2,
    pub trail: Vec<Point2>,
    pub trail_length: usize,
}

impl Body {
    pub fn new(position: Point2, mass_assign: f32, rad: f32, vel: Vector2) -> Body{
        Body {
            pos: position,
            mass: mass_assign,
            radius: rad,
            velocity: vel,
            trail: vec![Point2::new(position.x + rad/2.0, position.y + rad/2.0)], //ggez doesn't like it when all the points are the same
            trail_length: 120,
        }
    }

    pub fn update(&mut self){
        self.trail.push(self.pos);
        
        if self.trail.len() > self.trail_length {
            self.trail = self.trail.split_off(self.trail.len() - self.trail_length);
        }

        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;

    }
}
