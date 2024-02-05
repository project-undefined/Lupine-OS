// This is just a simple ANSI test program

use crate::{console, print, println};
use libm::{cos, sin};

/// Width of the cube from origin
const CUBE_WIDTH: f64 = 15.;

/// Width of the screen in characters
const WIDTH: usize = 120;

/// Height of the screen in characters
const HEIGHT: usize = 48;

/// Distance the Cube is rendered, from the camera
const DISTANCE_FROM_CAM: i32 = 60;

/// Perspective control.
/// High Value: `Foreground > Background.`
/// Low Value: `Foreground ~= Background.`
const K1: f64 = 31.;

/// Sample point locations (On a 2d plane)
const CUBE_AXIS_INCREMENT: [f64; 61] = [
    -15., -14.5, -14., -13.5, -13., -12.5, -12., -11.5, -11., -10.5, -10., -9.5, -9., -8.5, -8.,
    -7.5, -7., -6.5, -6., -5.5, -5., -4.5, -4., -3.5, -3., -2.5, -2., -1.5, -1., -0.5, 0., 0.5, 1.,
    1.5, 2., 2.5, 3., 3.5, 4., 4.5, 5., 5.5, 6., 6.5, 7., 7.5, 8., 8.5, 9., 9.5, 10., 10.5, 11.,
    11.5, 12., 12.5, 13., 13.5, 14., 14.5, 15.,
];

