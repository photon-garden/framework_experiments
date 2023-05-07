use crate::prelude::*;
use markov;
use nannou::image;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

type Srgb8Components = [u8; 3];
type Srgb8 = nannou::color::rgb::Rgb<nannou::color::encoding::Srgb, u8>;

pub struct ColorPredictor {
    markov_chain: markov::Chain<MarkovColor>,
    previous_color: MarkovColor,
    all_colors: Vec<Srgb8>,
    rand: Rand,
}

impl ColorPredictor {
    pub fn new(app: &App, rand: Rand, image_filename: &str) -> ColorPredictor {
        let (markov_chain, all_colors) = train_markov_chain(app, image_filename);

        let previous_color = markov_chain.generate().swap_remove(0);

        ColorPredictor {
            markov_chain,
            all_colors,
            previous_color,
            rand,
        }
    }

    pub fn next(&mut self) -> Srgb8 {
        let next_color = self.next_from_color(&self.previous_color.to_srgb8());

        self.previous_color = MarkovColor::from_srgb8(&next_color);

        next_color
    }

    pub fn next_from_color(&self, prompt: &Srgb8) -> Srgb8 {
        let prompt = MarkovColor::from_srgb8(prompt);

        let predicted_sentence = self.markov_chain.generate_from_token(prompt);
        let predicted_color = predicted_sentence.get(1);

        match predicted_color {
            Some(predicted_color) => predicted_color.to_srgb8(),
            None => *self.rand.element(&self.all_colors),
        }
    }
}

fn train_markov_chain(app: &App, image_filename: &str) -> (markov::Chain<MarkovColor>, Vec<Srgb8>) {
    let mut markov_chain = markov::Chain::of_order(4);
    let mut colors_set = HashSet::new();

    let path_to_image = app.assets_path().unwrap().join(image_filename);
    let image = image::open(path_to_image).unwrap().into_rgb8();

    let width = image.width();
    let height = image.height();

    let grid = Grid::new(width as usize, height as usize);

    for point in grid.iterate_points() {
        let x = point.x;
        let y = point.y;

        let pixel = image.get_pixel(x as u32, y as u32).0;
        let neighbors = grid.neighbors_around(x, y);

        colors_set.insert(pixel);

        let color = MarkovColor::from_components(&pixel);

        for neighbor_point in neighbors {
            let neighbor_x = neighbor_point.x;
            let neighbor_y = neighbor_point.y;

            let neighbor_pixel = image.get_pixel(neighbor_x as u32, neighbor_y as u32).0;
            let neighbor_color = MarkovColor::from_components(&neighbor_pixel);
            markov_chain.feed([color.clone(), neighbor_color]);
        }
    }

    let all_colors = colors_set.drain().map(|[r, g, b]| srgb8(r, g, b)).collect();

    (markov_chain, all_colors)
}

#[derive(Clone)]
struct MarkovColor {
    original_color: Srgb8Components,
    compressed_color: Srgb8Components,
}

impl MarkovColor {
    fn from_components(color: &Srgb8Components) -> MarkovColor {
        let compressed_color = MarkovColor::compress(color);

        MarkovColor {
            original_color: *color,
            compressed_color,
        }
    }

    fn from_srgb8(color: &Srgb8) -> MarkovColor {
        let components = [color.red, color.green, color.blue];
        MarkovColor::from_components(&components)
    }

    fn compress(color: &Srgb8Components) -> Srgb8Components {
        let [r, g, b] = color;
        [*r / 2, *g / 2, *b / 2]
    }

    fn to_srgb8(&self) -> Srgb8 {
        let [r, g, b] = self.original_color;
        srgb8(r, g, b)
    }
}

impl Eq for MarkovColor {}

impl PartialEq for MarkovColor {
    // When hashing and comparing equality, MarkovColors only pay attention to
    // the compressed color, ignoring the original color.
    fn eq(&self, other: &MarkovColor) -> bool {
        self.compressed_color[0] == other.compressed_color[0]
            && self.compressed_color[1] == other.compressed_color[1]
            && self.compressed_color[2] == other.compressed_color[2]
    }
}

impl Hash for MarkovColor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.compressed_color.hash(state)
    }
}
