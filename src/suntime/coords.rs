use std::fmt::Display;

pub struct Coords {
    pub lat: f64,
    pub lon: f64,
}

impl Coords {
    pub fn new(lat: f64, lon: f64) -> Self {
        Coords { lat: lat, lon: lon }
    }

    pub fn zero() -> Self {
        Coords { lat: 0.0, lon: 0.0 }
    }
    
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lat: {:.8}, lon: {:.8}", self.lat, self.lon)
    }
}
