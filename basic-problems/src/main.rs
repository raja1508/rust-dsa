use std::io;


fn main(){
    hello_function();
    print_factors();
    print_factors_reverse();
    check_prime();
    print_prime();
    count_zeros();
}

fn count_zeros() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u32 = input.trim().parse().unwrap(); 

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

    for i in 2..n {
        if is_prime(i){
            print!("{} ", i)
        }
    }

}

fn is_prime(n: u32) -> bool {
    let mut idx = 2u32;
    let mut is_prime = true; 

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

