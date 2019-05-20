use specs::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Pos{
    pub x: f32,
    pub y: f32,
}

impl Component for Pos{
    type Storage = VecStorage<Self>;
}

impl Eq for Pos{}

#[derive(Clone)]
pub struct Movement{
    pub vel: (f32, f32),
    pub accel: (f32, f32),
    pub past_accel: (f32, f32),
}

impl Movement{
    pub fn new(x: f32, y: f32) -> Self{
        Movement{
            vel: (x, y),
            accel: (0.0, 0.0),
            past_accel: (0.0, 0.0),
        }
    }
}

impl Component for Movement{
    type Storage = VecStorage<Self>;
}
