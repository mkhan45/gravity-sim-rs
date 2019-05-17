use specs::prelude::*;

use ggez::{
    nalgebra as na, input, GameResult, Context, graphics,
    graphics::{DrawParam},
    event::{EventHandler},
};

use crate::components::*;

pub struct MainState<'a, 'b>{
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b>{
    pub fn new(world: World, dispatcher: Dispatcher<'a, 'b>) -> Self{
        MainState{
            world,
            dispatcher,
        }
    }
}

impl<'a, 'b> EventHandler for MainState<'a, 'b>{
    fn update(&mut self, ctx: &mut Context) -> GameResult{
        self.world.maintain();
        self.dispatcher.dispatch(&mut self.world.res);

        if ggez::timer::ticks(ctx) % 60 == 0{
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult{
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let positions = self.world.read_storage::<Pos>();
        let radii = self.world.read_storage::<Radius>();

        for (position, radius) in (&positions, &radii).join(){
            let outline = graphics::Mesh::new_circle( //draw outline
                ctx,
                graphics::DrawMode::fill(),
                [position.x, position.y],
                radius.0,
                0.25,
                graphics::Color::new(1.0, 1.0, 1.0, 1.0))
                .expect("error building outline");

            graphics::draw(ctx, &outline, DrawParam::new()).expect("error drawing outline");
        }

        graphics::present(ctx).expect("error rendering");
        Ok(())
    }
}
