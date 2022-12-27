use minifb::{Key, Window, WindowOptions};
use rand::*;
const HEIGHT: usize = 360;
const WIDTH: usize = 640;

#[derive(Copy, Clone)]
pub struct Cell {
    pub alive: bool,
    pub neighbor_count: u8,
}

pub struct World {
    pub height: u32,
    pub width: u32,
    pub grid: Vec<Vec<Cell>>,
    pub grid_buffer: Vec<Vec<u8>>,
}

pub struct Simulation {
    pub name: String,
    pub world: World,
    pub population: u32,
}

impl Simulation {
    // Creates a new simulation with a fully dead world
    pub fn new(name: String, width: u32, height: u32) -> Self {
        let mut sim = Simulation {
            name: name,
            world: World {
                height: height,
                width: width,
                grid: vec![
                    vec![
                        Cell {
                            alive: false,
                            neighbor_count: 0
                        };
                        height as usize
                    ];
                    width as usize
                ],
                grid_buffer: vec![vec![0; height as usize]; width as usize],
            },
            population: 0,
        };
        sim.randomize_world();
        return sim;
    }

    pub fn randomize_world(&mut self) {
        self.clear();
        let mut rng = rand::thread_rng();
        for x in 0..self.world.width {
            for y in 0..self.world.height {
                if rng.gen_range(0..=1) == 0 {
                    self.flip_cell(x as usize, y as usize);
                };
            }
        }
        let buffer = &mut self.world.grid_buffer;
        for (buffer_row, world_row) in buffer.iter().zip(self.world.grid.iter_mut()) {
            for (neighbor_count, cell) in buffer_row.iter().zip(world_row.iter_mut()) {
                cell.neighbor_count = *neighbor_count;
            }
        }
    }


    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_population(&self) -> u32 {
        return self.population;
    }

    // Kills the world
    pub fn clear(&mut self) {
        for x in 0..self.world.width {
            for y in 0..self.world.height {
                self.world.grid[x as usize][y as usize] = Cell {
                    alive: false,
                    neighbor_count: 0,
                }
            }
        }
        self.population = 0;
        self.world.grid_buffer.iter_mut().for_each(|row|row.fill(0));
    }

    pub fn get_world(&self) -> &World {
        return &self.world;
    }

    pub fn flip_cell(&mut self, x: usize, y: usize) {
        let buffer = &mut self.world.grid_buffer;
        let cell = &mut self.world.grid[x][y];

        cell.alive = !cell.alive;
        // Updating neighbor counts in buffer
        for neighbor_x in x.saturating_sub(1)..=(x + 1).min(self.world.width as usize - 1) {
            for neighbor_y in y.saturating_sub(1)..=(y + 1).min(self.world.height as usize - 1) {
                if (neighbor_x != x) && (neighbor_y != y) {
                    if cell.alive {
                        buffer[neighbor_x][neighbor_y] += 1
                    } else {
                        buffer[neighbor_x][neighbor_y] -= 1
                    }
                }
            }
        }
        if cell.alive {
            self.population += 1
        } else {
            self.population -= 1
        }
    }

    pub fn step(&mut self) {
        // The rules of Conway's game of life:

        for x in 0..self.world.width as usize {
            for y in 0..self.world.height as usize {
                let cell = &mut self.world.grid[x][y];
                // 1. A live cell dies if it has fewer than 2 alive neighbors.
                // 3. A live cell with more than three live neighbors dies.
                if cell.alive && (cell.neighbor_count > 3 || cell.neighbor_count < 2) {
                    self.flip_cell(x, y);
                }
                // 4. A dead cell comes alive if it has exactly three live neighbors.
                else if !cell.alive && cell.neighbor_count == 3 {
                    self.flip_cell(x, y);
                }

                // 2. A live cell with two or three live neighbors lives on to the next gen.
            }
        }
        // Apply the changes in the buffer to the real world before finishing the update step
        let buffer = &mut self.world.grid_buffer;
        for (buffer_row, world_row) in buffer.iter().zip(self.world.grid.iter_mut()) {
            for (neighbor_count, cell) in buffer_row.iter().zip(world_row.iter_mut()) {
                cell.neighbor_count = *neighbor_count;
            }
        }
    }
}
