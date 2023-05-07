use crate::prelude::*;

pub struct ShellBundle {
    lerpable_shells: Vec<LerpableShell>,
}

impl ShellBundle {
    pub fn new(shells: impl IntoIterator<Item = Shell>) -> Self {
        let lerpable_shells = shells
            .into_iter()
            .map(DistanceLerpablePath::from_shell)
            .collect();
        Self { lerpable_shells }
    }

    pub fn interior_point(
        &self,
        distance_along: NormalizedF32,
        distance_between: NormalizedF32,
    ) -> Point2 {
        // Elements is all the shell elements at distance_along.
        let elements: Vec<LerpablePath2> = self
            .lerpable_shells
            .iter()
            .map(|lerpable_shell| {
                let element = lerpable_shell.lerp(distance_along).to_vec();
                DistanceLerpablePath::from_path2(element)
            })
            .collect();

        dbg!(elements.len());

        // Make elements lerpable.
        let lerpable_elements = DistanceLerpablePath::from_lerpable_paths(elements);

        lerpable_elements.lerp(distance_between)
    }
}
