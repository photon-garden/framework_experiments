use nannou::ease::*;
pub trait EasingsExtension {
    fn ease_in_circ(&self) -> f32;
    fn ease_out_circ(&self) -> f32;
    fn ease_in_out_circ(&self) -> f32;

    fn ease_in_cubic(&self) -> f32;
    fn ease_out_cubic(&self) -> f32;
    fn ease_in_out_cubic(&self) -> f32;

    fn ease_in_quad(&self) -> f32;
    fn ease_out_quad(&self) -> f32;
    fn ease_in_out_quad(&self) -> f32;

    fn ease_in_quart(&self) -> f32;
    fn ease_out_quart(&self) -> f32;
    fn ease_in_out_quart(&self) -> f32;

    fn ease_in_sine(&self) -> f32;
    fn ease_out_sine(&self) -> f32;
    fn ease_in_out_sine(&self) -> f32;

    fn apply_easing_function<EasingFunction>(&self, ease: EasingFunction) -> f32
    where
        EasingFunction: Fn(f32, f32, f32, f32) -> f32;
}

impl EasingsExtension for f32 {
    fn ease_in_circ(&self) -> f32 {
        self.apply_easing_function(circ::ease_in)
    }

    fn ease_out_circ(&self) -> f32 {
        self.apply_easing_function(circ::ease_out)
    }

    fn ease_in_out_circ(&self) -> f32 {
        self.apply_easing_function(circ::ease_in_out)
    }

    fn ease_in_cubic(&self) -> f32 {
        self.apply_easing_function(cubic::ease_in)
    }

    fn ease_out_cubic(&self) -> f32 {
        self.apply_easing_function(cubic::ease_out)
    }

    fn ease_in_out_cubic(&self) -> f32 {
        self.apply_easing_function(cubic::ease_in_out)
    }

    fn ease_in_quad(&self) -> f32 {
        self.apply_easing_function(quad::ease_in)
    }

    fn ease_out_quad(&self) -> f32 {
        self.apply_easing_function(quad::ease_out)
    }

    fn ease_in_out_quad(&self) -> f32 {
        self.apply_easing_function(quad::ease_in_out)
    }

    fn ease_in_quart(&self) -> f32 {
        self.apply_easing_function(quart::ease_in)
    }

    fn ease_out_quart(&self) -> f32 {
        self.apply_easing_function(quart::ease_out)
    }

    fn ease_in_out_quart(&self) -> f32 {
        self.apply_easing_function(quart::ease_in_out)
    }

    fn ease_in_sine(&self) -> f32 {
        self.apply_easing_function(sine::ease_in)
    }

    fn ease_out_sine(&self) -> f32 {
        self.apply_easing_function(sine::ease_out)
    }

    fn ease_in_out_sine(&self) -> f32 {
        self.apply_easing_function(sine::ease_in_out)
    }

    // Applies the given easing function assuming that self is normalized to the range 0.0 to 1.0.
    fn apply_easing_function<EasingFunction>(&self, ease: EasingFunction) -> f32
    where
        EasingFunction: Fn(f32, f32, f32, f32) -> f32,
    {
        ease(*self, 0.0, 1.0, 1.0)
    }
}
