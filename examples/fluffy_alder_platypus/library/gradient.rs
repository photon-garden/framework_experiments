use std::marker::PhantomData;

use crate::prelude::*;

#[derive(Debug)]
pub struct Gradient {
    elements: Vec<GradientElement>,
}

impl Gradient {
    pub fn build() -> GradientBuilder<LastElementIsATransition> {
        let gradient = Gradient { elements: vec![] };

        GradientBuilder {
            gradient,
            phantom: PhantomData,
        }
    }

    pub fn get_color(&self, progress: NormalizedF32) -> Hsl {
        let element_location = self
            .elements
            .weighted_get(progress, |element| element.get_duration());

        let element = element_location.element;

        match element {
            GradientElement::Color {
                hsl,
                duration: _duration,
            } => *hsl,

            GradientElement::Transition { duration } => {
                let previous_color_start = element_location.total_weight_at_previous_element;
                let next_color_start = previous_color_start + duration;

                let element_index = element_location.index;
                let previous_color = self.elements.get(element_index - 1).expect("There's a bug in gradient.rs. You shouldn't be able to construct a transition unless it happens between two colors.").try_get_hsl().expect("If this isn't a color, there's a bug in gradient.rs.");
                let next_color = self.elements.get(element_index + 1).expect("There's a bug in gradient.rs. You shouldn't be able to construct a transition unless it happens between two colors.").try_get_hsl().expect("If this isn't a color, there's a bug in gradient.rs.");

                let progress_between_colors =
                    progress.normalize(previous_color_start, next_color_start);

                previous_color.mix_pigment(next_color, progress_between_colors)
                // previous_color.lerp(next_color, progress_between_colors)
            }
        }
    }

    pub fn add_color(&mut self, color: Hsl, duration: f32) -> &mut Gradient {
        self.elements.push(GradientElement::Color {
            hsl: color,
            duration,
        });

        self
    }

    pub fn add_transition(&mut self, duration: f32) -> &mut Gradient {
        let last_element_is_color = self
            .elements
            .last()
            .is_some_and_matches(|element| element.is_color());

        if !last_element_is_color {
            panic!("Transitions need a color immediately before them. Currently there's another transition, or the transition is the first element in the gradient.");
        }

        self.elements.push(GradientElement::Transition { duration });
        self
    }

    fn finish(mut self) -> Self {
        self.normalize();
        self
    }

    fn normalize(&mut self) {
        let total_duration: f32 = self
            .elements
            .iter()
            .map(|element| element.get_duration())
            .sum();

        if total_duration.close_to(1.0) {
            return;
        }

        for element in self.elements.iter_mut() {
            let duration = element.get_duration();
            let normalized_duration = duration / total_duration;
            element.set_duration(normalized_duration);
        }
    }
}

pub struct GradientBuilder<LastElement> {
    gradient: Gradient,
    phantom: PhantomData<LastElement>,
}

impl GradientBuilder<LastElementIsATransition> {
    pub fn add_color(mut self, color: Hsl, duration: f32) -> GradientBuilder<LastElementIsAColor> {
        self.gradient.add_color(color, duration);

        GradientBuilder {
            gradient: self.gradient,
            phantom: PhantomData,
        }
    }
}

impl GradientBuilder<LastElementIsAColor> {
    pub fn add_color(mut self, color: Hsl, duration: f32) -> GradientBuilder<LastElementIsAColor> {
        self.gradient.add_color(color, duration);

        GradientBuilder {
            gradient: self.gradient,
            phantom: PhantomData,
        }
    }

    pub fn add_transition(mut self, duration: f32) -> GradientBuilder<LastElementIsATransition> {
        self.gradient.add_transition(duration);

        GradientBuilder {
            gradient: self.gradient,
            phantom: PhantomData,
        }
    }

    pub fn finish(self) -> Gradient {
        self.gradient.finish()
    }
}

pub struct LastElementIsAColor;
pub struct LastElementIsATransition;

#[derive(Debug)]
enum GradientElement {
    Color { hsl: Hsl, duration: f32 },
    Transition { duration: f32 },
}

impl GradientElement {
    fn get_duration(&self) -> f32 {
        match self {
            GradientElement::Color {
                hsl: _hsl,
                duration,
            } => *duration,
            GradientElement::Transition { duration } => *duration,
        }
    }

    fn set_duration(&mut self, new_duration: f32) {
        match self {
            GradientElement::Color {
                hsl: _hsl,
                ref mut duration,
            } => *duration = new_duration,

            GradientElement::Transition { ref mut duration } => *duration = new_duration,
        }
    }

    fn try_get_hsl(&self) -> Option<&Hsl> {
        match self {
            GradientElement::Color {
                hsl,
                duration: _duration,
            } => Some(hsl),

            GradientElement::Transition {
                duration: _duration,
            } => None,
        }
    }

    fn is_color(&self) -> bool {
        match self {
            GradientElement::Color {
                hsl: _hsl,
                duration: _duration,
            } => true,
            GradientElement::Transition {
                duration: _duration,
            } => false,
        }
    }

    fn is_transition(&self) -> bool {
        !self.is_color()
    }
}
