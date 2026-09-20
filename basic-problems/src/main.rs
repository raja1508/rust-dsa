use std::io; 

fn main() {
    for i in 1..=5 {
        for j in 1..=5 {

            if i == 1 || i == 5 || j == 5 + 1 - i {
                print!("{}", "*"); 
            }else{
                print!("{}", " ")
            }

        }
        println!()
    }
    for i in 1..=10 {
        let format = format!("5 * {} = {}", i , 5*i); 
        println!("{}", format); 
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let mut parts = input.split_whitespace(); 

    let l = parts.next().unwrap().parse().unwrap(); 
    let b = parts.next().unwrap().parse().unwrap(); 

    rect(l, b); 
    calculator(10i64.pow(9), 10i64.pow(9));
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n = input.trim().parse().unwrap(); 
    table(n);
}

fn calculator(a: i64, b: i64) {
    let format = format!("{} + {} = {}", a, b , a.checked_add(b).unwrap_or(0)); 
    println!("{}", format);  
    let format = format!("{} - {} = {}", a, b , a.checked_sub(b).unwrap_or(0)); 
    println!("{}", format);  
    let format = format!("{} * {} = {}", a, b , a.checked_mul(b).unwrap_or(0)); 
    println!("{}", format); 
    let format = format!("{} / {} = {}", a, b , a.checked_div(b).unwrap_or(0)); 
    println!("{}", format); 
    let format = format!("{} % {} = {}", a, b , a % b); 
    println!("{}", format); 
}

fn table(n: u32) {
    for i in 1..=10 {
        let format = format!("{} * {} = {}", n, i , n*i); 
        println!("{}", format); 
    }
}

fn rect(l: u32, b: u32) {
    let area = l.checked_mul(b).unwrap_or(0); 
    let perimeter = 2u32 * l.checked_add(b).unwrap_or(0);
    println!("Area = {}", area); 
    println!("Perimeter = {}", perimeter); 
}