use std::fmt;

pub struct Satellite {
    pub name: String,
    pub velocity: f64,
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "is a satellite with the name {} and a speed of {}",
            self.name, self.velocity
        )
    }
}
