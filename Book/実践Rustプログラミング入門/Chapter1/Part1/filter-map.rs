fn main() {
    let source = vec![1,2,3,4,5];
    let result = source
        .into_iter()
        .filter(|n| n % 2 ==0)
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", result);
}