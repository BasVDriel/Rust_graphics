pub struct Config {
    pub width: u32,
    pub height: u32,
    pub g: f32,
    pub particles: u32
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 1000, 
            height: 1000, 
            g: 0.00001, 
            particles: 5000,
        }
    }
}