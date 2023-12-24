use std::process::Command;
use std::fs;
use std::path::Path;
extern crate image;
use image::GenericImageView;


pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn process_video(path_str: &str, downscale: u32) -> Vec<Vec<Vec<Rgb>>> {
    let path = Path::new(path_str);

    let _ = fs::remove_dir_all("frames");

    fs::create_dir("frames").expect("Failed to create frames directory");
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(path)
        .arg("-vf")
        .arg(format!("scale={}", format!("iw/{}:ih/{}", downscale, downscale))) 
        .arg("frames/thumb%09d.png")
        .arg("-hide_banner")
        .output()
        .expect("Failed to execute command");


    println!("Status: {}", output.status);

    let file_count = fs::read_dir("frames").unwrap().count();

    let mut pixels: Vec<Vec<Vec<Rgb>>> = Vec::new();

    for x in 1..file_count + 1 {
        let zero_count = 9 - x.to_string().len();
        let file_path = format!("frames/thumb{}{}.png", "0".repeat(zero_count), x);
        let img = image::open(file_path).unwrap();

        let mut current_col: Vec<Vec<Rgb>> = Vec::new();

        for y in 0..img.height() {
            let mut current_row: Vec<Rgb> = Vec::new();
            for x in 0..img.width() {
                let rgba = img.get_pixel(x, y);
                let rgb = Rgb {
                    r: rgba[0],
                    g: rgba[1],
                    b: rgba[2],
                };
                current_row.push(rgb);

            }
            current_col.push(current_row);
        }
        pixels.push(current_col);


    }

    println!("{} frames processed", file_count);
    
    let _ = fs::remove_dir_all("frames");
    
    pixels

}
