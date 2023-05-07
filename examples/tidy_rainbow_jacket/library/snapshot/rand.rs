use crate::prelude::*;
use fast_poisson::Poisson2D;
use nannou::noise::NoiseFn;
use nannou::noise::Perlin;
use nannou::noise::Seedable;
use nannou::noise::SuperSimplex;
use nanorand::Rng;
use std::cell::RefCell;
use std::fmt;
use std::ops::RangeBounds;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Rand {
    seed: u64,
    rng: RefCell<nanorand::WyRand>,
    perlin_noise_generator: RefCell<Option<Perlin>>,
    super_simplex_noise_generator: RefCell<Option<SuperSimplex>>,
    pub previous_alternation: RefCell<bool>,
}

unsafe impl Send for Rand {}

impl fmt::Debug for Rand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rand")
            .field("seed", &self.seed)
            .field("previous_alternation", &self.previous_alternation)
            .finish()
    }
}

impl Rand {
    pub fn from_seed(seed: u64) -> Rand {
        let mut rng = nanorand::WyRand::new_seed(seed);
        let previous_alternation = rng.generate::<usize>().is_even();

        Rand {
            seed,
            rng: RefCell::new(rng),
            perlin_noise_generator: RefCell::new(None),
            super_simplex_noise_generator: RefCell::new(None),
            previous_alternation: RefCell::new(previous_alternation),
        }
    }

    pub fn sum_octaves<Noise>(
        &self,
        num_octaves: usize,
        amplitude_factor: f32,
        point: &Point2,
        noise: Noise,
    ) -> f32
    where
        Noise: Fn(&Point2) -> f32,
    {
        let mut noise_value = 0.0;

        for octave in 1..=num_octaves {
            let frequency = octave as f32;
            let amplitude_exponent = (octave - 1) as i32;
            let amplitude = amplitude_factor.pow(amplitude_exponent);
            let scaled_point = point.times(frequency);
            let current_octave_noise_value = noise(&scaled_point) * amplitude;
            noise_value += current_octave_noise_value;
        }

        noise_value
    }

    pub fn sum_octaves_3d<Noise>(
        &self,
        num_octaves: usize,
        amplitude_factor: f32,
        point: &Point3,
        noise: Noise,
    ) -> f32
    where
        Noise: Fn(&Point3) -> f32,
    {
        let mut noise_value = 0.0;

        for octave in 1..=num_octaves {
            let frequency = octave as f32;
            let amplitude_exponent = (octave - 1) as i32;
            let amplitude = amplitude_factor.pow(amplitude_exponent);
            let scaled_point = point.times(frequency);
            let current_octave_noise_value = noise(&scaled_point) * amplitude;
            noise_value += current_octave_noise_value;
        }

        noise_value
    }

    pub fn new_with_random_seed(&self) -> Rand {
        let seed = self.rng.borrow_mut().generate::<u64>();
        Rand::from_seed(seed)
    }

    pub fn zero_to_one(&self) -> f32 {
        let random_number = self.rng.borrow_mut().generate::<u16>();
        (random_number as f32) / (u16::MAX as f32)
    }

    pub fn range_f32(&self, range: &RangeInclusive<f32>) -> f32 {
        self.zero_to_one().denormalize(*range.start(), *range.end())
    }

    pub fn range<Range: RangeBounds<usize>>(&self, range: Range) -> usize {
        self.rng.borrow_mut().generate_range(range)
    }

    pub fn complicated_gaussian(&self, x: f32, mean: f32, standard_deviation: f32) -> bool {
        let g = math::normalized_gaussian(x, mean, standard_deviation);
        self.flip_coin(g)
    }

    // Stays in the range -1.0 to 1.0 most of the time.
    pub fn standard_gaussian(&self) -> f32 {
        let u1 = self.zero_to_one();
        let u2 = self.zero_to_one();

        let left = (-2.0 * u1.ln()).sqrt();
        let right = (TAU * u2).cos();

        left * right
    }

    pub fn clamped_standard_gaussian(&self) -> f32 {
        self.standard_gaussian().clamp(-1.0, 1.0)
    }

