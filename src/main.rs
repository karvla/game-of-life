const WIDTH: usize = 50;
const HEIGHT: usize = 10;


#[derive(Copy, Clone)]
struct World{
    state: [[State; WIDTH]; HEIGHT],
}

#[derive(Copy, Clone)]
enum State { 
    Alive,
    Dead,
}

fn main() {
    let mut world = World {
        state: [[State::Dead; WIDTH]; HEIGHT]
    };

    world.state[1][1] = State::Alive;

    display_world(&world);
    world = update_world(world);
    display_world(&world);
}


fn display_world(world: &World)
{
    for x in world.state.iter()
    {
        for y in x.iter()
        {
            match y
            {
                State::Dead => print!(" "),
                State::Alive => print!("O"),
            }
        }
        println!();
    }
}

fn update_world(mut world: World) -> World
{
    let new_world = world;
    for (x_pos, x) in world.state.iter().enumerate()
    {
        for (y_pos, y) in x.iter().enumerate()
        {
            
        }
    }
    new_world
}

