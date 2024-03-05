pub enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64),
}

impl Location {
    pub fn display(&self) {
        match *self {
            Location::Unknown => println!("\tUnknown location..."),
            Location::Anonymous => println!("\tAnonymous location ..."),
            Location::Known(x, y) => println!("\tknown location at ({}, {})", x, y),
        }
    }
}
