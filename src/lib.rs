pub mod global {
    // Turn String i.e. "1.73" into a float 1.73
    pub fn pars(s: String) -> f32 {
        return s.trim().parse().unwrap()
    }
    
    // Convert cm to m
    pub fn to_meters(h: f32) -> f32 {
        return h / 10000.0;
    }
}
