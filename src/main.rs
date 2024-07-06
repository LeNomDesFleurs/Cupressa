use std::error::Error;
extern crate anyhow;
use std::io;
use std::{thread, time};
mod vec;
use std::ops::{Add, Mul, Sub};
use vec::{Vec2, Vec3};
mod svf;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

// const GRAYSCALE_STRING: String = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const GRAYSCALE_CHAR: [char; 92] = [
    ' ', '`', '.', '-', '\'', ':', '_', ',', '^', '=', ';', '>', '<', '+', '!', 'r', 'c', '*', '/',
    'z', '?', 's', 'L', 'T', 'v', ')', 'J', '7', '(', '|', 'F', 'i', '{', 'C', '}', 'f', 'I', '3',
    '1', 't', 'l', 'u', '[', 'n', 'e', 'o', 'Z', '5', 'Y', 'x', 'j', 'y', 'a', ']', '2', 'E', 'S',
    'w', 'q', 'k', 'P', '6', 'h', '9', 'd', '4', 'V', 'p', 'O', 'G', 'b', 'U', 'A', 'K', 'X', 'H',
    'm', '8', 'R', 'D', '#', '$', 'B', 'g', '0', 'M', 'N', 'W', 'Q', '%', '&', '@',
];
const GRAYSCALE_VALUE: [f32; 92] = [
    0., 0.0751, 0.0829, 0.0848, 0.1227, 0.1403, 0.1559, 0.185, 0.2183, 0.2417, 0.2571, 0.2852,
    0.2902, 0.2919, 0.3099, 0.3192, 0.3232, 0.3294, 0.3384, 0.3609, 0.3619, 0.3667, 0.3737, 0.3747,
    0.3838, 0.3921, 0.396, 0.3984, 0.3993, 0.4075, 0.4091, 0.4101, 0.42, 0.423, 0.4247, 0.4274,
    0.4293, 0.4328, 0.4382, 0.4385, 0.442, 0.4473, 0.4477, 0.4503, 0.4562, 0.458, 0.461, 0.4638,
    0.4667, 0.4686, 0.4693, 0.4703, 0.4833, 0.4881, 0.4944, 0.4953, 0.4992, 0.5509, 0.5567, 0.5569,
    0.5591, 0.5602, 0.5602, 0.565, 0.5776, 0.5777, 0.5818, 0.587, 0.5972, 0.5999, 0.6043, 0.6049,
    0.6093, 0.6099, 0.6465, 0.6561, 0.6595, 0.6631, 0.6714, 0.6759, 0.6809, 0.6816, 0.6925, 0.7039,
    0.7086, 0.7235, 0.7302, 0.7332, 0.7602, 0.7834, 0.8037, 1.0,
];
const FRAMERATE: i32 = 40;
const DEFAULT_FRAMERATE:i32 = 20;
const FRAMERATE_COMPENSATION:f32 = DEFAULT_FRAMERATE as f32 / FRAMERATE as f32;
const GAMMA:f32 = 2.; 

fn get_color(light: f32) -> char {
    for (i, value) in GRAYSCALE_VALUE.iter().enumerate() {
        if *value >= light {
            return GRAYSCALE_CHAR[i];
        }
    }
    //default
    return '@';
}

fn main() -> Result<(), Box<dyn Error>> {
    // disable_raw_mode().unwrap();
    let mut time: f32 = 0.;

    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide)?;

    // grayscale_char.reverse(); //reverse pour avoir un noir qui vaut zero

    loop {
        // get terminal size
        // deform space
        let default_terminal_size = (10 as u16, 10 as u16);
        //moche, voir si mieux est possible, passer directement du tuple au Vec2 => définir une conversion `as` ?
        let terminal_size_temp = crossterm::terminal::size().unwrap_or(default_terminal_size);
        let terminal_size: Vec2<u16> = Vec2::new(terminal_size_temp.0, terminal_size_temp.1);
        // let mut buffer : [[char; terminal_size.x]; terminal_size.y]=
        for i in 0..terminal_size.x {
            for j in 0..terminal_size.y {
                let coord: Vec2<f32> = Vec2::new(i as f32, j as f32);

                let normalize_x =
                    (coord.x - (terminal_size.x as f32 / 2.0)) / terminal_size.x as f32;
                let normalize_y =
                    (coord.y - (terminal_size.y as f32 / 2.0)) / terminal_size.y as f32;

                let uv: Vec2<f32> = Vec2::new(normalize_x, normalize_y);

                //ray origin and direction
                let focal_length: f32 = 1.;
                let ray_origin: Vec3<f32> = Vec3 {
                    x: 0.,
                    y: -5.0,
                    z: 0.,
                };
                let ray_direction = svf::normalize(
                    Vec3 {
                        x: uv.x,
                        y: ray_origin.y + focal_length,
                        z: uv.y,
                    } - ray_origin,
                );

                //Ray marching
                let mut distance_along_ray = 0.;
                let mut position_along_ray: Vec3<f32> = Vec3::fill(0.);
                let mut has_intersected_object = false;

                for _ in 0..100 {
                    position_along_ray =
                        ray_origin + ray_direction * Vec3::fill(distance_along_ray);
                    let distance_to_closest_object = svf::scene(position_along_ray, time);
                    distance_along_ray += distance_to_closest_object;
                    if distance_to_closest_object < 0.0001 {
                        has_intersected_object = true;
                        break;
                    }
                }

                let mut color: char = ' ';
                if has_intersected_object {
                    let normal: Vec3<f32> = svf::compute_normal(position_along_ray, time);
                    let light_direction: Vec3<f32> = svf::normalize(Vec3 {
                        x: time.cos(),
                        y: time.sin(),
                        z: 0.9,
                    });
                    let mut light_attenuation = (-vec::dot(light_direction, normal)).max(0.);
                    light_attenuation *= 0.9;
                    light_attenuation += 0.05;
                    color = get_color(light_attenuation.powf(GAMMA));
                } else {
                    //black
                    color = get_color(0.);
                }

                //gradient
                // for (i, value) in grayscale_value.iter().enumerate() {
                //     if (*value - 0.5) > uv.x{
                //         color = grayscale_char[i];
                //         break;
                //     }
                // }

                time += 0.000005 / FRAMERATE_COMPENSATION;
                execute!(stdout, cursor::MoveTo(i, j), style::Print(color),)?;
            }
        }

        thread::sleep(time::Duration::from_millis(1000 / FRAMERATE as u64));
        // use std::panic;
    }
    Ok(())
}
