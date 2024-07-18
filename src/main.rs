mod framebuffer;
mod bmp;
mod color;
mod line_impl;

use framebuffer::Framebuffer;
use line_impl::Line; 
use color::Color;

fn main() -> std::io::Result<()> {
    println!("Hello World");
    Ok(())
}

fn draw_polygon(framebuffer: &mut Framebuffer, points: &[(usize, usize)]) {
    if points.len() < 3 {
        // Not enough points to form a polygon
        return;
    }

    // Draw lines between consecutive points
    let mut last_point = points[0];
    for &point in &points[1..] {
        framebuffer.line(last_point.0, last_point.1, point.0, point.1);
        last_point = point;
    }

    // Connect the last point back to the first point to close the polygon
    let first_point = points[0];
    framebuffer.line(last_point.0, last_point.1, first_point.0, first_point.1);
}

fn fill_polygon(framebuffer: &mut Framebuffer, points: &[(usize, usize)]) {
    // Similar implementation as previously described, tailored for filling with the current color
}
