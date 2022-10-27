use GravityGrid;
use Particle;
use config::Config;

pub struct World {
    pub ggs: Vec<GravityGrid>,
    particles: Vec<Particle>,
    gl: Config,
}

impl World {
    pub fn new(particles: Vec<Particle>) -> World{
        World { 
            ggs: Vec::new(),
            particles: particles,
            gl: Config::new(),
        }
    }

    pub fn update(&mut self, dt: f32){
        for gg in self.ggs.iter_mut(){
            gg.zero_mass();
            gg.compute_mass(&mut self.particles);
            gg.compute_force();
            gg.apply_force(&mut self.particles);
        }
        for p in &mut self.particles{
            p.update(dt);
        }
    }

    pub fn draw(&self, frame: &mut [u8], draw_grid: i32) {
        //set all pixels to black
        for i in 0..frame.len(){
            frame[i] = 0;
            if draw_grid > 0{
                 //convert i to x and y
                let x = (i as f32/4.0) as u32 %self.gl.width;
                let y = (i as f32/4.0) as u32 /self.gl.width;
                if x%self.ggs[draw_grid as usize].cell_size as u32 == 0|| y%self.ggs[draw_grid as usize].cell_size as u32 == 0{
                    frame[i] = 50;
                }
            }
        }
        //set particle color
        for p in &self.particles{
            let x = p.x_pos as u32;
            let y = p.y_pos as u32;
            if x < self.gl.width && y < self.gl.height && x > 0 && y > 0{
                let index = (y*self.gl.width + x) as usize;
                frame[index*4] = 255;
                frame[index*4 + 1] = 0;
                frame[index*4 + 2] = 0;
                frame[index*4 + 3] = 255;
            }
        }
    }
}