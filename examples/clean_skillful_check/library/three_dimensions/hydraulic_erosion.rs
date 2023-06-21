use crate::prelude::*;

const friction: f32 = 0.05;
const min_particle_volume: f32 = 0.01;
const deposition_rate: f32 = 0.3;
const evaporation_rate: f32 = 0.001;
const density: f32 = 1.0;
const num_particles_per_batch: usize = 1_000;
pub const max_particles: usize = 400_000;

pub struct HydraulicErosion {
    pub heightmap: Heightmap,
    pub particles: Vec<ErosionParticle>,
    pub total_particles_generated: usize,
}

impl HydraulicErosion {
    pub fn new(heightmap: Heightmap) -> HydraulicErosion {
        HydraulicErosion {
            heightmap,
            particles: vec![],
            total_particles_generated: 0,
        }
    }

    fn generate_particles(rand: &Rand) -> Vec<ErosionParticle> {
        let mut particles = Vec::with_capacity(num_particles_per_batch);

        for _ in 0..num_particles_per_batch {
            let position = rand.xy();
            let particle = ErosionParticle::new(position);
            particles.push(particle);
        }

        particles
    }

    pub fn iterate_until_all_particles_in_batch_are_evaporated(
        &mut self,
        elapsed_time: f32,
        rand: &Rand,
    ) {
        self.generate_more_particles_if_empty(rand);

        let heightmap = &mut self.heightmap;
        let particles = &mut self.particles;

        for particle in particles {
            while particle.alive {
                HydraulicErosion::update_particle(heightmap, elapsed_time, particle);
            }
        }

        self.particles = vec![];
    }

    pub fn iterate(&mut self, elapsed_time: f32, rand: &Rand) {
        self.generate_more_particles_if_empty(rand);

        let heightmap = &mut self.heightmap;
        let particles = &mut self.particles;

        particles.retain_mut(|particle| {
            HydraulicErosion::update_particle(heightmap, elapsed_time, particle);
            particle.alive
        })
    }

    fn update_particle(
        heightmap: &mut Heightmap,
        elapsed_time: f32,
        particle: &mut ErosionParticle,
    ) {
        HydraulicErosion::move_particle(heightmap, elapsed_time, particle);
        HydraulicErosion::transfer_sediment(heightmap, elapsed_time, particle);
        HydraulicErosion::evaporate(elapsed_time, particle);

        if particle.volume < min_particle_volume {
            particle.alive = false;
        }
    }

    fn generate_more_particles_if_empty(&mut self, rand: &Rand) {
        if self.particles.is_empty() {
            let mut new_particles = HydraulicErosion::generate_particles(rand);
            self.total_particles_generated += new_particles.len();
            self.particles.append(&mut new_particles);
        }
    }

    pub fn particle_paths(&self) -> impl Iterator<Item = Vec<Point3>> + '_ {
        self.particles.iter().map(|particle| {
            particle
                .path
                .iter()
                .map(|point_in_path| self.particle_xyz(point_in_path))
                .collect::<Vec<_>>()
        })
    }

    pub fn particle_positions(&self) -> impl Iterator<Item = (&ErosionParticle, Point3)> + '_ {
        self.particles.iter().map(|particle| {
            let position = self.particle_xyz(&particle.position);
            (particle, position)
        })
    }

    pub fn finished(&self) -> bool {
        self.total_particles_generated >= max_particles
    }

    fn particle_xyz(&self, domain_position: &Point2) -> Point3 {
        let height = self.heightmap.height_at(domain_position).unwrap();
        pt3(domain_position.x, height, domain_position.y)
    }

    fn move_particle(heightmap: &Heightmap, elapsed_time: f32, particle: &mut ErosionParticle) {
        let position = particle.position;
        let particle_direction = HydraulicErosion::particle_direction_at(heightmap, &position);

        particle.speed += elapsed_time * particle_direction / particle.mass();

        let new_position = particle.position + elapsed_time * particle.speed;

        if !heightmap.in_bounds(&new_position) {
            particle.alive = false;
            return;
        }

        particle.set_position(new_position);
        particle.speed *= 1.0 - elapsed_time * friction;
    }

    fn transfer_sediment(
        heightmap: &mut Heightmap,
        elapsed_time: f32,
        particle: &mut ErosionParticle,
    ) {
        if !particle.alive {
            return;
        }

        let previous_position = particle.previous_position;
        let current_position = particle.position;

        let height_at_previous_position = heightmap
            .height_at(&previous_position)
            .expect("Tried to transfer sediment at a point outside the heightmap.");
        let height_at_current_position = heightmap
            .height_at(&current_position)
            .expect("Tried to transfer sediment at a point outside the heightmap.");

        // This number is supposed to be bigger if we're moving downhill. But I
        // think this is going to be smaller if we're moving downhill.
        let height_difference = height_at_previous_position - height_at_current_position;
        let mut equilibrium_sediment_content =
            particle.volume * particle.speed.length() * height_difference;

        if equilibrium_sediment_content < 0.0 {
            equilibrium_sediment_content = 0.0
        }

        let distance_from_equilibrium = equilibrium_sediment_content - particle.sediment;
        let change_in_sediment_proportion =
            elapsed_time * deposition_rate * distance_from_equilibrium;
        particle.sediment += change_in_sediment_proportion;

        let change_in_sediment_mass = change_in_sediment_proportion * particle.volume;
        heightmap.update_height(&current_position, |height| height - change_in_sediment_mass)
    }

    fn evaporate(elapsed_time: f32, particle: &mut ErosionParticle) {
        if !particle.alive {
            return;
        }

        let evaporation_amount = elapsed_time * evaporation_rate;
        particle.volume *= 1.0 - evaporation_amount;
    }

    fn particle_direction_at(heightmap: &Heightmap, point: &Point2) -> Vec2 {
        let surface_normal = heightmap.surface_normal_at(point);
        vec2(surface_normal.x, surface_normal.z)
    }
}

pub struct ErosionParticle {
    position: Point2,
    previous_position: Point2,
    path: Vec<Point2>,
    speed: Vec2,
    pub volume: f32,
    sediment: NormalizedF32, // Fraction of volume that is sediment.
    alive: bool,
}

impl ErosionParticle {
    fn new(position: Point2) -> ErosionParticle {
        ErosionParticle {
            position,
            previous_position: position,
            speed: vec2(0.0, 0.0),
            volume: 1.0,
            sediment: 0.0,
            alive: true,
            path: vec![position],
        }
    }

    fn mass(&self) -> f32 {
        self.volume * density
    }

    fn set_position(&mut self, new_position: Point2) {
        self.previous_position = self.position;
        self.position = new_position;
        // self.path.push(new_position);
    }
}
