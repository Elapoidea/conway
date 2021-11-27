use std::ops::{Index, IndexMut};
use std::fmt::{Display, Result, Formatter};
use std::iter::{Iterator};
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn swap(&self) -> Self{
        match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }

    fn conway(&self, neighbors: usize) -> Self {
        match self {
            Cell::Alive => { 
                match neighbors {
                    2 | 3 => Cell::Alive,
                    _ => Cell::Dead,
                }
            },
            Cell::Dead => { 
                match neighbors {
                    3 => Cell::Alive,
                    _ => Cell::Dead,
                }
             },
        }
    }

    fn caves(&self, neighbors: usize) -> Self {
        match self {
            Cell::Alive => { 
                match neighbors {
                    3 | 4 | 5 | 6 | 7 | 8 => Cell::Alive,
                    _ => Cell::Dead,
                }
            },
            Cell::Dead => { 
                match neighbors {
                    6 | 7 | 8 => Cell::Alive,
                    _ => Cell::Dead,
                }
             },
        }
    }

    fn smooth_caves(&self, neighbors: usize) -> Self {
        match self {
            Cell::Alive => { 
                match neighbors {
                    4 | 5 | 6 | 7 | 8 => Cell::Alive,
                    _ => Cell::Dead,
                }
            },
            Cell::Dead => { 
                match neighbors {
                    5 | 6 | 7 | 8 => Cell::Alive,
                    _ => Cell::Dead,
                }
             },
        }
    }

    fn smooth(&self, neighbors: usize) -> Self {
        match self {
            Cell::Alive => { 
                match neighbors {
                    5 | 6 | 7 | 8 => Cell::Alive,
                    _ => Cell::Dead,
                }
            },
            Cell::Dead => { 
                match neighbors {
                    5 | 6 | 7 | 8 => Cell::Alive,
                    _ => Cell::Dead,
                }
             },
        }       
    }

    fn lines(&self, neighbors: usize) -> Self {
        match self {
            Cell::Alive => { 
                match neighbors {
                    5 | 6 | 7 | 8 => Cell::Alive,
                    _ => Cell::Dead,
                }
            },
            Cell::Dead => { 
                match neighbors {
                    4 | 5 | 6 | 7 | 8 => Cell::Alive,
                    _ => Cell::Dead,
                }
             },
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Cell::{}", {
            match &self {
                Cell::Dead => "Dead",
                Cell::Alive => "Alive",
            }
        })
    }
}

#[derive(Clone)]
pub struct World {
    pub x: u32,
    pub y: u32,
    world: Vec<Vec<Cell>>,
    i: usize,
}

impl World {
    pub fn new(x: u32, y: u32) -> Self {
        World {
            x,
            y,
            world: vec![vec![Cell::Dead; y as usize]; x as usize],
            i: 0,
        }
    }

    pub fn rand(&mut self, dist: u32) {
        let mut rng = rand::thread_rng();

        for i in 1..self.x {
            for j in 1..self.y {
                self.world[i as usize][j as usize] = if rng.gen_range(0, 100) <= dist {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }
        }
    }

    pub fn invert(&self) -> Self {
        let mut new_world: World = World::new(self.x, self.y);

        for (x, row) in self.clone().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                new_world[x][y] = cell.swap();
            }
        }

        new_world
    }

    pub fn next(&mut self, rules: &str) {
        let mut new_world: World = World::new(self.x, self.y);
        let neighborhood: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1), 
            ( 0, -1),          ( 0, 1), 
            ( 1, -1), ( 1, 0), ( 1, 1)
        ];

        for (x, row) in self.clone().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                let mut density: usize = 0;

                for i in neighborhood {
                    let rel_x: i32 = x as i32 + i.0;
                    let rel_y: i32 = y as i32 + i.1;

                    if self.in_world(rel_x, rel_y) {
                        density += match self[rel_x as usize][rel_y as usize] {
                            Cell::Alive => 1,
                            Cell::Dead => 0,
                        }
                    }
                }

                new_world[x][y] = match rules {
                    "caves" => cell.caves(density),
                    "smooth caves" => cell.smooth_caves(density),
                    "conway" => cell.conway(density),
                    "smooth" => cell.smooth(density),
                    "lines" => cell.lines(density),
                    _ => Cell::Dead,
                }
            }
        }

        self.world = new_world.world;
    }

    pub fn grow(&mut self) {
        let mut new_world: World = World::new(self.x * 2, self.y * 2);

        for (x, row) in self.clone().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                new_world[x * 2    ][y * 2] = *cell;
                new_world[x * 2 + 1][y * 2] = *cell;
                new_world[x * 2    ][y * 2 + 1] = *cell;
                new_world[x * 2 + 1][y * 2 + 1] = *cell;
            }
        }

        *self = new_world;
    }

    fn in_world(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.x as i32 && y < self.y as i32
    }
}

impl Index<usize> for World {
    type Output = Vec<Cell>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.world[i]
    }
}

impl IndexMut<usize> for World {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.world[i]
    }
}

impl Iterator for World {
    type Item = Vec<Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        
        if self.i <= self.x as usize {
            Some((&self.world[self.i - 1]).to_vec())
        } else {
            None
        }
    }
}