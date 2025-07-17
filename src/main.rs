mod framebuffer;

use raylib::prelude::*;
use framebuffer::FrameBuffer;

fn main() {
    let window_width = 800;
    let window_height = 800;

    let framebuffer_width = 150;
    let framebuffer_height = 150;

    let cell_size_x = window_width / framebuffer_width;
    let cell_size_y = window_height / framebuffer_height;

    let (mut window, thread) = raylib::init()
        .size(window_width as i32, window_height as i32)
        .title("Conway's Game of Life")
        .build();

    window.set_target_fps(10);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);
    framebuffer.set_initial_pattern();

    while !window.window_should_close() {
        framebuffer.step();

        let mut d = window.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        // Usar la función render
        framebuffer.render(&mut d, cell_size_x, cell_size_y);
    }
}