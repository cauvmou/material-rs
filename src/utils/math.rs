pub fn lerp(v1: f64, v2: f64, f: f64) -> f64 {
    v1 + (v2 - v1) * f
}

pub fn matrix_multiply(row: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
    let a = row[0] * matrix[0][0] + row[1] * matrix[0][1] + row[2] * matrix[0][2];
    let b = row[0] * matrix[1][0] + row[1] * matrix[1][1] + row[2] * matrix[1][2];
    let c = row[0] * matrix[2][0] + row[1] * matrix[2][1] + row[2] * matrix[2][2];
    [a, b, c]
}

pub fn sanitize_degrees_double(mut degrees: f64) -> f64 {
    degrees = degrees % 360.0;
    if degrees < 0.0 {
        degrees = degrees + 360.0;
    }
    degrees
}
