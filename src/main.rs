mod framebuffer;

use raylib::prelude::*;
use framebuffer::FrameBuffer;
use std::fs::create_dir_all;

fn main() {
    let window_width = 400;
    let window_height = 400;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let cell_size_x = window_width / framebuffer_width;
    let cell_size_y = window_height / framebuffer_height;

    let (mut window, thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Conway's Game of Life")
        .build();

    window.set_target_fps(10);

    create_dir_all("frames").expect("No se pudo crear la carpeta de frames");

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);
    framebuffer.set_initial_pattern();

    let mut frame_count = 0;
    let max_frames = 100;

    while !window.window_should_close() && frame_count < max_frames {
        framebuffer.step();

        {

            let mut d = window.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            framebuffer.render(&mut d, cell_size_x, cell_size_y);
        } 

        let filename = format!("frames/frame_{:04}.png", frame_count);
        window.take_screenshot(&thread, &filename);

        frame_count += 1;
    }
}
