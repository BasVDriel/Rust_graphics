extern crate rand;
extern crate vecmath;
extern crate rayon;
extern crate pixels;
extern crate winit_input_helper;
extern crate winit;
extern crate log;
extern crate env_logger;

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
mod world;
use world::World;
mod config;
use config::Config;

fn main(){
    let gl = Config::new();
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(gl.width as f64, gl.height as f64);
        WindowBuilder::new()
            .with_title("Particle simulation")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels =  Pixels::new(gl.width, gl.width, surface_texture).unwrap();

    //distributions
    let x_range = Uniform::new(0.0, gl.width as f32);
    let y_range = Uniform::new(0.0, gl.height as f32);
    let sub_cell = Uniform::new(0.0, 50.0);
    let mut rng = rand::thread_rng();

    //generate particles
    let mut particles: Vec<Particle> = Vec::new();
    for n in 0..gl.particles{
        let particle = Particle::new(x_range.sample(&mut rng),  y_range.sample(&mut rng));
        particles.push(particle);
    }

    let gg1 = GravityGrid::new(gl.width,gl.height, 100.0);
    let gg2 = GravityGrid::new(gl.width,gl.height, 30.0);
    let gg3 = GravityGrid::new(gl.width,gl.height, 10.0);

    let mut world = World::new(particles);
    world.ggs.push(gg1);
    world.ggs.push(gg2);
    //world.ggs.push(gg3);
    
    let dt = 1.0;
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame(), -1);
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
            //world.ggs[0].set_offset(sub_cell.sample(&mut rng), sub_cell.sample(&mut rng));
            world.update(dt);
            window.request_redraw();
        }
    });
}
