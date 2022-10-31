use vecmath::*;
use particle::Particle;
use config::Config;

struct Permutation{
    x1: isize,
    y1: isize,
    mass1: u32,
    x2: isize,
    y2: isize,
    mass2: u32,
    cell_size: f32,
}
pub struct GravityGrid{
    //pub gravity_grid: Grid,
    pub width: u32,
    pub height: u32,
    pub cell_size: f64,
    pub cell_mass: Vec<u32>,
    offset: [f32; 2],
    cell_forces: Vec<Vector2<f32>>,
    pub gl: Config,
}

impl GravityGrid{
    pub fn new(world_width: u32, world_height: u32, cell_size: f64) -> GravityGrid{
        let grid_width = 4+world_width/cell_size as u32;
        let grid_height = 4+world_height/cell_size as u32;
        let cell_mass = vec![0; (grid_height*grid_width) as usize];
        let zero: Vector2<f32> = [0.0,0.0];
        let cell_forces = vec![zero; (grid_height*grid_width) as usize];
        let gl = Config::new();
        GravityGrid {
            width: grid_width, 
            height: grid_height, 
            cell_size: cell_size, 
            cell_mass: cell_mass, 
            offset: [2.0*cell_size as f32, 2.0*cell_size as f32],
            cell_forces: cell_forces,
            gl: Config::new(),
        }
    }

    pub fn set_offset(&mut self, x: f32, y: f32){
        self.offset = [x,y];
    }

    fn get_index(&self, particle: &mut Particle) -> Option<usize>{
        let x = particle.x_pos;
        let y = particle.y_pos;
        let mut index = None;
        if x < 0.0 || y < 0.0{
            particle.index = index;
            return index
        }
        let x_index = ((x+ self.offset[0])/self.cell_size as f32 ).floor() as u32;
        let y_index = ((y+ self.offset[0])/self.cell_size as f32).floor() as u32;

        if x_index < self.width && y_index < self.height {
            index = Some((y_index*self.width + x_index) as usize);
            particle.index = index;
        }
        return index
    }

    pub fn zero_mass(&mut self){
        for i in 0..self.cell_mass.len(){
            self.cell_mass[i] = 0;
        }
    }

    pub fn compute_mass(&mut self, particles: &mut Vec<Particle>){
        for particle in particles.iter_mut(){
            let cell_index = self.get_index(particle);
            if cell_index.is_some(){
                let cell = self.cell_mass.get_mut(cell_index.unwrap()).unwrap();
                *cell = *cell + 1;
            }
        }
    }

    fn compute_gravity(cell_size: f32, x1: isize, y1: isize, mass1: u32, x2: isize,  y2: isize, mass2: u32, g: f32)-> Vector2<f32>{
        //compute the force between two cells using the formula F = G*m1*m2/r^
        let x = (x2 - x1) as f32 * cell_size;
        let y = (y2 - y1) as f32 * cell_size;
        let r_squared = x*x + y*y;
        let force = g*mass1 as f32*mass2 as f32/r_squared;
        [x*force, y*force]
    }

    fn devide_and_conquer(perms: &mut [Permutation], g: f32) -> [f32; 2]{
        let l = perms.len();
        if l >= 2 {
            let mid = perms.len()/2;
            let (lo, hi) = perms.split_at_mut(mid);
            let (f1, f2) = rayon::join(
                || GravityGrid::devide_and_conquer(lo, g),
                || GravityGrid::devide_and_conquer(hi, g),
            );
            return [f1[0] + f2[0], f1[1] + f2[1]]
        }
        else{   
            let perm = &perms[0];
            return GravityGrid::compute_gravity(perm.cell_size, perm.x1, perm.y1, perm.mass1, perm.x2, perm.y2, perm.mass2, g)
        }
    }

    pub fn compute_force(&mut self){
        let radius: usize = 5;
        let cell_size = self.cell_size as f32;
        //for first cell
        for y1 in 0..self.height as isize{
            for x1 in 0..self.width as isize{
                //make sure that every check is within bounds
                let mut y_bound_low = y1 - radius as isize;
                let mut y_bound_high = y1 + radius as isize;
                let mut x_bound_low = x1 - radius as isize;
                let mut x_bound_high = x1 + radius as isize;
                if y_bound_low < 0 {
                    y_bound_low = 0;
                }
                if y_bound_high > self.height as isize - 1{
                    y_bound_high = self.height as isize - 1;
                }
                if x_bound_low < 0{
                    x_bound_low = 0;
                }
                if x_bound_high > self.width as isize - 1{
                    x_bound_high = self.width as isize - 1;
                }

                //store all the permutations in a vector for mulithreading
                let index1 = y1 as usize*self.width as usize + x1 as usize;
                let mut permutations: Vec<Permutation> = Vec::new();
                for y2 in y_bound_low..y_bound_high{
                    for x2 in x_bound_low..x_bound_high as isize{
                        let index2 = y2 as usize*self.width as usize + x2 as usize;
                        let mass1 = self.cell_mass[index1];
                        let mass2 = self.cell_mass[index2];
                        if mass1 != 0 && mass2 != 0 && index1 != index2{
                            let perm = Permutation{
                                x1: x1,
                                y1: y1,
                                mass1: mass1,
                                x2: x2,
                                y2: y2,
                                mass2: mass2,
                                cell_size: cell_size,
                            };
                            permutations.push(perm);
                        }
                    }
                }
                if permutations.len() == 0{
                    self.cell_forces[index1] = [0.0, 0.0];
                }
                else{
                    self.cell_forces[index1] = GravityGrid::devide_and_conquer(&mut permutations, self.gl.g);
                }      
            }   
        }
    }

    pub fn apply_force(&mut self, particles: &mut Vec<Particle>){
        for particle in particles.iter_mut(){
            if particle.index.is_some(){
                let force = self.cell_forces.get(particle.index.unwrap());
                if force.is_some(){
                    //apply the force to the particle
                    let force = force.unwrap();
                    particle.apply_force(force[0], force[1]);
                }
            }
        }
    }

}