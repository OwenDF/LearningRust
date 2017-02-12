extern crate serialize;
use std::fmt::{self, Formatter, Display};
use serialize::hex;

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
        let red_hex = self.red.to_hex();
        let green_hex = self.green.to_hex();
        let blue_hex = self.blue.to_hex();

        write!(f, "RGB ({}, {}, {}) 0x{}{}{}",
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
}
