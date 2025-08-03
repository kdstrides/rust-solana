use std::io;

fn main() {
  println!("Temperature Converter");
  println!("1: Fahrenheit to Celsius");
  println!("2: Celsius to Fahrenheit");
  println!("Please enter your choice: ");

  let mut choice = String::new();
  io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read line");

  let choice: i32 = match choice.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid choice");
      return;
    }
  };

  if choice == 1 {
    fahrenheit_to_celsius();
  } else if choice == 2 {
    celsius_to_fahrenheit();
  } else {
    println!("Invalid choice");
  }
}

fn fahrenheit_to_celsius() {
  println!("Enter the temperature in Fahrenheit:");
  let mut fahrenheit = String::new();
  io::stdin()
    .read_line(&mut fahrenheit)
    .expect("Failed to read line");
  let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please type a number!");
  let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
  println!("{fahrenheit}°F is {celsius}°C");
}

fn celsius_to_fahrenheit() {
  println!("Enter the temperature in Celsius:");
  let mut celsius = String::new();
  io::stdin()
    .read_line(&mut celsius)
    .expect("Failed to read line");
  let celsius: f64 = celsius.trim().parse().expect("Please type a number!");
  let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
  println!("{celsius}°C is {fahrenheit}°F");
}
