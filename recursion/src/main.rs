use std::io;

fn main() {
    let mut input=   String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap(); 
    print_decreasing(n);
}

fn print_decreasing(n: u64){
    if n == 1 {
        println!("{}", 1); 
        return; 
    }
    println!("{}", n); 
    print_decreasing(n - 1);
}