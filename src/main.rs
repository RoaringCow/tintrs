mod video;
use video::process_video;
use std::thread;
use std::io::{stdout, Write};


trait ChangableColor: std::fmt::Display {

    fn color(&self, rgb: (u8, u8, u8)) -> String;
    fn background_color(&self, rgb: (u8, u8, u8)) -> String;
}

impl ChangableColor for str {


    fn color(&self, rgb: (u8, u8, u8)) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", rgb.0, rgb.1, rgb.2, self)
    }
    fn background_color(&self, rgb: (u8, u8, u8)) -> String {
        format!("\x1b[48;2;{};{};{}m{}\x1b[0m", rgb.0, rgb.1, rgb.2, self)
    }
    //fn custom_color(&self, )
}


fn main() {
    let delay = std::time::Duration::from_millis(1000 / 30);

    let frames = process_video("/home/ersan/tintrs/nevergonnagiveyouup.mp4", 5);

    for frame in frames {
        print!("\x1B[2J\x1B[1;1H");

        let mut buffer = String::new();

        for row in frame {
            for pixel in row {
                buffer.push_str(&"  ".background_color((pixel.r, pixel.g, pixel.b)).to_string());
            }
            buffer.push_str("\n");
        }
        print!("{}", buffer);
        stdout().flush().unwrap();

        thread::sleep(delay);
    }
}
