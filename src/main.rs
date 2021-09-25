use std::fs::File;
use std::io;
use std::io::Read;
use std::{thread, time};
use terminal_size::terminal_size;


const HEIGHT: usize = 500;
const WIDTH: usize = 500;

#[derive(Copy, Clone)]
struct World {
    state: [[State; WIDTH]; HEIGHT],
}

struct Coord {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum State {
    Alive,
    Dead,
}

fn main() {
    let mut world = World {
        state: [[State::Dead; WIDTH]; HEIGHT],
    };

    world.set_state("map.txt");

    loop {
        world.display();
        world.update();
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn read_string(filename: &str) -> Result<String, io::Error> {
    let mut string = String::new();

    File::open(filename)?.read_to_string(&mut string)?;

    Ok(string)
}

impl World {
    fn set_state(&mut self, filename: &str) {
        let string = match read_string(filename) {
            Ok(s) => s,
            Err(_) => "".to_string(),
        };

        let mut x = 0;
        let mut y = 0;
        for c in string.chars() {
            match c {
                '\n' => {
                    y += 1;
                    x = 0;
                    continue;
                }
                ' ' => self.state[y][x] = State::Dead,
                _ => self.state[y][x] = State::Alive,
            }
            x += 1;

            if x >= HEIGHT || y >= WIDTH {
                break;
            }
        }
    }

    fn update(&mut self) {
        let mut new_state = self.state;
        for (y_pos, column) in self.state.iter().enumerate() {
            for (x_pos, state) in column.iter().enumerate() {
                new_state[y_pos][x_pos] = match (state, n_alive(&self, get_neighbors(y_pos, x_pos)))
                {
                    (State::Alive, 2 | 3) => State::Alive,
                    (State::Dead, 3) => State::Alive,
                    _ => State::Dead,
                }
            }
        }
        self.state = new_state;
    }

    fn display(&self) {
        let (width, height) = match terminal_size() {
            Some((w, h)) => (w.0 as usize, h.0 as usize),
            None => (80, 50),
        };

        for (y, row) in self.state.iter().enumerate() {
            if y >= height {
                println!();
                break;
            }
            for (x, state) in row.iter().enumerate() {
                if x >= width {
                    break;
                }
                match state {
                    State::Dead => print!("⠀"),
                    State::Alive => print!("█"),
                }
            }
            println!();
        }
    }
}

fn get_neighbors(y_pos: usize, x_pos: usize) -> Vec<Coord> {
    let mut neighbors = Vec::new();
    for (y, x) in [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ] {
        let neighbor = Coord {
            y: ((y_pos as i32) + y).rem_euclid(HEIGHT as i32) as usize,
            x: ((x_pos as i32) + x).rem_euclid(WIDTH as i32) as usize,
        };
        neighbors.push(neighbor);
    }
    neighbors
}

fn n_alive(world: &World, coords: Vec<Coord>) -> usize {
    let mut n_alive = 0;
    for coord in coords {
        if world.state[coord.y as usize][coord.x as usize] == State::Alive {
            n_alive += 1;
        };
    }
    n_alive
}