    // Technically can go out of the range 0 to 1.
    pub fn normalized_gaussian(&self) -> NormalizedF32 {
        self.standard_gaussian().normalize(-1.0, 1.0)
    }

    pub fn normalized_gaussian_xy(&self) -> Point2 {
        let x = self.normalized_gaussian();
        let y = self.normalized_gaussian();
        pt2(x, y)
    }

    pub fn clamped_normalized_gaussian_xy(&self) -> Point2 {
        let x = self.clamped_normalized_gaussian();
        let y = self.clamped_normalized_gaussian();
        pt2(x, y)
    }

    pub fn standard_gaussian_xyz(&self) -> Point3 {
        let x = self.standard_gaussian();
        let y = self.standard_gaussian();
        let z = self.standard_gaussian();
        pt3(x, y, z)
    }

    pub fn normalized_gaussian_xyz(&self) -> Point3 {
        let x = self.normalized_gaussian();
        let y = self.normalized_gaussian();
        let z = self.normalized_gaussian();
        pt3(x, y, z)
    }

    pub fn clamped_normalized_gaussian_xyz(&self) -> Point3 {
        let x = self.clamped_normalized_gaussian();
        let y = self.clamped_normalized_gaussian();
        let z = self.clamped_normalized_gaussian();
        pt3(x, y, z)
    }

    pub fn clamped_normalized_gaussian(&self) -> NormalizedF32 {
        self.normalized_gaussian().clamp(0.0, 1.0)
    }

    pub fn normalized_gaussian_point(&self) -> NormalizedPoint2 {
        let x = self.normalized_gaussian();
        let y = self.normalized_gaussian();
        pt2(x, y)
    }

    pub fn flip_coin(&self, probability: f32) -> bool {
        self.zero_to_one() < probability
    }

    pub fn pick_first<Choice>(&self, probability: f32, first: Choice, second: Choice) -> Choice {
        if self.flip_coin(probability) {
            first
        } else {
            second
        }
    }

    pub fn gaussian_point(&self) -> Point2 {
        let x = self.standard_gaussian();
        let y = self.standard_gaussian();

        pt2(x, y)
    }

    pub fn clamped_gaussian_point(&self) -> Point2 {
        let x = self.clamped_standard_gaussian();
        let y = self.clamped_standard_gaussian();

        pt2(x, y)
    }

    pub fn perlin_noise_generator(&self) -> Perlin {
        let mut noise_generator_ref_cell = self.perlin_noise_generator.borrow_mut();

        match *noise_generator_ref_cell {
            Some(noise_generator) => noise_generator,

            None => {
                let new_noise_generator = Perlin::new();
                new_noise_generator.set_seed(self.seed as u32);

                *noise_generator_ref_cell = Some(new_noise_generator);

                new_noise_generator
            }
        }
    }

    // Output range is -1 to 1.
    // https://github.com/Razaekel/noise-rs/issues/149
    pub fn super_simplex_noise_generator(&self) -> SuperSimplex {
        let mut noise_generator_ref_cell = self.super_simplex_noise_generator.borrow_mut();

        match *noise_generator_ref_cell {
            Some(noise_generator) => noise_generator,

            None => {
                let new_noise_generator = SuperSimplex::new().set_seed(self.seed as u32);
                *noise_generator_ref_cell = Some(new_noise_generator);

                new_noise_generator
            }
        }
    }

    // Output range is 0 to 1. If you interpret this as an angle, it only gives you points in the
    // top right quadrant. If you're using curl noise for a flow field,
    // you should probably use the curl_angle method below.
    pub fn super_simplex_curl(&self, point: &Point2) -> Point2 {
        let super_simplex = self.super_simplex_noise_generator();
        self.curl(point, |x, y| super_simplex.get([x, y]))
    }

    pub fn super_simplex_curl_angle(&self, point: &Point2) -> NumberOfTurns {
        self.super_simplex_curl(point)
            .denormalize_to_range(-1.0, 1.0) // We denormalize like this so we can hit all four quadrants. If x and y were both normalized, we would only hit the top right quadrant.
            .normalized_angle()
    }

