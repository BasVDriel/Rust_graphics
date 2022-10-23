use vecmath::*;
use particle::Particle;
use crossbeam::*;

static G: f32 = 100.0;
pub struct GravityGrid{
    //pub gravity_grid: Grid,
    pub width: u32,
    pub height: u32,
    pub cell_size: f64,
    pub cell_mass: Vec<f32>,
    cell_forces: Vec<Vector2<f32>>,
}

impl GravityGrid{
    pub fn new(grid_width: u32, grid_height: u32, cell_size: f64) -> GravityGrid{
        let cell_mass = vec![0.0; (grid_height*grid_width) as usize];
        let zero: Vector2<f32> = [0.0,0.0];
        let cell_forces = vec![zero; (grid_height*grid_width) as usize];

        GravityGrid {
            width: grid_width, 
            height: grid_height, 
            cell_size: cell_size, 
            cell_mass: cell_mass, 
            cell_forces: cell_forces,
        }
    }

    fn get_index(&self, particle: &mut Particle) -> Option<usize>{
        let x = particle.x_pos as f32;
        let y = particle.y_pos as f32;
        let cell_size = self.cell_size as f32;

        let x_index = (x/cell_size).floor() as u32;
        let y_index = (y/cell_size).floor() as u32;

        let mut index = None;
        if x_index < self.width && y_index < self.height{
            index = Some((y_index*self.width + x_index) as usize);
            particle.index = index;
        }
        index
    }

    pub fn zero_mass(&mut self){
        for i in 0..self.cell_mass.len(){
            self.cell_mass[i] = 0.0;
        }
    }

    pub fn compute_mass(&mut self, particles: &mut Vec<Particle>){
        for particle in particles.iter_mut(){
            let cell_index = self.get_index(particle);
            if cell_index != None{
                let cell = self.cell_mass.get_mut(cell_index.unwrap()).unwrap();
                *cell = *cell + particle.mass;
            }
        }
    }

    pub fn compute_force(&mut self){
        let radius: usize = 5;
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
                    2;
                }

                //store all the permutations in a vector for mulithreading
                let threads: usize = 1;
                struct Permutation{
                    x1: isize,
                    y1: isize,
                    mass1: f32,
                    x2: isize,
                    y2: isize,
                    mass2: f32,
                    cell_size: f32,
                }
                let mut permutations = Vec::new();
                let index1 = y1 as usize*self.width as usize + x1 as usize;
                for y2 in y_bound_low..y_bound_high{
                    for x2 in x_bound_low..x_bound_high as isize{
                        let index2 = y2 as usize*self.width as usize + x2 as usize;
                        let mass1 = self.cell_mass[index1];
                        let mass2 = self.cell_mass[index2];
                        if index1 != index2 && mass1 != 0.0 && mass2 != 0.0{
                            permutations.push(Permutation{
                                x1: x1, 
                                y1: y1, 
                                x2: x2, 
                                y2: y2, 
                                mass1:  mass1,
                                mass2: mass2,
                                cell_size: self.cell_size as f32,
                            });
                        }

                    }
                }
                let mut cell_forces: Vec<Vector2<f32>> = Vec::new();
                for n in 0..threads{
                    //split the permutations into chunks
                    let chunk = permutations.split_off(permutations.len()/threads);
                    //spawn a thread for each chunk and calculate the force
                    scope(|scope| {
                        let thread = scope.spawn(move |_| {
                            let mut perm_force = [0.0f32, 0.0f32];
                            for perm in chunk.iter(){
                                let distance = (((perm.x1 as f32 - perm.x2 as f32)*perm.cell_size).powi(2) + ((perm.y1 as f32 - perm.y2 as f32)*perm.cell_size).powi(2)).sqrt();
                                let force = G*perm.mass1*perm.mass2/distance.powi(2);
                                let angle = (perm.y2 as f32 - perm.y1 as f32).atan2(perm.x2 as f32 - perm.x1 as f32);
                                let force_x = force*angle.cos();
                                let force_y = force*angle.sin();
                                let force_vector: Vector2<f32> = [force_x, force_y];
                                perm_force = vec2_add(perm_force, force_vector);
                            }
                            perm_force
                        });
                        //concatenate the forces vector from the thread to the return vector
                        let perm_force = thread.join().unwrap();
                        cell_forces.push(perm_force);
                    });
                }
                let mut ret_force: Vector2<f32> = [0.0, 0.0]; 
                for force in cell_forces{
                    ret_force = vec2_add(ret_force, force);
                }
                self.cell_forces[index1] = ret_force;
            }   
        }
    }

    pub fn apply_force(&mut self, particles: &mut Vec<Particle>){
        for particle in particles.iter_mut(){
            if !particle.index.is_none(){
                let force = self.cell_forces.get(particle.index.unwrap()).unwrap();
                particle.apply_force(force[0], force[1])
            }
        }
    }

}