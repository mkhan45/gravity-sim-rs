use ggez::nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;



#[derive(Clone)]
pub struct Body {
    pub pos: Point2,
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vector2,
}

impl Body {
    pub fn new(position: Point2, mass_assign: f32, rad: f32, vel: Vector2) -> Body{
        Body {
            pos: position,
            mass: mass_assign,
            radius: rad,
            velocity: vel,
        }
    }

    pub fn update(&mut self){
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
    }
}
