use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const RED: u32 = 0xFF0000;
const OFFSET: usize = 250;

fn put_pixel(buffer: &mut Vec<u32>, x: usize, y: usize, color: u32) {
    if x < WIDTH && y < HEIGHT {
        buffer[y * WIDTH + x] = color;
    }
}

fn draw_ellipse(buffer: &mut Vec<u32>, radius_x: usize, radius_y: usize, color: u32) {
    put_pixel(buffer, 0usize, radius_x, color);
    let rx_2 = radius_x.pow(2) as isize;
    let ry_2 = radius_y.pow(2) as isize;

    let mut p: isize = ry_2 - rx_2 * radius_y as isize + rx_2 / 4;

    let mut x: isize = 0;
    let mut y: isize = radius_y as isize;
    let mut dx: isize = 0;
    let mut dy: isize = 2 * rx_2 * y;

    println!("Region 1");
    while dx < dy {
        put_pixel(buffer, x as usize + OFFSET, y as usize + OFFSET, color);
        put_pixel(buffer, OFFSET - x as usize, y as usize + OFFSET, color);
        put_pixel(buffer, x as usize + OFFSET, OFFSET - y as usize, color);
        put_pixel(buffer, OFFSET - x as usize, OFFSET - y as usize, color);

        x += 1;
        dx += 2 * ry_2;

        if p < 0 {
            p += dx + ry_2;
        } else {
            y -= 1;
            dy -= 2 * rx_2;
            p += dx - dy + ry_2;
        }
    }

    let mut p = ry_2 as f32 * (x as f32 + (1.0 / 2.0)).powf(2.0)
        + rx_2 as f32 * (y as f32 - 1.0).powf(2.0)
        - rx_2 as f32 * ry_2 as f32;

    println!("Region 2");
    while y >= 0 {
        put_pixel(buffer, x as usize + OFFSET, y as usize + OFFSET, color);
        put_pixel(buffer, OFFSET - x as usize, y as usize + OFFSET, color);
        put_pixel(buffer, x as usize + OFFSET, OFFSET - y as usize, color);
        put_pixel(buffer, OFFSET - x as usize, OFFSET - y as usize, color);

        y -= 1;
        dy -= 2 * rx_2;

        if p > 0.0 {
            p = p - dy as f32 + rx_2 as f32;
        } else {
            x += 1;
            dx += 2 * ry_2;
            p += dx as f32 - dy as f32 + rx_2 as f32;
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Create a window
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    draw_ellipse(&mut buffer, 200, 100, RED);

    // Event loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw a red pixel at position (100, 100)

        // Update the window with the buffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| panic!("{}", e));
    }
}