    pub fn curl<Noise>(&self, point: &Point2, noise: Noise) -> Point2
    where
        Noise: Fn(f64, f64) -> f64,
    {
        let x: f64 = point.x.into();
        let y: f64 = point.y.into();

        // We're going to approximate the derivative of
        // our noise function, so we need a very small
        // number to nudge our point by.
        let nudge_amount = 0.0001;
        let double_nudge_amount = nudge_amount * 2.0;

        let nudged_right = noise(x + nudge_amount, y);
        let nudged_left = noise(x - nudge_amount, y);

        // Approximate the derivative in the x direction.
        let dx = ((nudged_right - nudged_left) / double_nudge_amount) as f32;

        let nudged_up = noise(x, y + nudge_amount);
        let nudged_down = noise(x, y - nudge_amount);

        // Approximate the derivative in the y direction.
        let dy = ((nudged_up - nudged_down) / double_nudge_amount) as f32;

        // I ran an experiment where I input all combinations of x and y
        // ranging from -5,000 to 5,000. Then I measured the x and y outputs
        // of the noise function. I'm using those measurements to do this
        // normalization. The min and max for both x and y were the same.
        let output_x = dy.normalize(-3.6389112, 3.6389112);
        let output_y = dx.times(-1.0).normalize(-3.6389112, 3.6389112);

        pt2(output_x, output_y)
    }

    pub fn perlin_xy(&self, point: &Point2) -> f32 {
        self.perlin_x_y(point.x, point.y)
    }

    pub fn perlin_x_y(&self, x: f32, y: f32) -> f32 {
        let perlin_noise_generator = self.perlin_noise_generator();
        perlin_noise_generator.get([x.into(), y.into()]) as f32
    }

    pub fn super_simplex_x_y(&self, x: f32, y: f32) -> NormalizedF32 {
        let noise = self.super_simplex_noise_generator();

        let non_normalized_output = noise.get([x.into(), y.into()]) as f32;

        non_normalized_output.normalize(-1.0, 1.0)
    }

    pub fn super_simplex_xy(&self, point: &Point2) -> NormalizedF32 {
        self.super_simplex_x_y(point.x, point.y)
    }

    pub fn super_simplex_xyz(&self, point: &Point3) -> f32 {
        self.super_simplex_x_y_z(point.x, point.y, point.z)
    }

    pub fn super_simplex_x_y_z(&self, x: f32, y: f32, z: f32) -> f32 {
        let noise = self.super_simplex_noise_generator();

        let non_normalized_output = noise.get([x.into(), y.into(), z.into()]) as f32;

        non_normalized_output.normalize(-1.0, 1.0)
    }

    pub fn xy(&self) -> Point2 {
        pt2(self.zero_to_one(), self.zero_to_one())
    }

    pub fn xyz(&self) -> Point3 {
        pt3(self.zero_to_one(), self.zero_to_one(), self.zero_to_one())
    }

