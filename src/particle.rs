use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::color::*;

pub struct Particle{
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_vel: f32,
    pub y_vel: f32,
    pub mass: f32,
    pub index: Option<usize>,
}

impl Particle {
    pub fn new(x: f32, y: f32, mass: f32) -> Particle{
        Particle {x_pos: x, y_pos: y, x_vel: 0.0, y_vel: 0.0, mass: mass, index: None}
    }

    pub fn apply_force(&mut self, f_x: f32, f_y: f32){
        self.x_vel = self.x_vel + f_x/self.mass;
        self.y_vel = self.y_vel + f_y/self.mass;
    }

    pub fn update(&mut self, args: &UpdateArgs){
        self.x_pos = self.x_pos + args.dt as f32*self.x_vel;
        self.y_pos = self.y_pos + args.dt as f32*self.y_vel;
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics){
        use graphics::*;
        let square = rectangle::square(0.0, 0.0, 4.0);

        //render things in the world
        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(self.x_pos as f64, self.y_pos as f64);
            
                rectangle(RED, square, transform, gl); 
        });
    }
}