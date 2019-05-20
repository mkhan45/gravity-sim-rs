use specs::prelude::*;

use ggez::mint::Point2;

type Point = Point2<f32>;

#[derive(Clone)]
pub struct Mass(pub f32);

impl Component for Mass{
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct Radius(pub f32);

impl Component for Radius{
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct Charge(pub f32);

impl Component for Charge{
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct Trail{
    pub points: Vec<Point>,
    pub length: u32,
}

impl Trail{
    pub fn new(length: u32) -> Self{
        Trail{
            points: Vec::new(),
            length,
        }
    }
}

impl Component for Trail{
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct PreviewFlag;

impl Component for PreviewFlag{
    type Storage = NullStorage<Self>;
}