pub fn main() {
    #[allow(unused_assignments)]
    // The z-buffer for each sample point.
    // Used to calculate if a sample point is visible to the camera
    let mut z_buffer: [f64; WIDTH * HEIGHT * 4] = [0.; WIDTH * HEIGHT * 4];

    #[allow(unused_assignments)]
    // The character buffer that is drawn to the screen
    let mut buffer: [&'static str; WIDTH * HEIGHT] = [" "; WIDTH * HEIGHT];

    // The rotation of the cube on three axis
    let (mut a, mut b, mut c): (f64, f64, f64) = (0., 0., 0.);

    #[allow(unused_assignments)]
    // Pre-computed trigonometry passed to `calculate_for_surface()`
    let mut comp_trig: [f64; 6] = [0.; 6];

    print!("\x1b[2J"); // Clear Screen

    loop {
        // Reset buffers
        z_buffer = [0.; WIDTH * HEIGHT * 4];
        buffer = [" "; WIDTH * HEIGHT];

        // Recalculate trig
        comp_trig = [sin(a), cos(a), sin(b), cos(b), sin(c), cos(c)];

        // Reset loop runs
        let mut x_loop_runs: i64 = 0;
        let mut y_loop_runs: i64 = 0;
        let mut cfs_runs: i64 = 0;

        // The `cube_x` loop is ran 61 times
        // The `cube_y` loop is ran 3,721 times
        // `calculate_for_surface` is ran a total of 22,326 times
        for cube_x in CUBE_AXIS_INCREMENT.iter() {
            x_loop_runs += 1;

            for cube_y in CUBE_AXIS_INCREMENT.iter() {
                y_loop_runs += 1;

                //    N
                //  W + E
                //    S
                //
                //   []  <- Cube
                //
                //   \/
                // Camera

                // ANSI Color Codes: https://i.stack.imgur.com/9UVnC.png

                // Calculate point (cube_x, cube_y) for North Face
                calculate_for_surface(
                    &-cube_x,
                    cube_y,
                    &CUBE_WIDTH,
                    &comp_trig,
                    "\x1b[91mN", // Foreground = 'Bright Red' (91)
                    &mut z_buffer,
                    &mut buffer,
                );
                cfs_runs += 1;

                // Calculate point (cube_x, cube_y) for South Face
                calculate_for_surface(
                    cube_x,
                    cube_y,
                    &-CUBE_WIDTH,
                    &comp_trig,
                    "\x1b[92mS", // Foreground = 'Bright Green' (92)
                    &mut z_buffer,
                    &mut buffer,
                );
                cfs_runs += 1;

                // Calculate point (cube_x, cube_y) for East Face
                calculate_for_surface(
                    &CUBE_WIDTH,
                    cube_y,
                    cube_x,
                    &comp_trig,
                    "\x1b[93mE", // Foreground = 'Bright Yellow' (93)
                    &mut z_buffer,
                    &mut buffer,
                );
                cfs_runs += 1;

                // Calculate point (cube_x, cube_y) for West Face
                calculate_for_surface(
                    &-CUBE_WIDTH,
                    cube_y,
                    &-cube_x,
                    &comp_trig,
                    "\x1b[94mW", // Foreground = 'Bright Blue' (94)
                    &mut z_buffer,
                    &mut buffer,
                );
                cfs_runs += 1;

                // Calculate point (cube_x, cube_y) for Top Face
                calculate_for_surface(
                    cube_x,
                    &-CUBE_WIDTH,
                    &-cube_y,
                    &comp_trig,
                    "\x1b[95mT", // Foreground = 'Bright Magenta' (95)
                    &mut z_buffer,
                    &mut buffer,
                );
                cfs_runs += 1;

                // Calculate point (cube_x, cube_y) for Bottom Face
                calculate_for_surface(
                    cube_x,
                    &CUBE_WIDTH,
                    cube_y,
                    &comp_trig,
                    "\x1b[96mB", // Foreground = 'Bright Cyan' (96)
                    &mut z_buffer,
                    &mut buffer,
                );
                cfs_runs += 1;
            }
        }
        print!("\x1b[H"); // Move Cursor to beginning of screen

        // Goes through buffer and print's visible points to the screen
        for k in 0..(WIDTH * HEIGHT) {
            // Character going to be printed at the end of the line?
            // Yes:
            //      Print new line
            // No:
            //      Print character
            match k % WIDTH {
                0 => println!(""),
                _ => print!("{}", buffer[k]),
            };
        }
        println!(
            "\x1b[0m
            a = {:?}
            b = {:?}
            c = {:?}\n
            x_loop_runs = {:?}
            y_loop_runs = {:?}
            cfs_runs = {:?}
            ",
            a, b, c, x_loop_runs, y_loop_runs, cfs_runs,
        );

        a += 0.05;
        b -= 0.025;
        c += 0.01;

        // print!("z_buffer = {:?};\tbuffer = {:?}", z_buffer, buffer);
        console::console().flush();
    }
}

/// Calculate the X value for 2D point on a 3D plane
fn calculate_x(i: &f64, j: &f64, k: &f64, ct: &[f64; 6]) -> f64 {
    return j * ct[0] * ct[2] * ct[5] - k * ct[1] * ct[2] * ct[5]
        + j * ct[1] * ct[4]
        + k * ct[0] * ct[4]
        + i * ct[3] * ct[5];
}

/// Calculate the Y value for 2D point on a 3D plane
fn calculate_y(i: &f64, j: &f64, k: &f64, ct: &[f64; 6]) -> f64 {
    return j * ct[1] * ct[5] + k * ct[0] * ct[5] - j * ct[0] * ct[2] * ct[4]
        + k * ct[1] * ct[2] * ct[4]
        - i * ct[3] * ct[4];
}

/// Calculate the Z value for 2D point on a 3D plane
fn calculate_z(i: &f64, j: &f64, k: &f64, ct: &[f64; 6]) -> f64 {
    return k * ct[1] * ct[3] - j * ct[0] * ct[3] + i * ct[2];
}

/// Calculate whether a point (from a 2D plane projected onto a 3D plane) is visible
fn calculate_for_surface(
    cube_x: &f64,
    cube_y: &f64,
    cube_z: &f64,
    comp_trig: &[f64; 6],
    ch: &'static str,
    z_buffer: &mut [f64; WIDTH * HEIGHT * 4],
    buffer: &mut [&'static str; WIDTH * HEIGHT],
) {
    // (x,y,z) coordinate of the 2D Coordinate
    let x: f64 = calculate_x(cube_x, cube_y, cube_z, comp_trig);
    let y: f64 = calculate_y(cube_x, cube_y, cube_z, comp_trig);
    let z: f64 = calculate_z(cube_x, cube_y, cube_z, comp_trig) + DISTANCE_FROM_CAM as f64;

    // Inverse of Z-coordinate (Used to calculate depth of point)
    let ooz: f64 = 1. / z;

    // (x,y) screen coordinates for the corresponding 2D coordinate
    let xp: i32 = ((WIDTH / 2) as f64 - 2. * CUBE_WIDTH + K1 * ooz * x * 2.) as i32;
    let yp: i32 = ((HEIGHT / 2) as f64 + K1 * ooz * y) as i32;

    // Index of the buffer to place the character at
    let idx: i32 = xp + yp * (WIDTH as i32);

    // If the index is part of the valid buffer range
    if idx >= 0 && idx < (WIDTH * HEIGHT) as i32 {
        // If the 3D coordinate point is visible
        if ooz > z_buffer[idx as usize] {
            // Add character to Screen Buffer and point to Z-Buffer
            z_buffer[idx as usize] = ooz;
            buffer[idx as usize] = ch;
        }
    }

    // print!(
    //    "
    //    cube_x = {:?}
    //    cube_y = {:?}
    //    cube_z = {:?}
    //    comp_trig = {:?}
    //    ch = '{}\x1b[0m'
    //    x = {:?}
    //    y = {:?}
    //    z = {:?}
    //    ooz = {:?}
    //    xp = {:?}
    //    yp = {:?}
    //    idx = {:?}
    //    z_buffer[idx] = {:?}",
    //    cube_x, cube_y, cube_z, comp_trig, ch, x, y, z, ooz, xp, yp, idx, z_buffer[idx as usize],
    //);
}
