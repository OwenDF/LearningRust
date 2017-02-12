use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D
{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main()
{
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name",
             "Bale",
             "Christian",
             actor = "actor's");
    println!("Now {} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    
    println!("The big range is {big}, and the small range is {small}",
             big = big_range,
             small = small_range);

    let point = Point2D {x: 3.7, y: 1.2};
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}
