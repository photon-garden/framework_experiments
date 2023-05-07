use crate::prelude::*;

// pub fn render_points(
//     params: &RenderParams,
//     rect: &Rect,
//     radius: f32,
//     direction: &Vec2,
//     foreground_color: &Hsl,
//     background_color: &Hsl,
// ) {
//     let draw = params.draw;
//     let container = params.container;
//     let rand = params.rand;

//     let zero = vec2(0.0, 0.0);

//     for rect_progress in ZERO_TO_ONE.subdivide(500).into_iter().rev() {
//         // let min = params.app.normalized_mouse_x();
//         let min = 0.14;
//         let eased_rect_progress = rect_progress.ease_out_quad();
//         let blend_amount = eased_rect_progress.denormalize(min, 1.0);

//         // let shadow_color = foreground_color.mix_pigment(background_color, blend_amount);
//         let mut shadow_color: Hsla = foreground_color
//             .mix_pigment(background_color, blend_amount)
//             .into();
//         // shadow_color.alpha = 0.51;

//         let shift = zero.lerp(*direction, rect_progress);

//         let shadow_rect = rect.shift(shift);

//         for point_progress in ZERO_TO_ONE.subdivide(500) {
//             // let point_jitter_radius = point_progress.ease_in_cubic().times(jitter_radius);

//             // let point = rand
//             //     .point_in_rect(&shadow_rect)
//             //     .jitter(rand, point_jitter_radius);

//             let point = rand.point_in_rect(&shadow_rect);

//             if rect.contains(point) {
//                 continue;
//             }

//             let denormalized_point = container.denormalize_xy(&point);
//             let denormalized_radius = container.lerp_w(0.005);

//             // draw.ellipse()
//             //     .color(shadow_color)
//             //     .radius(denormalized_radius)
//             //     .xy(denormalized_point);

//             draw.rect()
//                 .color(shadow_color)
//                 .w_h(denormalized_radius, denormalized_radius)
//                 .xy(denormalized_point);
//         }
//     }
// }

// pub fn render_rects(
//     params: &RenderParams,
//     rect: &Rect,
//     radius: f32,
//     direction: &Vec2,
//     foreground_color: &Hsl,
//     background_color: &Hsl,
// ) {
//     let draw = params.draw;
//     let container = params.container;
//     let rand = params.rand;

//     // let blend_amount = params.app.normalized_mouse_x();
//     let zero = vec2(0.0, 0.0);

//     // Draw shadow.
//     for progress in ZERO_TO_ONE.subdivide(1000).into_iter().rev() {
//         // let blend_amount = 0.3;
//         let blend_amount = progress.ease_out_quad();
//         let mut shadow_color: Hsla = foreground_color
//             .mix_pigment(background_color, blend_amount)
//             .into();
//         // shadow_color.alpha = progress.ease_out_quad().denormalize(0.001, 0.00);
//         // shadow_color.alpha = alpha;

//         let radius = 0.003;
//         let shift = zero.lerp(*direction, progress);
//         // let shift = zero.lerp(*direction, progress);
//         // let denormalized_shift = shift.denormalize_to_range(0.0, container.w());

//         let shadow_rect = rect.shift(shift);
//         let denormalized_shadow_rect = container.denormalize(&shadow_rect);

//         draw.rect()
//             .color(shadow_color)
//             .xy(denormalized_shadow_rect.xy())
//             .wh(denormalized_shadow_rect.wh());
//     }
// }

pub fn draw(
    params: &DrawParams,
    rect: &NormalizedRect,
    scale: f32, // Spread in CSS terms. How far the shadow extends beyond the shape.
    blur: f32,
    num_rects: usize,
    direction: &NormalizedVec2,
    shadow_color: Hsla,
) {
    let draw = params.draw;
    let container = params.model.container;
    let rand = params.rand;

    let denormalized_rect = container.denormalize_rect(rect);

    // Draw shadow.
    for _ in 0..num_rects {
        let normalized_translate = rand.normalized_gaussian_point().plus_xy(direction);
        let translate = container.denormalize_xy(&normalized_translate).times(blur);

        draw.scale(scale)
            .translate(vec3(translate.x, translate.y, 0.0))
            .rect()
            .color(shadow_color)
            .xy(denormalized_rect.xy())
            .wh(denormalized_rect.wh());
    }
}
