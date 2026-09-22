use std::{io, ops::Mul};

fn main(){
    hello_function();
    print_factors();
    print_factors_reverse();
    check_prime();
    print_prime();
    count_zeros();
    find_hcf();
    find_nCr();
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1; 
    }

    let mut fact = 1; 

    for i in 1..=n {
        fact *= i; 
    }

    return fact; 
}

fn find_nCr(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let  n: u32 = parts.next().unwrap().parse().unwrap();
    let  r: u32 = parts.next().unwrap().parse().unwrap();

    let nCr = factorial(n).checked_div(factorial(r).mul(factorial(n - r))).unwrap(); 
    print!("{}", nCr); 


}


fn find_hcf(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let mut a: u32 = parts.next().unwrap().parse().unwrap();
    let mut b: u32 = parts.next().unwrap().parse().unwrap();

    if a == 0 {
        print!("{}", b); 
        return 
    }

    while b % a != 0 {
        let temp = b % a ; 
        b = a ; 
        a = temp ; 

    }

    println!("{}", a)
}

fn count_zeros() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u64 = input.trim().parse().unwrap(); 

    let mut count = 0u32;

    if n == 0 {
        count += 1; 
    }

    while n > 0 {
        if  n % 10 == 0 {
            count += 1; 
        }
        n = n / 10; 
    }

    print!("{}", count)
}

fn print_prime() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap(); 

    for i in 2..=n {
        if is_prime(i){
            print!("{} ", i)
        }
    }

}

fn is_prime(n: u32) -> bool {
    let mut idx = 2u32;
    let mut is_prime = true;

    if n == 1 {
        is_prime = false; 
    } 

    while idx < n {
        if  n % idx == 0 {
            is_prime = false; 
            break;
        }
        idx +=1 ;
    }

    return is_prime; 
}

fn check_prime() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap(); 

    if is_prime(n) {
        print!("Prime")
    }else {
        print!("Not Prime")
    }
}

fn print_factors_reverse() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap(); 

    let mut idx = n;  

    while idx > 0 {
        if  n % idx == 0 {
            print!("{} ", idx)
        }
        idx -=1 ;
    }
}


fn print_factors() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap(); 

    let mut idx = 1u32;  

    while idx <= n {
        if  n % idx == 0 {
            print!("{} ", idx)
        }
        idx +=1 ;
    }
}

fn hello_function(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap(); 

    for _ in 1..=n {
        println!("I am learning functions")
    } 
}

