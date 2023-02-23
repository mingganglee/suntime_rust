use crate::suntime::{coords::Coords, datetime::DateTime, SunTime, SunTimeArgs};
mod suntime;

fn main() {
    let lat: f64 = 39.79066082241891;
    let lon: f64 = 116.43802043961463;

    let mut sunrise: SunTime = SunTime::calc_sun_time(SunTimeArgs {
        coords: Coords::new(lat, lon),
        is_rise_time: true,
        datetime: DateTime::now(),
        zenith: 90.8,
    });

    let mut sunset: SunTime = SunTime::calc_sun_time(SunTimeArgs {
        coords: Coords::new(lat, lon),
        is_rise_time: false,
        datetime: DateTime::now(),
        zenith: 90.8,
    });

    // set timezone
    sunrise.datetime.hour = (sunrise.datetime.hour + 8) % 24;
    sunset.datetime.hour = (sunset.datetime.hour + 8) % 24;

    println!("sunrise: {}", sunrise);
    println!("sunset : {}", sunset);
}
