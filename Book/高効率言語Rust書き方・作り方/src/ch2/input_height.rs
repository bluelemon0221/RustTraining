fn main() {
    let mut height;

    loop{
        println!("height?");
        height = input_f(0.0);
        if height > 0.0 { break; }
        println!("正しく入力してください");
    }
    let weight = 22.0 * (height / 100.0 ).powf(2.0);
    println!("標準体重は{:1}kgです。", weight);
}

fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("入力エラー");
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(v) => v,
        Err(__) => def,
    }
}