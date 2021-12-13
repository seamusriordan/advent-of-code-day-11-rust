use std::str::Lines;

mod tests;

struct Octopus {
    energy: u64,
    flashing: bool
}

pub struct Grid {
    grid: Vec<Vec<Octopus>>,
    pub flashes: u64
}

impl Grid {
    pub fn new(input: Lines) -> Grid {
        let mut grid = Grid {
            grid: vec![],
            flashes: 0
        };
        for line in input {
            let mut octopuses = vec![];
            for c in line.chars() {
                let octopus = Octopus {
                    flashing: false,
                    energy: c.to_string().parse().unwrap()
                };
                octopuses.push(octopus)
            }
            grid.grid.push(octopuses)
        }

        grid
    }

    pub fn iterate(&mut self) {
        self.flashes = 0;

        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                self.grid[i][j].energy += 1
            }
        }


        loop {
            let mut new_flashing = 0;
            for i in 0..self.grid.len() {
                for j in 0..self.grid[i].len() {
                    new_flashing += self.start_flashing(i as i32,j as i32)
                }
            }


            if new_flashing == 0 {
                break;
            }
        }


        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j].flashing {
                    self.grid[i][j].flashing = false;
                    self.grid[i][j].energy = 0;
                }
            }
        }

    }

    fn start_flashing(&mut self, x: i32, y: i32) -> usize {
        let mut new_flashing = 0;


        if self.grid[x as usize][y as usize].energy > 9 && !self.grid[x as usize][y as usize].flashing {
            self.grid[x as usize][y as usize].flashing = true;
            new_flashing += 1;
            self.flashes += 1;

            for i in x-1..x+2 {
                if i >= 0 && i < self.grid.len() as i32 {
                    for j in y-1..y+2 {
                        if j >= 0 && j < self.grid[i as usize].len() as i32 {
                            if !(i == x && j == y) {
                                self.grid[i as usize][j as usize].energy += 1;
                            }
                        }
                    }
                }
            }
        }

        new_flashing
    }

    pub fn print(&self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                print!("{}", self.grid[i][j].energy);
            }
            println!();
        }
        println!();
    }
}