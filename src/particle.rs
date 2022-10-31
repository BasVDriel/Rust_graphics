

pub struct Particle{
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_vel: f32,
    pub y_vel: f32,
    pub index: Option<usize>,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Particle{
        Particle {x_pos: x, y_pos: y, x_vel: 0.0, y_vel: 0.0, index: None}
    }

    pub fn apply_force(&mut self, f_x: f32, f_y: f32){
        self.x_vel = self.x_vel + f_x;
        self.y_vel = self.y_vel + f_y;
    }

    pub fn update(&mut self, dt: f32){
        self.x_pos = self.x_pos + dt as f32*self.x_vel;
        self.y_pos = self.y_pos + dt as f32*self.y_vel;
    }
}