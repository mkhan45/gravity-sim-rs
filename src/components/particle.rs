use specs::prelude::*;

pub struct Mass(pub f32);

impl Component for Mass{
    type Storage = DenseVecStorage<Self>;
}

