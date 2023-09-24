use std::env;
use bmi::global::pars;
mod class;
use class::{do_bmi, classification};
mod bmr;
use bmr::{male_bmr, female_bmr, get_lstyle};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // Calculate Calorie consumption
    if args[1] == "-c" || args[1] == "--cal" {
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
    } else if args[1] == "-b" || args[1] == "--bmi" {
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
