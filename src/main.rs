extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate vecmath;
extern crate crossbeam;

use graphics::Line;
use graphics::math;
use graphics::color::*;
use graphics::draw_state;
use graphics::grid::Grid;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::distributions::{Distribution, Uniform};

mod particle;
use particle::Particle;
mod gravity_grid;
use gravity_grid::GravityGrid;

pub struct World{
    gl: GlGraphics, // OpenGL drawing backend.
    draw_grid: Grid,
    pub width: f64, //
    pub height: f64, 
}


impl World{
    fn new(gl: GlGraphics, cell_size: f64, grid_width: u32, grid_height: u32, subdevision: u64) -> World{
        //cell size is the smallest cell containing 
        let draw_grid = Grid{
            cols: grid_height,
            rows: grid_width,
            units: cell_size,
        };

        let width = cell_size*(grid_width as f64);
        let height = cell_size*(grid_height as f64);

        World {gl: gl, draw_grid: draw_grid, width: width, height: height} //returns the world sturct
    }

    fn render(&mut self, arg: &RenderArgs){
        use graphics::*;

        let grid = self.draw_grid;
        let background: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, grid.units);

        //render things in the world
        self.gl.draw(arg.viewport(), |c, gl| {
            clear(background, gl);
        });
        
    }
}


fn main(){
    //Generate window
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("my window", [700, 700])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut world = World::new(GlGraphics::new(opengl), 10.0, 1000, 1000, 10);
    
    //distributions
    let x_range = Uniform::new(0.0, 700.0);
    let y_range = Uniform::new(0.0, 700.0);
    let mut rng = rand::thread_rng();

    //generate particles
    let mut particles: Vec<Particle> = Vec::new();
    let n_particles= 100;
    for n in 0..n_particles {
        let particle = Particle::new(x_range.sample(&mut rng),  y_range.sample(&mut rng), 1.0);
        particles.push(particle);
    }

    let mut GG1 = GravityGrid::new(9,9, 700.0/9.0);
    let mut GG2 = GravityGrid::new(27,27, 700.0/27.0);

    //event handles
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        //if render event do something
        if let Some(r_args) = e.render_args() {
            world.render(&r_args);
            for p in &mut particles{
                p.render(&r_args, &mut GlGraphics::new(opengl));
            } 
        }

        if let Some(u_args) = e.update_args(){
            //set mass to zero and then compute the mass per cell
            GG1.zero_mass();
            GG1.compute_mass(&mut particles);
            GG1.compute_force();
            GG1.apply_force(&mut particles);

            GG2.zero_mass();
            GG2.compute_mass(&mut particles);   
            GG2.compute_force();
            GG2.apply_force(&mut particles);

            for p in &mut particles{
                p.update(&u_args);
            }
        }
    }
}
