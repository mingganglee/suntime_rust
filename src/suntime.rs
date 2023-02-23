use std::default::Default;
use std::f64::consts::PI;
use std::fmt::Display;

use self::coords::Coords;
use self::datetime::DateTime;

pub mod coords;
pub mod datetime;

pub struct SunTimeArgs {
    pub coords: Coords,
    pub is_rise_time: bool,
    pub datetime: DateTime,
    pub zenith: f64,
}

impl Default for SunTimeArgs {
    fn default() -> Self {
        SunTimeArgs {
            coords: Coords::zero(),
            is_rise_time: false,
            datetime: DateTime::now(),
            zenith: 90.8,
        }
    }
}

pub struct SunTime {
    pub decimal: f64,
    pub datetime: DateTime,
}

impl Default for SunTime {
    fn default() -> Self {
        SunTime {
            decimal: 0.0,
            datetime: DateTime::now(),
        }
    }
}

impl SunTime {
    pub fn calc_sun_time(sun_time_args: SunTimeArgs) -> Self {
        let coords: Coords = sun_time_args.coords;
        let is_rise_time: bool = sun_time_args.is_rise_time;
        let date: DateTime = sun_time_args.datetime;
        let zenith: f64 = sun_time_args.zenith;

        let to_rad: f64 = PI / 180.0;

        // 1. first calculate the day of the year
        let n1: f64 = (275 * date.month / 9) as f64;
        let n2: f64 = ((date.month + 9) / 12) as f64;
        let n3: f64 = (1 + (date.year - 4 * (date.year / 4) + 2) / 3) as f64;
        let n: f64 = n1 - (n2 * n3) + date.day as f64 - 30.0;

        // 2. convert the longitude to hour value and calculate an approximate time
        let lng_hour: f64 = coords.lon / 15.0;

        let t: f64;
        if is_rise_time {
            t = n + ((6.0 - lng_hour) / 24.0)
        } else {
            t = n + ((18.0 - lng_hour) / 24.0)
        }

        // 3. calculate the Sun's mean anomaly
        let m = (0.9856 * t) - 3.289;

        // 4. calculate the Sun's true longitude
        let mut l: f64 =
            m + (1.9616 * (to_rad * m).sin()) + (0.020 * (to_rad * 2.0 * m).sin()) + 282.634;
        // NOTE: L adjusted into the range [0, 360]
        l = Self::force_range(l, 360.0);

        // 5a. calculate the Sun's right ascension
        let mut ra: f64 = (1.0 / to_rad) * (0.91764 * (to_rad * l).tan()).atan();
        // NOTE: RA adjusted into the range [0, 360]
        ra = Self::force_range(ra, 360.0);

        // 5b. right ascnsion value needs to be in the same quadrant as L
        let lquadrant: f64 = (l / 90.0).floor() * 90.0;
        let raquadrant: f64 = (ra / 90.0).floor() * 90.0;
        ra = ra + (lquadrant - raquadrant);

        // 5c. right ascension value needs to be converted into hours
        ra = ra / 15.0;

        // 6. calculate the Sun's declination
        let sin_dec: f64 = 0.39782 * (to_rad * l).sin();
        let cos_dec: f64 = sin_dec.asin().cos();

        // 7a. calculate the Sun's local hour angle
        let cos_h = ((to_rad * zenith).cos() - (sin_dec * (to_rad * coords.lat).sin()))
            / (cos_dec * (to_rad * coords.lat).cos());

        if cos_h > 1.0 {
            return SunTime {
                decimal: todo!(),
                datetime: todo!(),
            };
        } else if cos_h < -1.0 {
            return SunTime {
                decimal: todo!(),
                datetime: todo!(),
            };
        }

        // 7b. finish calculation H and convert into hours
        let mut h: f64;
        if is_rise_time {
            h = 360.0 - (1.0 / to_rad) * cos_h.acos();
        } else {
            h = 1.0 / to_rad * cos_h.acos();
        }
        h = h / 15.0;

        // 8. calculate local mean time of rising/setting
        let t = h + ra - (0.06571 * t) - 6.622;

        // 9. adjust back to UTC
        let mut ut = t - lng_hour;
        // UTC time in decimal format (e.g. 23.23)
        ut = Self::force_range(ut, 24.0);

        // 10. return
        let hour: f64 = Self::force_range(ut, 24.0);
        let minute: f64 = (ut - ut.floor()) * 60.0;
        let second: f64 = (minute - minute.floor()) * 60.0;

        let result: SunTime = SunTime {
            decimal: ut,
            datetime: DateTime {
                year: date.year,
                month: date.month,
                day: date.day,
                hour: hour as u32,
                minute: minute as u32,
                second: second as u32,
            },
        };

        return result;
    }

    fn force_range(v: f64, max: f64) -> f64 {
        // force v to be >= 0 and < max
        if v < 0.0 {
            return v + max;
        } else if v >= max {
            return v - max;
        }
        return v;
    }
}

impl Display for SunTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.decimal, self.datetime)
    }
}
