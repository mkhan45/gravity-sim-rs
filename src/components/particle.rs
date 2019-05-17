use specs::prelude::*;

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
