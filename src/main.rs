mod framebuffer;
mod bmp;
mod color;
mod line_impl;

use framebuffer::Framebuffer;
use line_impl::Line; 
use color::Color;

fn main() -> std::io::Result<()> {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Clear the framebuffer with a default color, possibly black
    framebuffer.set_background_color(0x000000); // Pass hexadecimal value directly

    // Define the points for the polygons
    let points1: Vec<(usize, usize)> = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
    let points2: Vec<(usize, usize)> = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];
    let points3: Vec<(usize, usize)> = vec![
        (377, 249), (411, 197), (436, 249)
    ];
    let points4: Vec<(usize, usize)> = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), 
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192), 
        (659, 214), (615, 214), (632, 230), (580, 230), (597, 215), 
        (552, 214), (517, 144), (466, 180)
    ];
    let points5: Vec<(usize, usize)> = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    // Set the current drawing color to white for the polygon outline and then fill it
    framebuffer.set_current_color(0xFFFFFF); // White for the outline
    draw_polygon(&mut framebuffer, &points1);
    framebuffer.set_current_color(0xFFFF00); // Yellow for the fill
    fill_polygon(&mut framebuffer, &points1);

    framebuffer.set_current_color(0xFFFFFF); 
    draw_polygon(&mut framebuffer, &points2);
    framebuffer.set_current_color(0x0000FF);
    fill_polygon(&mut framebuffer, &points2);

    framebuffer.set_current_color(0xFFFFFF);
    draw_polygon(&mut framebuffer, &points3);
    framebuffer.set_current_color(0xFF0000);
    fill_polygon(&mut framebuffer, &points3);

    // Polygon 4 with Polygon 5 as a hole
    framebuffer.set_current_color(0xFFFFFF);
    draw_polygon(&mut framebuffer, &points4);
    framebuffer.set_current_color(0x00FF00); // Green for fill
    fill_polygon(&mut framebuffer, &points4);

    // Simulating the hole by redrawing and refilling the area of Polygon 5 with the background color
    framebuffer.set_current_color(0x000000);
    draw_polygon(&mut framebuffer, &points5);
    fill_polygon(&mut framebuffer, &points5);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("out.bmp")?;

    println!("Framebuffer rendered to out.bmp");
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
    if points.len() < 3 {
        return; // Not enough points to form a polygon
    }

    // Determine the bounding box of the polygon
    let min_y = points.iter().map(|p| p.1).min().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        let mut p1 = points.last().unwrap();
        for p2 in points.iter() {
            // Check for horizontal lines and skip them
            if p1.1 == p2.1 {
                p1 = p2;
                continue;
            }

            // Calculate intersections with the scanline
            if (p1.1 <= y && p2.1 > y) || (p1.1 > y && p2.1 <= y) {
                let dy = p2.1 as i32 - p1.1 as i32;
                let dx = p2.0 as i32 - p1.0 as i32;

                // Prevent division by zero and check for valid range
                if dy != 0 {
                    let x = p1.0 as i32 + ((y as i32 - p1.1 as i32) * dx) / dy;
                    if x >= 0 {
                        intersections.push(x as usize);
                    }
                }
            }
            p1 = p2;
        }

        // Fill between pairs of intersections
        intersections.sort_unstable();
        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                for x in pair[0]..=pair[1] {
                    framebuffer.point(x as u32, y as u32);
                }
            }
        }
    }
}

