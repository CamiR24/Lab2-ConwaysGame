mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::FrameBuffer;
use crate::line::line;

use std::thread;
use std::time::Duration;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window 1")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = FrameBuffer::new(framebuffer_width as u32, framebuffer_height as u32, Color::WHITE);

    framebuffer.set_background_color(Color::new(255, 255, 255, 255));

    let mut translate_x = 0.0;
    let mut translate_y = 0.0;


    while !window.window_should_close() {
        translate_x += 1.0;
        translate_y += 1.0;

        framebuffer.clear();

        render(&mut framebuffer, translate_x, translate_y);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));

    }

    fn render(
        framebuffer: &mut FrameBuffer,
        translate_x: f32,
        translate_y: f32,
    ) {
        framebuffer.set_current_color(Color::SKYBLUE);
        line(
            framebuffer,
            Vector2::new(50.0 + translate_x, 50.0 + translate_y),
            Vector2::new(350.0 + translate_x, 350.0 + translate_y),
        );

        framebuffer.set_current_color(Color::SKYBLUE);
        line(
            framebuffer,
            Vector2::new(350.0 + translate_x, 50.0 + translate_y),
            Vector2::new(50.0 + translate_x, 350.0 + translate_y),
        );
    }

}