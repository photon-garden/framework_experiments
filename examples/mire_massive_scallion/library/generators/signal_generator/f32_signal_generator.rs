use super::*;

pub type F32SignalGenerator = Box<dyn SignalGenerator<Output = f32>>;

pub struct OneNumberSignalGenerator {
    number: f32,
}

impl SignalGenerator for OneNumberSignalGenerator {
    type Output = f32;
    fn generate(&mut self, _rand: &Rand, _progress: NormalizedF32) -> f32 {
        self.number
    }
}

pub trait IntoF32SignalGenerator {
    fn into_signal_generator(self) -> F32SignalGenerator;
}

impl IntoF32SignalGenerator for f32 {
    fn into_signal_generator(self) -> F32SignalGenerator {
        Box::new(OneNumberSignalGenerator { number: self })
    }
}

impl<T> IntoF32SignalGenerator for T
where
    T: SignalGenerator<Output = f32> + 'static,
{
    fn into_signal_generator(self) -> F32SignalGenerator {
        Box::new(self)
    }
}
