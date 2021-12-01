fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut list: Vec<i32> = Vec::new();
    for arg in args.iter().skip(1) {
        list.push(arg.parse().unwrap());
    }
    let mut count = 0;
    let mut sum = 0;
    for i in 0..list.len() {
        if i + 2 < list.len() {
            if sum < list[i] + list[i + 1] + list[i + 2] {
                count += 1;
            }
            sum = list[i] + list[i + 1] + list[i + 2];
        }
    }
    println!("{}", count);
}
