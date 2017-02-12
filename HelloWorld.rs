fn main()
{
    print!("Hello, World, ");
    println!("I'm a Rustacean!");
    println!("My Name is {name}",
             name = "Owen");
    println!("Pi is roughly {pi_exact:.precision$}",
             pi_exact = 3.141592,
             precision = 3)
}
