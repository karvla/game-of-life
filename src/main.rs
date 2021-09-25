use std::{time, thread, env};

const HEIGHT: usize = 50;
const WIDTH: usize = 50;

#[derive(Copy, Clone)]
struct World {
    state: [[State; WIDTH]; HEIGHT],
}

struct Coord {
    x: i32,
    y: i32,
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

    // Blinker
    world.state[9][6] = State::Alive;
    world.state[9][7] = State::Alive;
    world.state[9][8] = State::Alive;

    // Spaceship
    world.state[3][8] = State::Alive;
    world.state[4][8] = State::Alive;
    world.state[5][8] = State::Alive;
    world.state[5][7] = State::Alive;
    world.state[4][6] = State::Alive;

    // Spaceship
    world.state[3][0] = State::Alive;
    world.state[4][0] = State::Alive;
    world.state[5][0] = State::Alive;
    world.state[5][1] = State::Alive;
    world.state[4][2] = State::Alive;

    // Spaceship
    world.state[3][19] = State::Alive;
    world.state[4][19] = State::Alive;
    world.state[5][19] = State::Alive;
    world.state[5][20] = State::Alive;
    world.state[4][21] = State::Alive;

    loop {
        world.display();
        world.update();
        thread::sleep(time::Duration::from_millis(50));
    }
}


impl World {
    fn update(&mut self) {
        let mut new_state = self.state;
        for (y_pos, column) in self.state.iter().enumerate() {
            for (x_pos, state) in column.iter().enumerate() {
                new_state[y_pos][x_pos] =
                    match (state, n_alive(&self, get_neighbors(y_pos, x_pos))) {
                        (State::Alive, 2 | 3) => State::Alive,
                        (State::Dead, 3) => State::Alive,
                        _ => State::Dead,
                    }
            }
        }
        self.state = new_state;
    }

    fn display(&self) {
        for row in self.state.iter() {
            for state in row.iter() {
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
            y: (y_pos as i32) + y,
            x: (x_pos as i32) + x,
        };

        if is_inside(&neighbor) {
            neighbors.push(neighbor);
        }
    }
    neighbors
}

fn is_inside(coord: &Coord) -> bool {
    coord.y >= 0 && (coord.y as usize) < HEIGHT && coord.x >= 0 && (coord.x as usize) < WIDTH
}

fn n_alive(world: &World, coords: Vec<Coord>) -> i32 {
    let mut n_alive = 0;
    for coord in coords {
        if world.state[coord.y as usize][coord.x as usize] == State::Alive {
            n_alive += 1;
        };
    }
    n_alive
}
