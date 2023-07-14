
fn main() {
    let body = Body::new(163.0, 75.2, "田中");
    body.print_result();
}

struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange {
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange{ min, max, label: label.to_string() }
    }
    fn test(&self, v: f64) -> bool {
        (self.min <= v) && (v < self.max)
    }
}


struct Body {
    height: f64,
    weight: f64,
    name: String,
}
impl Body {
    fn new(height: f64, weight: f64, name:&str) -> Self {
        Body{ height, weight, name: name.to_string() }
    }
    fn clac_bmi(&self) -> f64 {
        self.weight / (self.height / 100.0).powf(2.0)
    }
}
