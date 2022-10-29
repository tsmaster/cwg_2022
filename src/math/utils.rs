


pub fn degrees_to_radians(deg: f32) -> f32 {
    std::f32::consts::PI * deg / 180.0
}

pub fn radians_to_degrees(rad: f32) -> f32 {
    180.0 * rad / std::f32::consts::PI
}
