use specs::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Pos{
    pub x: f32,
    pub y: f32,
}

impl Component for Pos{
    type Storage = DenseVecStorage<Self>;
}

impl Eq for Pos{}

#[derive(Clone)]
pub struct Vel{
    pub x: f32,
    pub y: f32,
}

impl Component for Vel{
    type Storage = DenseVecStorage<Self>;
}
