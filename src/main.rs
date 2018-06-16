extern crate image;

use image::GenericImage;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return std::process::exit(-1);
    }

    let filepath = &args[1];
    let mut img = image::open(filepath).unwrap();
    let (width, height) = img.dimensions();
    let (mut diff16, mut min, mut cindex, mut rerr, mut gerr, mut berr): (
        i32,
        i32,
        usize,
        i32,
        i32,
        i32,
    );

    const R: usize = 0;
    const G: usize = 1;
    const B: usize = 2;

    let db16: [u8; 48] = [
        20, 12, 28, 68, 36, 52, 48, 52, 109, 78, 74, 78, 133, 76, 48, 52, 101, 36, 208, 70, 72,
        117, 113, 97, 89, 125, 206, 210, 125, 44, 133, 149, 161, 109, 170, 44, 210, 170, 153, 109,
        194, 202, 218, 212, 94, 222, 238, 214,
    ];

    for y in 0..height {
        for x in 0..width {
            let mut px = img.get_pixel(x, y);

            min = 10000000;
            cindex = 0;

            for c in 0..db16.len() / 3 {
                rerr = px.data[R] as i32 - db16[c * 3 + R] as i32;
                gerr = px.data[G] as i32 - db16[c * 3 + G] as i32;
                berr = px.data[B] as i32 - db16[c * 3 + B] as i32;
                diff16 = rerr * rerr + gerr * gerr + berr * berr;

                if diff16 < min {
                    min = diff16;
                    cindex = c;
                }
            }

            px.data[R] = db16[cindex * 3 + R];
            px.data[G] = db16[cindex * 3 + G];
            px.data[B] = db16[cindex * 3 + B];

            img.put_pixel(x, y, px);
        }
    }

    let filename = &Path::new(filepath).file_stem().unwrap().to_str().unwrap();
    let out_path = format!("db16_{}.png", filename);

    let ref mut fout = File::create(out_path).unwrap();

    img.save(fout, image::PNG).unwrap();
}
