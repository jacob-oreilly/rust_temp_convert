use std::io::stdin;

fn main() {
    let mut final_temp:f32 = 0.0;
    //formula for converstion
    println!("If you want to convert Celsius to Fahrenheit type F otherwise type C");
    let mut response = String::new();
    stdin() 
        .read_line(&mut response)
        .expect("Failed to recieve input.");
    response = response.trim().to_string();
    if response.to_lowercase() == "c" {
        println!("Enter your temp for {response}");
        response.clear();
        stdin()
            .read_line(&mut response)
            .expect("Failed to recieve input.");
        let your_temp = response.trim().parse().expect("Not a valid float");
        final_temp = convert_to_celsius(your_temp);
    }
    else if response.to_lowercase() == "f" {
        println!("Enter your temp for {response}");
        response.clear();
        stdin()
            .read_line(&mut response)
            .expect("Failed to recieve input.");
        let your_temp = response.trim().parse().expect("Not a valid float");
        final_temp = convert_to_fahrenheit(your_temp);
    }
    println!("Your converted temp: {final_temp}");
 
}

fn convert_to_celsius(fahrenheit_temp: f32) -> f32 {
    let five_over_nine = 5.0/9.0;
    (fahrenheit_temp - 32.0) * five_over_nine
}

fn convert_to_fahrenheit(celsius_temp: f32) -> f32 {
    let nine_over_five:f32 = 9.0/5.0;
    celsius_temp * nine_over_five + 32.0
}