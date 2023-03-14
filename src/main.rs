#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

mod colors;
mod crates;
mod elements;
mod lib;
mod model;
mod prelude;
mod window_dimensions;

use std::thread::sleep;

use itertools::Itertools;
use prelude::*;

fn main() {
    if should_create_timelapse() {
        snapshot::timelapse::create();
        return;
    }

    nannou::app(start).update(update).exit(snapshot::exit).run();
}

fn start(app: &App) -> Model {
    let [window_width, window_height] = get_window_dimensions();
    let monitor_is_bigger_than_window = check_if_monitor_is_bigger_than_window(app);

    let _window_builder = app
        .new_window()
        .view(draw)
        .size(window_width as u32, window_height as u32)
        .key_released(capture_frame_on_s)
        .pipe_if(monitor_is_bigger_than_window, |builder| {
            // If we show decorations, Nannou won't make the window bigger than the screen.
            // And it only saves the part of the image that's visible in the window.
            builder.decorations(false)
        })
        .build()
        .unwrap();

    let loop_mode = get_loop_mode();
    app.set_loop_mode(loop_mode);

    Model::new(app)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.done_rendering {
        return;
    }

    // let params = UpdateParams {
    //     app,
    //     update,
    //     rand: &model.rand,
    // };

    if let DoneDrawing::Yes = model.drawing.update() {
        println!("Done drawing.");
        model.snapshot.capture_frame(app);
        model.done_rendering = true;
        app.set_loop_mode(LoopMode::loop_once())
    }
}

pub fn draw(app: &App, model: &Model, frame: Frame) {
    if model.done_rendering {
        return;
    }

    // Required for incremental rendering to work sometimes. Not sure why.
    // std::thread::sleep(std::time::Duration::from_millis(200));

    let rand = model.snapshot.get_rand();
    let container = &model.container;

    // Scale and translate the draw instance so that we
    // can use normalized points.
    let draw = app
        .draw()
        .scale_axes(vec3(container.w(), container.h(), container.w()))
        .translate(vec3(-0.5, -0.5, -0.5));

    let params = DrawParams {
        app,
        model,
        rand: &rand,
        draw: &draw,
        container,
    };

    // model.root_element.draw(&params);
    model.drawing.draw(&params);

    draw.to_frame(app, &frame).unwrap();
}

fn capture_frame_on_s(app: &App, model: &mut Model, key: Key) {
    if key == Key::S {
        model.snapshot.capture_frame(app);
    }
}

fn get_loop_mode() -> LoopMode {
    let still = std::env::args().any(|argument| argument == "--dont-animate");
    if still {
        LoopMode::loop_once()
    } else {
        LoopMode::default() // RefreshSync
    }
}

fn check_if_monitor_is_bigger_than_window(app: &App) -> bool {
    let [window_width, window_height] = get_window_dimensions();

    let primary_monitor = app.primary_monitor().unwrap();
    let primary_monitor_size: nannou::winit::dpi::LogicalSize<u32> = primary_monitor
        .size()
        .to_logical(primary_monitor.scale_factor());

    let monitor_width = primary_monitor_size.width as f32;
    let monitor_height = primary_monitor_size.height as f32;

    monitor_width > window_width && monitor_height > window_height
}

fn should_create_timelapse() -> bool {
    std::env::args().any(|argument| argument == "--create-timelapse")
}
