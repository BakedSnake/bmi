use std::io;
use std::env;

// Turn String i.e. "1.73" into a float 1.73
pub fn pars(s: String) -> f32 {
    return s.trim().parse().unwrap()
}

// Lifestyle
pub fn get_lstyle(l: &str) -> f32 {
    return match l {
        "sedentary" => 1.2,
        "light-active" => 1.375,
        "active" => 1.55,
        "very-active" => 1.725,
        "super-active" => 1.9,
        _ => 0.0,
    };
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
    return h / 10000.0;
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

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // Calculate Calorie consumption
    if args[1] == "-c" || args[1] == "-cal" {
        for i in 2..3 { args[i] = args[i].clone() + ".0"; }
        if args[5] == "male" {
            let bmr = male_bmr(pars(args[3].to_string()), pars(args[2].to_string()), pars(args[4].to_string()));
            let cals = bmr * get_lstyle(&args[6].clone());
            println!("To reach your desired weight you need {} calories daily.", cals.ceil());
        } else if args[5] == "female" {
            let bmr = female_bmr(pars(args[3].to_string()), pars(args[2].to_string()), pars(args[4].to_string()));
            let cals = bmr * get_lstyle(&args[6].clone());
            println!("To reach your desired weight you need {} calories daily.", cals.ceil());
        }
    // Calculate BMI
    } else if args[1] == "-b" || args[1] == "-bmi" {
        let bmi = do_bmi();
        println!("Your BMI score is {}", bmi);
        classification(bmi);
    // Print help
    } else {
        println!("");
        println!("Usage:");
        println!("------");
        println!("  bmi     -h, --help  Print help.");
        println!("  bmi     -b, --bmi   Calculate bmi.");
        println!("  bmi     -c, --cal   Calculate calorie intake.");
        println!("");
    }
}
