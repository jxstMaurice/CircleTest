use std::f64::consts::PI;
use std::io;
fn main()  {
    let mut input = String::new();

    input = input_number();

    let radius: f64 = input.trim().parse().expect("Please enter a valid number");

    calculate_circle_properties(radius);
    main();
}


fn input_number() -> String {
    let mut input = String::new();
    println!("Please enter the radius of the circle:");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input
}

fn calculate_circle_properties(radius: f64) {
    let circumference = 2.0 * PI * radius;
    let area = PI * radius.powi(2);
    let diameter = 2.0 * radius;

    println!("For a circle with radius {:.2}:", radius);
    println!("Circumference: {:.2}", circumference);
    println!("Area: {:.2}", area);
    println!("Diameter: {:.2}", diameter);

    for _ in 0..3 {
        println!(" ")
    }
}