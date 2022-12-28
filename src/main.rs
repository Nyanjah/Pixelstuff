use minifb::*;

mod conway;
use conway::*;

const WIDTH:usize = 200;
const HEIGHT:usize = 120;
const SECOND_PER_FRAME:f32 = 0.1;
fn main(){
    // Setting up the window
    let mut window = Window::new(
        "Conway's Game of Life - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: true,
            title: true,
            resize: true,
            scale: Scale::X1,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
   
    // Limit to update rate to retain  ~1 sec / frame
    window.limit_update_rate(Some(std::time::Duration::from_secs_f32(SECOND_PER_FRAME)));

    let mut paused:bool = false;
    let mut simulation = Simulation::new(String::from("Game of Life"),WIDTH as u32, HEIGHT as u32);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // If the pause button is pressed, toggle the paused flag.
        if window.is_key_pressed(Key::X,KeyRepeat::No){
            // If the simulation is about to be unpaused
            if paused {
                window.limit_update_rate(Some(std::time::Duration::from_secs_f32(SECOND_PER_FRAME)));
                window.set_cursor_style(CursorStyle::Arrow);
            }
            // If the simulation is being pasued
            else {
                window.limit_update_rate(Some(std::time::Duration::from_secs_f32(0.0)));
                window.set_cursor_style(CursorStyle::Crosshair);
            }
            paused = !paused;
        }
        // If the clear button is pressed, kill all cells
        if window.is_key_released(Key::C){
            simulation.clear();
        }
        if !paused{
            // Step the simulation
            simulation.step();
        }
        // If the randomize button is pressed, randomize all cell life
        if window.is_key_pressed(Key::R,KeyRepeat::No){
            simulation.randomize_world();
        }

        // Modify the game world based on mouse input
        // Fetch the mouse position as (f32,f32) where (0,0) is the top-left most corner.
        let (mut mouse_x,mut mouse_y) = window.get_mouse_pos(MouseMode::Discard).unwrap_or((0.0,0.0)); 
        // Fetch the window size
        let (window_x,window_y) = window.get_size();
        mouse_x = mouse_x / (window_x as f32 / WIDTH as f32) - 0.5;
        mouse_y = mouse_y / (window_y as f32 / HEIGHT as f32) - 0.5; 

        // If the left mouse button is down, create cells
        if window.get_mouse_down(MouseButton::Left) && paused{
            simulation.insert_life(mouse_x.round() as usize,mouse_y.round() as usize);
        }
        // If the right mouse button is down, erase cells
        if window.get_mouse_down(MouseButton::Right) && paused{
            simulation.delete_life(mouse_x.round() as usize,mouse_y.round() as usize);
        }
    
        
        // Fetch the state of the simulation's world
        let world_state = simulation.get_world();
        // Map the simulations world to the pixel buffer
        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                let cell_is_alive = world_state.grid[x][y].alive;
                let count = world_state.grid[x][y].neighbor_count;
                buffer.push({
                     if cell_is_alive && count == 2 {(255 as u32) << 8}
                     else if cell_is_alive && count == 3 {(255 as u32) << 8 | (125 as u32) | (125 as u32) <<16}
                     else if cell_is_alive {(255 as u32) << 8 | (190 as u32) | (190 as u32) <<16}
                     else{0}
                });
            }
        }
        // Update the window using the pixel buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        // flush the buffer
        buffer.clear();
        //println!("{}",simulation.population);
    }



}



