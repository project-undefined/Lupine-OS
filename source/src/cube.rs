use crate::{console, print, println};
use libm::{cos, sin};

const CUBE_WIDTH: f64 = 15.;
const WIDTH: usize = 120;
const HEIGHT: usize = 48;
const DISTANCE_FROM_CAM: i32 = 60;
const K1: f64 = 31.;
const CUBE_AXIS_INCREMENT: [f64; 61] = [
    -15., -14.5, -14., -13.5, -13., -12.5, -12., -11.5, -11., -10.5, -10., -9.5, -9., -8.5, -8.,
    -7.5, -7., -6.5, -6., -5.5, -5., -4.5, -4., -3.5, -3., -2.5, -2., -1.5, -1., -0.5, 0., 0.5, 1.,
    1.5, 2., 2.5, 3., 3.5, 4., 4.5, 5., 5.5, 6., 6.5, 7., 7.5, 8., 8.5, 9., 9.5, 10., 10.5, 11.,
    11.5, 12., 12.5, 13., 13.5, 14., 14.5, 15.,
];

pub fn main() {
    #[allow(unused_assignments)]
    let mut z_buffer: [f64; WIDTH * HEIGHT * 4] = [0.; WIDTH * HEIGHT * 4];
    #[allow(unused_assignments)]
    let mut buffer: [&'static str; WIDTH * HEIGHT] = [" "; WIDTH * HEIGHT];
    let (mut a, mut b, mut c): (f64, f64, f64) = (0., 0., 0.);
    #[allow(unused_assignments)]
    let mut comp_trig: [f64; 6] = [0.; 6];
    print!("\x1b[2J"); // Clear Screen
    console::console().flush();

    loop {
        z_buffer = [0.; WIDTH * HEIGHT * 4];
        buffer = [" "; WIDTH * HEIGHT];
        comp_trig = [sin(a), cos(a), sin(b), cos(b), sin(c), cos(c)];
        for cube_x in CUBE_AXIS_INCREMENT.iter() {
            for cube_y in CUBE_AXIS_INCREMENT.iter() {
                calculate_for_surface(
                    cube_x,
                    cube_y,
                    &-CUBE_WIDTH,
                    &comp_trig,
                    "\x1b[91mQ",
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_for_surface(
                    &CUBE_WIDTH,
                    cube_y,
                    cube_x,
                    &comp_trig,
                    "\x1b[92mW",
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_for_surface(
                    &-CUBE_WIDTH,
                    cube_y,
                    &-cube_x,
                    &comp_trig,
                    "\x1b[93mE",
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_for_surface(
                    &-cube_x,
                    cube_y,
                    &CUBE_WIDTH,
                    &comp_trig,
                    "\x1b[94mR",
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_for_surface(
                    cube_x,
                    &-CUBE_WIDTH,
                    &-cube_y,
                    &comp_trig,
                    "\x1b[95mT",
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_for_surface(
                    cube_x,
                    &CUBE_WIDTH,
                    cube_y,
                    &comp_trig,
                    "\x1b[96mY",
                    &mut z_buffer,
                    &mut buffer,
                );
            }
        }
        print!("\x1b[H");
        for k in 0..(WIDTH * HEIGHT) {
            match k % WIDTH {
                0 => println!(""),
                _ => print!("{}", buffer[k]),
            };
        }
        a += 0.05;
        b -= 0.025;
        c += 0.01;
        console::console().flush();
    }
}

fn calculate_x(i: &f64, j: &f64, k: &f64, ct: &[f64; 6]) -> f64 {
    return j * ct[0] * ct[2] * ct[5] - k * ct[1] * ct[2] * ct[5]
        + j * ct[1] * ct[4]
        + k * ct[0] * ct[4]
        + i * ct[3] * ct[5];
}

fn calculate_y(i: &f64, j: &f64, k: &f64, ct: &[f64; 6]) -> f64 {
    return j * ct[1] * ct[5] + k * ct[0] * ct[5] - j * ct[0] * ct[2] * ct[4]
        + k * ct[1] * ct[2] * ct[4]
        - i * ct[3] * ct[4];
}

fn calculate_z(i: &f64, j: &f64, k: &f64, ct: &[f64; 6]) -> f64 {
    return k * ct[1] * ct[3] - j * ct[0] * ct[3] + i * ct[2];
}

fn calculate_for_surface(
    cube_x: &f64,
    cube_y: &f64,
    cube_z: &f64,
    comp_trig: &[f64; 6],
    ch: &'static str,
    z_buffer: &mut [f64; WIDTH * HEIGHT * 4],
    buffer: &mut [&'static str; WIDTH * HEIGHT],
) {
    let x: f64 = calculate_x(cube_x, cube_y, cube_z, comp_trig);
    let y: f64 = calculate_y(cube_x, cube_y, cube_z, comp_trig);
    let z: f64 = calculate_z(cube_x, cube_y, cube_z, comp_trig) + DISTANCE_FROM_CAM as f64;
    let ooz: f64 = 1. / z;
    let xp: i32 = ((WIDTH / 2) as f64 - 2. * CUBE_WIDTH + K1 * ooz * x * 2.) as i32;
    let yp: i32 = ((HEIGHT / 2) as f64 + K1 * ooz * y) as i32;

    let idx: i32 = xp + yp * (WIDTH as i32);
    if idx >= 0 && idx < (WIDTH * HEIGHT) as i32 {
        if ooz > z_buffer[idx as usize] {
            z_buffer[idx as usize] = ooz;
            buffer[idx as usize] = ch;
        }
    }
}
