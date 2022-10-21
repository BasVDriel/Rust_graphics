use vecmath::*;
use particle::Particle;
static G: f32 = 0.001;
pub struct GravityGrid{
    //pub gravity_grid: Grid,
    pub width: u32,
    pub height: u32,
    pub cell_size: f64,
    pub cell_mass: Vec<f32>,
    pub cell_bounds: Vec<[f32; 2]>,
}

impl GravityGrid{
    pub fn new(grid_width: u32, grid_height: u32, cell_size: f64) -> GravityGrid{
        let cell_mass = vec![0.0; (grid_height*grid_width) as usize];

        //compute cell bounds
        let mut cell_bounds = Vec::new();
        for y in 0..grid_width{
            let y_min = y as f32 * cell_size as f32;
            for x in 0..grid_height{
                let x_min = x as f32 * cell_size as f32;
                cell_bounds.push([x_min, y_min]);
            }
        }

        GravityGrid {
            width: grid_width, 
            height: grid_height, 
            cell_size: cell_size, 
            cell_mass: cell_mass, 
            cell_bounds: cell_bounds
        }
    }

    pub fn compute_force(&mut self, particles: &mut Vec<Particle>){
        //calculating the force
        let radius = 5;
        let mut forces: Vec<Vector2<f32>> = Vec::new();

        for y1 in 0..self.height{
            for x1 in 0..self.width{
                //for each cell
                let mut force: vecmath::Vector2<f32> = [0.0f32, 0.0f32];
                let cell1_mass = self.cell_mass.get((x1 + y1*self.width) as usize).unwrap();
                let cell1_bounds = self.cell_bounds.get((x1 + y1*self.width) as usize).unwrap();
                let cell1_center: vecmath::Vector2<f32> = [cell1_bounds[0]+self.cell_size as f32/2.0, cell1_bounds[1]+self.cell_size as f32/2.0];
                
                //for each adjacent cell in radius
                for y2 in (y1 as i32 -radius as i32)..(y1 as i32+radius as i32){
                    if y2 < self.height as i32 && y2 >= 0{
                        for x2 in (x1 as i32 -radius as i32)..(x1 as i32+radius as i32){
                            if x2 < self.width as i32 && x2 >= 0{
                                let cell2_mass = self.cell_mass.get((x2 + y2*self.width as i32) as usize).unwrap();
                                let cell2_bounds = self.cell_bounds.get((x2 + y2*self.width as i32) as usize).unwrap();
                                let cell2_center: vecmath::Vector2<f32> = [cell2_bounds[0]+self.cell_size as f32/2.0, cell2_bounds[1]+self.cell_size as f32/2.0];

                                if !(cell2_bounds[0] == cell1_bounds[0] && cell2_bounds[1] == cell1_bounds[1]){
                                    //calculate force
                                    let dir= vec2_normalized_sub(cell2_center, cell1_center);
                                    let dist_sqr = vec2_square_len(vec2_sub(cell1_center, cell2_center)); 
                                    //add force of all these cells   
                                    let inter = (cell2_mass*cell1_mass*G)/dist_sqr;
                                    force = vec2_add(force, vec2_scale(dir, inter));
                                }
                            }
                        }
                    }
                }
                forces.push(force);
            }
        }

        //applying the force
        for p in particles{
            let mut cell_found = false;
            for y in 0..self.height{
                for x in 0..self.width{
                    let bounds = self.cell_bounds.get((x + y*self.width) as usize).unwrap();
                    if p.x_pos > bounds[0] && p.x_pos <= bounds[0]+self.cell_size as f32 && p.y_pos > bounds[1] && p.y_pos <= bounds[1]+self.cell_size as f32{
                        //apply force code
                        let f = forces.get((x + y*self.width) as usize).unwrap();
                        p.applyForce(f[0], f[1]);
                        cell_found = true;
                    }
                    if cell_found{
                        break;
                    }
                }
                if cell_found{
                    break;
                }
            }
        }
    }

    pub fn compute_mass(&mut self, particles: &Vec<Particle>){
        for p in particles{
            let mut cell_found = false;
            for y in 0..self.height{
                for x in 0..self.width{
                    let bounds = self.cell_bounds.get((x + y*self.width) as usize).unwrap();
                    if p.x_pos > bounds[0] && p.x_pos <= bounds[0]+self.cell_size as f32 && p.y_pos > bounds[1] && p.y_pos <= bounds[1]+self.cell_size as f32{
                        let mut mass = self.cell_mass.get_mut((x + y*self.width) as usize).unwrap();
                        *mass += p.mass;
                        cell_found = true;
                    }
                    if cell_found{
                        break;
                    }
                }
                if cell_found{
                    break;
                }
            }
        }
    }
}