    pub fn element<'v, T>(&self, elements: &'v [T]) -> &'v T {
        let index = self.index(elements);
        elements
            .get(index)
            .expect("Couldn't get a random element from array.")
    }

    pub fn index<T>(&self, elements: &[T]) -> usize {
        let index: usize = self.rng.borrow_mut().generate_range(0..elements.len());

        index
    }

    pub fn weighted_choice<'a, Choice>(
        &self,
        choices_with_weights: &'a [(f32, Choice)],
    ) -> &'a Choice {
        let weight_total: f32 = choices_with_weights
            .iter()
            .map(|(weight, _choice)| *weight)
            .sum();

        let mut remaining_distance = self.zero_to_one() * weight_total;
        let last_index = choices_with_weights.len() - 1;

        for (index, (weight, choice)) in choices_with_weights.iter().enumerate() {
            let is_last_choice = index == last_index;

            remaining_distance -= *weight;
            if remaining_distance <= 0.0 || is_last_choice {
                return choice;
            }
        }

        panic!("There's a bug in weighted_choice and we got to the end of the function.");
    }

    pub fn alternate(&self) -> bool {
        let mut previous_alternation = self.previous_alternation.borrow_mut();
        let next_alternation = !*previous_alternation;
        *previous_alternation = next_alternation;

        next_alternation
    }

    // Do a random walk between elements.
    pub fn element_walker<Element>(self, elements: Vec<Element>) -> ElementWalker<Element> {
        let index = self.index(&elements);
        ElementWalker {
            index: RefCell::new(index),
            rand: self,
            elements,
        }
    }

    pub fn normalized_point_walker(self, step_size: f32) -> NormalizedXyWalker {
        let x = self.zero_to_one();
        let y = self.zero_to_one();
        NormalizedXyWalker {
            previous_xy: pt2(x, y),
            step_size,
            rand: self,
        }
    }

    pub fn normalized_point_walker_with_momentum(
        self,
        step_size: f32,
        window_size: usize,
    ) -> impl Iterator<Item = Point2> {
        let walker = self.normalized_point_walker(step_size);
        NormalizedXyWalkerWithMomentum {
            walker,
            averaging_window: AveragingWindow::new(window_size),
        }
    }

    pub fn normalized_poisson_points(&self, min_radius: f32) -> Vec<Point2> {
        Poisson2D::new()
            .with_dimensions([1.0, 1.0], min_radius)
            .with_seed(self.seed)
            .iter()
            .map(|[x, y]| pt2(x, y))
            .collect()
    }

    pub fn hsl(&self) -> Hsl {
        hsl(self.zero_to_one(), self.zero_to_one(), self.zero_to_one())
    }

    pub fn circular_xy_jitter(&self, xy: Point2, magnitude_range: &RangeInclusive<f32>) -> Point2 {
        let magnitude = self.range_f32(magnitude_range);
        self.circular_xy_jitter_with_magnitude(xy, magnitude)
    }

    pub fn circular_xy_jitter_with_magnitude(&self, xy: Point2, magnitude: f32) -> Point2 {
        let angle = self.zero_to_one();
        let jitter = Vec2::from_angle(angle).times(magnitude);
        xy + jitter
    }

    pub fn jitter_within_normalized_range(
        &self,
        value: NormalizedF32,
        amount: f32,
    ) -> NormalizedF32 {
        loop {
            let jitter = self.zero_to_one().denormalize_symmetrically(amount);
            let new_value = value + jitter;

            if new_value.is_normalized() {
                return new_value;
            }
        }
    }
}

pub struct ElementWalker<Element> {
    index: RefCell<usize>,
    rand: Rand,
    elements: Vec<Element>,
}

impl<Element> ElementWalker<Element> {
    pub fn next(&self) -> &Element {
        let mut index = *self.index.borrow();

        if self.rand.flip_coin(0.5) {
            index += 1;
        } else {
            index -= 1;
        }

        let min = 0;
        let max = self.elements.last_index();

        index = index.clamp(min, max);

        let element = self.elements.get(index).expect(
            "There's a bug in RandomWalker because it generated an index that was out of bounds.",
        );

        *self.index.borrow_mut() = index;

        element
    }
}

pub struct NormalizedXyWalker {
    previous_xy: Point2,
    step_size: NormalizedF32,
    rand: Rand,
}

impl Iterator for NormalizedXyWalker {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        let new_xy = self
            .rand
            .circular_xy_jitter_with_magnitude(self.previous_xy, self.step_size)
            .clamp_normalized();

        self.previous_xy = new_xy;

        new_xy.into_some()
    }
}

pub struct NormalizedXyWalkerWithMomentum {
    walker: NormalizedXyWalker,
    averaging_window: AveragingWindow<Point2>,
}

impl Iterator for NormalizedXyWalkerWithMomentum {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_new_xy = self.walker.next();
        if let Some(new_xy) = maybe_new_xy {
            self.averaging_window.add(new_xy);
            self.averaging_window.average().into_some()
        } else {
            None
        }
    }
}
