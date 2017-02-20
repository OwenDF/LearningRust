use std::fmt::{self, Formatter, Display};

struct City
{
    name: &'static str,
    lat: f32,
    long: f32,
}

impl Display for City
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let long_c = if self.long >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.long.abs(), long_c)
    }
}

#[derive(Debug)]
struct Color
{
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let red_hex = self.red;
        let green_hex = self.green;
        let blue_hex = self.blue;

        write!(f, "RGB ({}, {}, {}) 0x{:02.X}{:02.X}{:02.X}",
               self.red, self.green, self.blue, red_hex, green_hex, blue_hex)
    }
}

fn main()
{
    for city in [
        City { name: "Dublin", lat: 53.347778, long: -6.259722 },
        City { name: "Oslo", lat: 59.95, long: 10.75 },
        City { name: "Vancouver", lat: 49.25, long: -123.1 },
    ].iter()
    {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color)
    }
}
