fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut list: Vec<i32> = Vec::new();
    for arg in args.iter().skip(1) {
        list.push(arg.parse().unwrap());
    }
    let mut count = 0;
    for i in 1..list.len() {
        if list[i] > list[i - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}
