use std::io;
use bmi::global::{pars, to_meters};

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
    let mheight: f32 = to_meters(pars(height)); // Convert from string to number

    // Weight
    println!("How much do you weight in kilograms?");
    // Get weight input
    let mut weight = String::new();
    io::stdin().read_line(&mut weight)
        .expect("Error reading input");
    // Convert input to number
    let mweight: f32 = to_meters(pars(weight)); // Convert from string to number

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
