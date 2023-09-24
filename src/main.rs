use std::io;
use std::env;

// Convert cm to m
pub fn to_meters(h: f32) -> f32 {
    return h / 100.0;
}

// BMI formula
pub fn calc_bmi(h: f32, w: f32) -> f32 {
    return w / (h * h) ;
}
pub fn do_bmi() -> f32 {
    // Height
    println!("What is your height in centimeters?");
    // Get height input
    let mut height = String::new();
    io::stdin().read_line(&mut height)
        .expect("Error reading input");
    let mheight: f32 = to_meters(height.trim().parse().unwrap()); // Convert from string to number

    // Weight
    println!("How much do you weight in kilograms?");
    // Get weight input
    let mut weight = String::new();
    io::stdin().read_line(&mut weight)
        .expect("Error reading input");
    // Convert input to number
    let mweight: f32 = weight.trim().parse().unwrap(); // Convert from string to number

    // BMI
    let bmi = calc_bmi(mheight, mweight);
    return bmi;

}

// Classification
pub fn classification(b: f32) {
    if b < 18.5 {
        println!("According to your BMI score you are underweight.")
    } else if b > 18.5 && b < 25.0 {
        println!("According to your BMI score you have a healthy weight.")
    } else if b > 25.0 && b < 30.0 {
        println!("According to your BMI score you are overweight.")
    } else {
        println!("According to your BMI score you are extremely overweight.")
    }
}

fn main() {
    let bmi = do_bmi();
    let args: Vec<String> = env::args().collect();

    // Calculate Calorie consumption
    if args[1] == "c" {
        println!("Your BMI score is {}", bmi);
        println!("You chose {} days.", args[2]);
    // Calculate BMI
    } else if args[1] == "b" {
        println!("Your BMI score is {}", bmi);
        classification(bmi);
    // Print help
    } else {
        println!("Print help.");
    }
}
