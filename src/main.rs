extern crate image;
extern crate ansi_term;

use ansi_term::Colour::Fixed;

use std::path::Path;

use image::{imageops, FilterType};
use image::Pixel;

fn main() {
    let img = image::open(&Path::new("test.jpg")).unwrap();

    let ansi_colours = [
[ 0x00, 0x00, 0x00 ],[ 0x80, 0x00, 0x00 ],[ 0x00, 0x80, 0x00 ],[ 0x80, 0x80, 0x00 ],[ 0x00, 0x00, 0x80 ],
[ 0x80, 0x00, 0x80 ],[ 0x00, 0x80, 0x80 ],[ 0xc0, 0xc0, 0xc0 ],[ 0x80, 0x80, 0x80 ],[ 0xff, 0x00, 0x00 ],
[ 0x00, 0xff, 0x00 ],[ 0xff, 0xff, 0x00 ],[ 0x00, 0x00, 0xff ],[ 0xff, 0x00, 0xff ],[ 0x00, 0xff, 0xff ],
[ 0xff, 0xff, 0xff ],[ 0x00, 0x00, 0x00 ],[ 0x00, 0x00, 0x5f ],[ 0x00, 0x00, 0x87 ],[ 0x00, 0x00, 0xaf ],
[ 0x00, 0x00, 0xd7 ],[ 0x00, 0x00, 0xff ],[ 0x00, 0x5f, 0x00 ],[ 0x00, 0x5f, 0x5f ],[ 0x00, 0x5f, 0x87 ],
[ 0x00, 0x5f, 0xaf ],[ 0x00, 0x5f, 0xd7 ],[ 0x00, 0x5f, 0xff ],[ 0x00, 0x87, 0x00 ],[ 0x00, 0x87, 0x5f ],
[ 0x00, 0x87, 0x87 ],[ 0x00, 0x87, 0xaf ],[ 0x00, 0x87, 0xd7 ],[ 0x00, 0x87, 0xff ],[ 0x00, 0xaf, 0x00 ],
[ 0x00, 0xaf, 0x5f ],[ 0x00, 0xaf, 0x87 ],[ 0x00, 0xaf, 0xaf ],[ 0x00, 0xaf, 0xd7 ],[ 0x00, 0xaf, 0xff ],
[ 0x00, 0xd7, 0x00 ],[ 0x00, 0xd7, 0x5f ],[ 0x00, 0xd7, 0x87 ],[ 0x00, 0xd7, 0xaf ],[ 0x00, 0xd7, 0xd7 ],
[ 0x00, 0xd7, 0xff ],[ 0x00, 0xff, 0x00 ],[ 0x00, 0xff, 0x5f ],[ 0x00, 0xff, 0x87 ],[ 0x00, 0xff, 0xaf ],
[ 0x00, 0xff, 0xd7 ],[ 0x00, 0xff, 0xff ],[ 0x5f, 0x00, 0x00 ],[ 0x5f, 0x00, 0x5f ],[ 0x5f, 0x00, 0x87 ],
[ 0x5f, 0x00, 0xaf ],[ 0x5f, 0x00, 0xd7 ],[ 0x5f, 0x00, 0xff ],[ 0x5f, 0x5f, 0x00 ],[ 0x5f, 0x5f, 0x5f ],
[ 0x5f, 0x5f, 0x87 ],[ 0x5f, 0x5f, 0xaf ],[ 0x5f, 0x5f, 0xd7 ],[ 0x5f, 0x5f, 0xff ],[ 0x5f, 0x87, 0x00 ],
[ 0x5f, 0x87, 0x5f ],[ 0x5f, 0x87, 0x87 ],[ 0x5f, 0x87, 0xaf ],[ 0x5f, 0x87, 0xd7 ],[ 0x5f, 0x87, 0xff ],
[ 0x5f, 0xaf, 0x00 ],[ 0x5f, 0xaf, 0x5f ],[ 0x5f, 0xaf, 0x87 ],[ 0x5f, 0xaf, 0xaf ],[ 0x5f, 0xaf, 0xd7 ],
[ 0x5f, 0xaf, 0xff ],[ 0x5f, 0xd7, 0x00 ],[ 0x5f, 0xd7, 0x5f ],[ 0x5f, 0xd7, 0x87 ],[ 0x5f, 0xd7, 0xaf ],
[ 0x5f, 0xd7, 0xd7 ],[ 0x5f, 0xd7, 0xff ],[ 0x5f, 0xff, 0x00 ],[ 0x5f, 0xff, 0x5f ],[ 0x5f, 0xff, 0x87 ],
[ 0x5f, 0xff, 0xaf ],[ 0x5f, 0xff, 0xd7 ],[ 0x5f, 0xff, 0xff ],[ 0x87, 0x00, 0x00 ],[ 0x87, 0x00, 0x5f ],
[ 0x87, 0x00, 0x87 ],[ 0x87, 0x00, 0xaf ],[ 0x87, 0x00, 0xd7 ],[ 0x87, 0x00, 0xff ],[ 0x87, 0x5f, 0x00 ],
[ 0x87, 0x5f, 0x5f ],[ 0x87, 0x5f, 0x87 ],[ 0x87, 0x5f, 0xaf ],[ 0x87, 0x5f, 0xd7 ],[ 0x87, 0x5f, 0xff ],
[ 0x87, 0x87, 0x00 ],[ 0x87, 0x87, 0x5f ],[ 0x87, 0x87, 0x87 ],[ 0x87, 0x87, 0xaf ],[ 0x87, 0x87, 0xd7 ],
[ 0x87, 0x87, 0xff ],[ 0x87, 0xaf, 0x00 ],[ 0x87, 0xaf, 0x5f ],[ 0x87, 0xaf, 0x87 ],[ 0x87, 0xaf, 0xaf ],
[ 0x87, 0xaf, 0xd7 ],[ 0x87, 0xaf, 0xff ],[ 0x87, 0xd7, 0x00 ],[ 0x87, 0xd7, 0x5f ],[ 0x87, 0xd7, 0x87 ],
[ 0x87, 0xd7, 0xaf ],[ 0x87, 0xd7, 0xd7 ],[ 0x87, 0xd7, 0xff ],[ 0x87, 0xff, 0x00 ],[ 0x87, 0xff, 0x5f ],
[ 0x87, 0xff, 0x87 ],[ 0x87, 0xff, 0xaf ],[ 0x87, 0xff, 0xd7 ],[ 0x87, 0xff, 0xff ],[ 0xaf, 0x00, 0x00 ],
[ 0xaf, 0x00, 0x5f ],[ 0xaf, 0x00, 0x87 ],[ 0xaf, 0x00, 0xaf ],[ 0xaf, 0x00, 0xd7 ],[ 0xaf, 0x00, 0xff ],
[ 0xaf, 0x5f, 0x00 ],[ 0xaf, 0x5f, 0x5f ],[ 0xaf, 0x5f, 0x87 ],[ 0xaf, 0x5f, 0xaf ],[ 0xaf, 0x5f, 0xd7 ],
[ 0xaf, 0x5f, 0xff ],[ 0xaf, 0x87, 0x00 ],[ 0xaf, 0x87, 0x5f ],[ 0xaf, 0x87, 0x87 ],[ 0xaf, 0x87, 0xaf ],
[ 0xaf, 0x87, 0xd7 ],[ 0xaf, 0x87, 0xff ],[ 0xaf, 0xaf, 0x00 ],[ 0xaf, 0xaf, 0x5f ],[ 0xaf, 0xaf, 0x87 ],
[ 0xaf, 0xaf, 0xaf ],[ 0xaf, 0xaf, 0xd7 ],[ 0xaf, 0xaf, 0xff ],[ 0xaf, 0xd7, 0x00 ],[ 0xaf, 0xd7, 0x5f ],
[ 0xaf, 0xd7, 0x87 ],[ 0xaf, 0xd7, 0xaf ],[ 0xaf, 0xd7, 0xd7 ],[ 0xaf, 0xd7, 0xff ],[ 0xaf, 0xff, 0x00 ],
[ 0xaf, 0xff, 0x5f ],[ 0xaf, 0xff, 0x87 ],[ 0xaf, 0xff, 0xaf ],[ 0xaf, 0xff, 0xd7 ],[ 0xaf, 0xff, 0xff ],
[ 0xd7, 0x00, 0x00 ],[ 0xd7, 0x00, 0x5f ],[ 0xd7, 0x00, 0x87 ],[ 0xd7, 0x00, 0xaf ],[ 0xd7, 0x00, 0xd7 ],
[ 0xd7, 0x00, 0xff ],[ 0xd7, 0x5f, 0x00 ],[ 0xd7, 0x5f, 0x5f ],[ 0xd7, 0x5f, 0x87 ],[ 0xd7, 0x5f, 0xaf ],
[ 0xd7, 0x5f, 0xd7 ],[ 0xd7, 0x5f, 0xff ],[ 0xd7, 0x87, 0x00 ],[ 0xd7, 0x87, 0x5f ],[ 0xd7, 0x87, 0x87 ],
[ 0xd7, 0x87, 0xaf ],[ 0xd7, 0x87, 0xd7 ],[ 0xd7, 0x87, 0xff ],[ 0xd7, 0xaf, 0x00 ],[ 0xd7, 0xaf, 0x5f ],
[ 0xd7, 0xaf, 0x87 ],[ 0xd7, 0xaf, 0xaf ],[ 0xd7, 0xaf, 0xd7 ],[ 0xd7, 0xaf, 0xff ],[ 0xd7, 0xd7, 0x00 ],
[ 0xd7, 0xd7, 0x5f ],[ 0xd7, 0xd7, 0x87 ],[ 0xd7, 0xd7, 0xaf ],[ 0xd7, 0xd7, 0xd7 ],[ 0xd7, 0xd7, 0xff ],
[ 0xd7, 0xff, 0x00 ],[ 0xd7, 0xff, 0x5f ],[ 0xd7, 0xff, 0x87 ],[ 0xd7, 0xff, 0xaf ],[ 0xd7, 0xff, 0xd7 ],
[ 0xd7, 0xff, 0xff ],[ 0xff, 0x00, 0x00 ],[ 0xff, 0x00, 0x5f ],[ 0xff, 0x00, 0x87 ],[ 0xff, 0x00, 0xaf ],
[ 0xff, 0x00, 0xd7 ],[ 0xff, 0x00, 0xff ],[ 0xff, 0x5f, 0x00 ],[ 0xff, 0x5f, 0x5f ],[ 0xff, 0x5f, 0x87 ],
[ 0xff, 0x5f, 0xaf ],[ 0xff, 0x5f, 0xd7 ],[ 0xff, 0x5f, 0xff ],[ 0xff, 0x87, 0x00 ],[ 0xff, 0x87, 0x5f ],
[ 0xff, 0x87, 0x87 ],[ 0xff, 0x87, 0xaf ],[ 0xff, 0x87, 0xd7 ],[ 0xff, 0x87, 0xff ],[ 0xff, 0xaf, 0x00 ],
[ 0xff, 0xaf, 0x5f ],[ 0xff, 0xaf, 0x87 ],[ 0xff, 0xaf, 0xaf ],[ 0xff, 0xaf, 0xd7 ],[ 0xff, 0xaf, 0xff ],
[ 0xff, 0xd7, 0x00 ],[ 0xff, 0xd7, 0x5f ],[ 0xff, 0xd7, 0x87 ],[ 0xff, 0xd7, 0xaf ],[ 0xff, 0xd7, 0xd7 ],
[ 0xff, 0xd7, 0xff ],[ 0xff, 0xff, 0x00 ],[ 0xff, 0xff, 0x5f ],[ 0xff, 0xff, 0x87 ],[ 0xff, 0xff, 0xaf ],
[ 0xff, 0xff, 0xd7 ],[ 0xff, 0xff, 0xff ],[ 0x08, 0x08, 0x08 ],[ 0x12, 0x12, 0x12 ],[ 0x1c, 0x1c, 0x1c ],
[ 0x26, 0x26, 0x26 ],[ 0x30, 0x30, 0x30 ],[ 0x3a, 0x3a, 0x3a ],[ 0x44, 0x44, 0x44 ],[ 0x4e, 0x4e, 0x4e ],
[ 0x58, 0x58, 0x58 ],[ 0x60, 0x60, 0x60 ],[ 0x66, 0x66, 0x66 ],[ 0x76, 0x76, 0x76 ],[ 0x80, 0x80, 0x80 ],
[ 0x8a, 0x8a, 0x8a ],[ 0x94, 0x94, 0x94 ],[ 0x9e, 0x9e, 0x9e ],[ 0xa8, 0xa8, 0xa8 ],[ 0xb2, 0xb2, 0xb2 ],
[ 0xbc, 0xbc, 0xbc ],[ 0xc6, 0xc6, 0xc6 ],[ 0xd0, 0xd0, 0xd0 ],[ 0xda, 0xda, 0xda ],[ 0xe4, 0xe4, 0xe4 ],
[ 0xee, 0xee, 0xee ]];

    let img = imageops::resize(&img, 130, 60, FilterType::Nearest);

    for (x, _y, pixel) in img.enumerate_pixels() {
        let pixel = pixel.to_rgb();
        let channels = pixel.channels();
        let r = channels[0] as i32;
        let g = channels[1] as i32;
        let b = channels[2] as i32;
       
        let mut best = 0 as u8;
        let mut best_distance = 255 * 255 * 3 + 1;
        for i in 0..255 {
            let ansi_colour = ansi_colours[i];
            let dr: i32 = ansi_colour[0] - r;
            let dg: i32 = ansi_colour[1] - g;
            let db: i32 = ansi_colour[2] - b;
            let distance = dr * dr + dg * dg + db * db;
        
            if distance < best_distance {
                best_distance = distance;
                best = i as u8;
            }
        }
        if x == 0 {
            print!("\n");
        }    
        print!("{}", Fixed(best).paint("█"));
    }
    print!("\n");
}


