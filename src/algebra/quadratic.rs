pub fn compute_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let delta = (b*b) - (4.0*a*c);

    if delta < 0.0 {
        (f64::INFINITY, f64::INFINITY)
    } else {
        let t1 = (-b + delta.sqrt()) / (2.0*a);
        let t2 = (-b - delta.sqrt()) / (2.0*a);
        (t1, t2)
    }
}