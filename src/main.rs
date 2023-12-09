

trait ChangableColor {
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;

    fn black_back(&self) -> String;
    fn red_back(&self) -> String;
    fn green_back(&self) -> String;
    fn yellow_back(&self) -> String;
    fn blue_back(&self) -> String;
    fn magenta_back(&self) -> String;
    fn cyan_back(&self) -> String;
    fn white_back(&self) -> String;

}

impl ChangableColor for str {

    fn black(&self) -> String {
        ("\x1b[30m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn red(&self) -> String {
        ("\x1b[31m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn green(&self) -> String {
        ("\x1b[32m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn yellow(&self) -> String {
        ("\x1b[33m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn blue(&self) -> String {
        ("\x1b[34m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn magenta(&self) -> String {
        ("\x1b[35m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn cyan(&self) -> String {
        ("\x1b[36m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn white(&self) -> String {
        ("\x1b[37m".to_owned() + self + "\x1b[0m").to_owned()
    }

    // Backgrounds
    
    fn black_back(&self) -> String {
        ("\x1b[40m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn red_back(&self) -> String {
        ("\x1b[41m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn green_back(&self) -> String {
        ("\x1b[42m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn yellow_back(&self) -> String {
        ("\x1b[43m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn blue_back(&self) -> String {
        ("\x1b[44m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn magenta_back(&self) -> String {
        ("\x1b[45m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn cyan_back(&self) -> String {
        ("\x1b[46m".to_owned() + self + "\x1b[0m").to_owned()
    }
    fn white_back(&self) -> String {
        ("\x1b[47m".to_owned() + self + "\x1b[0m").to_owned()
    }
    
}

#[allow(dead_code)]
fn set_color(color: (u8, u8, u8)) {
    print!("\x1b[");
}

fn main() {
    println!("{}{}{}{}{}{}{}{}", " ".white_back(), " ".white_back(), " ".cyan_back(), " ".cyan_back(), " ".magenta_back(), " ".magenta_back(), " ".yellow_back(), " ".yellow_back());

    println!("{}{}{}{}{}{}{}{}", " ".cyan_back(), " ".cyan_back(), " ".white_back(), " ".white_back(), " ".yellow_back(), " ".yellow_back(), " ".magenta_back(), " ".magenta_back());

}

