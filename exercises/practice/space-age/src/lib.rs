const SECONDS_IN_YEAR: f64 = 31557600.0;

pub enum PlanetScale {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}
impl PlanetScale {
    fn value(&self) -> f64 {
        match *self {
            PlanetScale::Mercury => 0.2408467,
            PlanetScale::Venus => 0.61519726,
            PlanetScale::Earth => 1.0,
            PlanetScale::Mars => 1.8808158,
            PlanetScale::Jupiter => 11.862615,
            PlanetScale::Saturn => 29.447498,
            PlanetScale::Uranus => 84.016846,
            PlanetScale::Neptune => 164.79132,
        }
    }
}

#[derive(Debug)]
pub struct Duration(f64);
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / SECONDS_IN_YEAR)
    }
}

pub trait Planet {
    const SCALE: PlanetScale;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::SCALE.value()
    }
}

macro_rules! struct_builder {
    ($name: ident) => {
        pub struct $name;
        impl Planet for $name {
            const SCALE: PlanetScale = PlanetScale::$name;
        }
    };
}

struct_builder!(Mercury);
struct_builder!(Venus);
struct_builder!(Earth);
struct_builder!(Mars);
struct_builder!(Jupiter);
struct_builder!(Saturn);
struct_builder!(Uranus);
struct_builder!(Neptune);