use specs::prelude::*;

#[derive(Debug)]
pub struct Pos{
    pub x: f32,
    pub y: f32,
}

impl Component for Pos{
    type Storage = DenseVecStorage<Self>;
}

pub struct Vel{
    pub x: f32,
    pub y: f32,
}

impl Component for Vel{
    type Storage = DenseVecStorage<Self>;
}
