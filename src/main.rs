extern crate rand;
extern crate vecmath;
extern crate rayon;
extern crate pixels;
extern crate winit_input_helper;
extern crate winit;
extern crate log;
extern crate env_logger;

use std::time::Duration;

use rand::distributions::{Distribution, Uniform};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use log::error;

mod particle;
use particle::Particle;
mod gravity_grid;
use gravity_grid::GravityGrid;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct World {
    GG1: GravityGrid,
    GG2: GravityGrid,
    particles: Vec<Particle>,
}

impl World {
    fn new(gg1: GravityGrid, gg2: GravityGrid, particles: Vec<Particle>) -> World{
        World { 
            GG1: gg1,
            GG2: gg2,
            particles: particles 
        }
    }

    fn update(&mut self, dt: f32){
        self.GG1.zero_mass();
        self.GG1.compute_mass(&mut self.particles);
        self.GG1.compute_force();
        self.GG1.apply_force(&mut self.particles);

        self.GG2.zero_mass();
        self.GG2.compute_mass(&mut self.particles);   
        self.GG2.compute_force();
        self.GG2.apply_force(&mut self.particles);
        for p in &mut self.particles{
            p.update(dt);
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        for p in &self.particles{
            let x = p.x_pos as u32;
            let y = p.y_pos as u32;
            if x < WIDTH && y < HEIGHT{
                let index = (y*WIDTH + x) as usize;
                frame[index*4] = 255;
                frame[index*4 + 1] = 0;
                frame[index*4 + 2] = 0;
                frame[index*4 + 3] = 255;
            }
        }
    }
}

fn main(){
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Particle simulation")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels =  Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    //distributions
    let x_range = Uniform::new(0.0, WIDTH as f32);
    let y_range = Uniform::new(0.0, HEIGHT as f32);
    let mut rng = rand::thread_rng();

    //generate particles
    let mut particles: Vec<Particle> = Vec::new();
    let n_particles: usize = 1000;
    for n in 0..n_particles {
        let particle = Particle::new(x_range.sample(&mut rng),  y_range.sample(&mut rng), 1.0);
        particles.push(particle);
    }

    let mut GG1 = GravityGrid::new(9,9, WIDTH as f64/9.0);
    let mut GG2 = GravityGrid::new(27,27, WIDTH as f64/27.0);

    let mut world = World::new(GG1, GG2, particles);
    
    let mut elapsed: Option<Duration> = None;
    let mut dt = 0.0;
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        // get time
        let now = Some(std::time::Instant::now());
        if elapsed.is_some() {;
            dt = elapsed.unwrap().as_secs_f32();
        }

        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update(dt);
            window.request_redraw();
        }
        elapsed = Some(now.unwrap().elapsed());
        print!("dt: {}\r", dt);
    });
}
