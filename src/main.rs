extern crate sdl2;

mod magnifier;

use sdl2::event;

use magnifier::Slide;

fn main() {
    let sdl2_ctx = sdl2::init().unwrap();
    let video_ctx = sdl2_ctx.video().unwrap();
    let mut event_pump = sdl2_ctx.event_pump().unwrap();

    sdl2_ctx.mouse().set_relative_mouse_mode(true);

    let title = &format!(
        "{} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let width = 640;
    let height = 480;

    let mut window = video_ctx
        .window(title, width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    window.show();

    for event in event_pump.wait_iter() {
        match event {
            event::Event::MouseMotion { xrel, yrel, .. } => {
                window.slide(xrel, yrel).unwrap()
            }
            event::Event::Quit { .. } => break,
            _ => continue,
        }
    }
}
