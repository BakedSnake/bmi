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
