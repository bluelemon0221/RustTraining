
struct Body {
    height: f64,
    weight: f64,
}

impl Body {
    fn clac_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }
    fn clac_per(&self) -> f64 {
        self.clac_bmi() / 22.0 * 100.0
    }
}

fn main() {

    let taro = Body{height: 160.0, weight:70.0};
    println!("BMI={:.2}", taro.clac_bmi());
    println!("乖離率={:.1}%", taro.clac_per())
}