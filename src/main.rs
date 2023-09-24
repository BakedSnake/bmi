use std::io;
use std::env;

// Lifestyle
pub fn get_lstyle(l: String) -> f32 {
    if l == "sedentary" {
        return 1.2; 
    } else if l == "light-active" {
        return 1.375;
    } else if l == "active" {
        return 1.55;
    } else if l == "very-active" {
        return 1.725;
    } else if l == "super-active"{
        return 1.9;
    } else {
        return 0.0;
    }
}

// BMR Formula
pub fn male_bmr(w: f32, h: f32, a: f32) -> f32 {
    return 10.0 * w + 6.25 * h - 5.0 * a + 5.0;
}
pub fn female_bmr(w: f32, h: f32, a: f32) -> f32 {
    return 10.0 * w + 6.25 * h - 5.0 * a - 161.0;
}

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
    let args: Vec<String> = env::args().collect();

    // Calculate Calorie consumption
    if args[1] == "c" {
        if args[5] == "male" {
            let a2 = args[2].clone() + ".0";
            let a3 = args[3].clone() + ".0";
            let a4 = args[4].clone() + ".0";
            let bmr = male_bmr(a3.trim().parse().unwrap(), a2.trim().parse().unwrap(), a4.trim().parse().unwrap());
            let cals = bmr * get_lstyle(args[6].clone());
            println!("To reach your desired weight you need {} calories daily.", cals.ceil());
        } else if args[5] == "female" {
            let bmr = female_bmr(args[3].trim().parse().unwrap(), args[2].trim().parse().unwrap(), args[4].trim().parse().unwrap());
            let cals = bmr * get_lstyle(args[6].clone());
            println!("To reach your desired weight you need {} calories daily.", cals.ceil());
        }
    // Calculate BMI
    } else if args[1] == "b" {
        let bmi = do_bmi();
        println!("Your BMI score is {}", bmi);
        classification(bmi);
    // Print help
    } else {
        println!("Print help.");
    }
}
