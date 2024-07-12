#[derive(Clone, Debug, PartialEq)]
pub struct PinholeCamera {
    pub id: u32,
    pub width: u64,
    pub height: u64,
    pub focal_length_x: f64,
    pub focal_length_y: f64,
    pub principal_point_x: f64,
    pub principal_point_y: f64,
}
