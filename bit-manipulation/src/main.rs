use std::io;

fn main() {
    // println!("Hello, world!");
    powers_of_two();
}


fn powers_of_two(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap(); 

    let mut i = 0; 
    while 1<<i <= n {
        print!("{} ", 1<<i); 
        i += 1; 
    }
}