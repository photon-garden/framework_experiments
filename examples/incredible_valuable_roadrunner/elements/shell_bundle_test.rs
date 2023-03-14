use crate::prelude::*;

pub fn new() -> Element {
    Element::once(|params| {
        let shells = zero_to_one(9).map(|progress| get_shell(params.rand, progress));

        // for shell in shells {
        //     for where_in_shell in zero_to_one_xy(8, 16) {
        //         let point = shell.interior_point_xy(&where_in_shell);
        //         params
        //             .draw
        //             .ellipse()
        //             .xy(point)
        //             .resolution(64.0)
        //             .w_h(0.003, 0.003)
        //             .color(soft_black());
        //     }
        // }

        let shell_bundle = ShellBundle::new(shells);

        for along in zero_to_one(12) {
            for between in zero_to_one(27) {
                let point = shell_bundle.interior_point(along, between);
                params
                    .draw
                    .ellipse()
                    .xy(point)
                    .resolution(64.0)
                    .w_h(0.01, 0.01)
                    .color(soft_black());
            }
        }
    })
}

fn get_shell(rand: &Rand, progress_through_shells: f32) -> Shell {
    let start_x = progress_through_shells.denormalize(0.2, 0.8);
    let start_y = 0.2;

    let north_angle = 0.25;
    let turn_bias = progress_through_shells.denormalize(-0.1, 0.1);
    let turn_min = -0.1 * turn_bias;
    let turn_max = turn_bias;

    let step_size = 0.001;

    // let mut previous_angle = 0.0;
    let mut previous_point = pt2(start_x, start_y);
    let mut points = vec![previous_point];
    for _ in 0..512 {
        let noise_input = previous_point * 10.0;
        // let angle_offset = rand
        //     .super_simplex_curl_angle(&noise_input)
        //     .denormalize(turn_min, turn_max);
        let angle_offset = 0.0;
        let angle = north_angle + angle_offset;
        let new_point = previous_point.walk(angle, step_size);
        points.push(new_point);
        previous_point = new_point;
    }

    points.as_shell(|_progress, _point_to_translate_from| [0.03, 0.03])
}
