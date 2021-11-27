extern crate graphics;
extern crate piston;
extern crate piston_window;

use piston::window::WindowSettings;
use piston_window::*;
use lib::*;
use std::{thread, time};

const WINDOW_X: u32 = 1000;
const WINDOW_Y: u32 = 1000;
const SCALE: usize = 1;
const WORLD_X: u32 = 125;
const WORLD_Y: u32 = 125;
const SPEED: u64 = 10;
const STARTING_DENSITY: u32 = 50;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Conway",
        [WINDOW_X, WINDOW_Y])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut world = World::new(WORLD_X, WORLD_Y);
    world.rand(STARTING_DENSITY);

    let mut iters = 0;
    
    while let Some(e) = window.next() {
        window.draw_2d(&e, |b, g, _| {
            clear([0.1, 0.1, 0.1, 0.1], g);
            
            for (x, row) in world.clone().invert().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    let colour = match cell {
                        Cell::Alive => [1.0, 1.0, 1.0, 1.0],
                        Cell::Dead => [0.1, 0.1, 0.1, 1.0],
                    };

                    rectangle(
                        colour,
                        rectangle::square((x * SCALE) as f64, (y * SCALE) as f64, SCALE as f64),
                        b.transform,
                        g
                    );
                }
            }

            if iters < 3  {
                world.next("caves");
                world.next("smooth caves");
            } else if iters < 6 {
                world.next("smooth");
                world.grow();
            } else {
                
            }
            
            iters += 1;

            thread::sleep(time::Duration::from_millis(1000 / SPEED));
        });
    }
    
}