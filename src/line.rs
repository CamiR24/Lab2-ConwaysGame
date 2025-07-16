use raylib::prelude::*;
use crate::framebuffer::FrameBuffer;

pub fn line(framebuffer: &mut FrameBuffer, start: Vector2, end: Vector2) {
    let points = bresenham_line(start.x as i32, start.y as i32, end.x as i32, end.y as i32);

    for (x, y) in points {
        framebuffer.set_pixel(x as u32, y as u32);
    }
}

fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut puntos = Vec::new();

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut x = x0;
    let mut y = y0;
    let mut err = dx - dy;

    loop {
        puntos.push((x, y));
        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    puntos
}