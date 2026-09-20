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

    let a = parts.next().unwrap().parse().unwrap(); 
    let b = parts.next().unwrap().parse().unwrap(); 
    let c = parts.next().unwrap().parse().unwrap(); 

    min_and_max(a, b, c);
    
    // rect(a, b); 
    calculator(10i64.pow(9), 10i64.pow(9));
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n = input.trim().parse().unwrap(); 
    table(n);
}


fn find_location(x: i64, y: i64) {
    if x == 0 && y == 0 {println!("Origin")}
    if x == 0 && y != 0 {println!("Y axis")}
    if x != 0 && y == 0 {println!("X axis")}
    if x > 0 && y > 0 {println!("1st Quadrant")}
    if x > 0 && y < 0 {println!("4th Quadrant")}
    if x < 0 && y > 0 {println!("2nd Quadrant")}
    if x < 0 && y < 0 {println!("3rd Quadrant")}
}

fn evaluator(marks: u32) {
    if !(marks <= 100){
        return 
    }

    if marks > 90 {
        println!("Excellent")
    } 
    else if marks > 80  {
        println!("Good")
    }else if marks > 70 {
        println!("Fair")
    }else if marks > 60 {
        println!("Meets Expectations")
    }else {
        println!("Below Par")
    }
    
}

fn min_and_max(a: i64, b: i64, c: i64) {
    let mut max; 
    let mut min; 
    if a > b {
        max = a; 
        min = b ; 
    }else {
        max = b; 
        min = a 
    }

    if c > max {
        max = c; 
    } 

    if c < min {
        min = c; 
    }

    println!("Min = {}", min); 
    println!("Max = {}", max);   
}

fn is_passed(marks: i64) {
    if marks >= 35 {
        println!("Pass")
    }else {
        println!("Fail")
    }
}

fn is_multiple(a: i64, b: i64) {
    if b % a == 0 {
        println!("Yes"); 
    }else{
        println!("No")
    }
}


fn even_or_odd(n: u32){
    if is_even(n) {
        println!("Even")
    }else{
        println!("Odd")
    }
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn last_digits_sum(a: i64, b: i64) {
    let last1 = a % 10; 
    let last2 = b % 10; 
    println!("{}", last1 + last2)
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