extern crate ggez;
use ggez::*; use ggez::graphics; use ggez::nalgebra as na;
use ggez::input;

use crate::body::*;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

pub fn grid(start: &Point2, radius: &f32, density: &f32, zoom: &f32) -> Vec<Body> {
    //create a 10x10 grid of bodies
    let mut new_bodies: Vec<Body> = Vec::new();

    (1..=10).for_each(|y|{
        (1..=10).for_each(|x| {
            let point = Point2::new((x as f32 * radius * 50.0) - (start.x * (1.0/zoom)), (y as f32 * radius * 50.0) - (start.y * (1.0/zoom)));
            new_bodies.push(Body::new(
                    point,
                    radius.powi(3) * density,
                    *radius,
                    Vector2::new(0.0, 0.0)));
        });
    });

    new_bodies
}


pub fn nested_orbit(start: &Point2, radius: &f32, density: &f32, zoom: &f32) -> Vec<Body> {
    vec![
        Body::new(
            Point2::new(500.0, 400.0),
            30_000_000.0,
            1000.0,
            Vector2::new(0.0, 0.0)),

        Body::new(
            Point2::new(150_000.0, 400.0),
            150_000.0,
            100.0,
            Vector2::new(0.0, -40.0)),
    ]
}
