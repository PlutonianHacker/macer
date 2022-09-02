#![allow(dead_code)]

use rusty_ecs::*;

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    let mut world = World::new();

    world.spawn((12, "Hello", Velocity { x: 10.0, y: 5.0 }));
}
