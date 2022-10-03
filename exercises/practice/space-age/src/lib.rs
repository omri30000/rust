const SECONDS_IN_YEAR: f64 = 31557600.0;

enum PlanetScale {
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
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.0 / PlanetScale::Mercury.value()
    }
}
pub struct Venus;
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.0 / PlanetScale::Venus.value()
    }
}
pub struct Earth;
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.0 / PlanetScale::Earth.value()
    }
}
pub struct Mars;
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.0 / PlanetScale::Mars.value()
    }
}
pub struct Jupiter;
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.0 / PlanetScale::Jupiter.value()
    }
}
pub struct Saturn;
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.0 / PlanetScale::Saturn.value()
    }
}
pub struct Uranus;
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.0 / PlanetScale::Uranus.value()
    }
}
pub struct Neptune;
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.0 / PlanetScale::Neptune.value()
    }
}
