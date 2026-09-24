use std::{collections::HashMap, io};

fn main(){
    // sum();
    // interval_sum();
    counting_intervals();
    // count_factors();
    // am_i_prime();
    // count_prime();
    // print_factors();
    // Kth_factors();

}

fn Kth_factors(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let k: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut i: u64 = 2; 

    let mut v = Vec::new(); 
    let mut map: HashMap<u64, u64> = HashMap::new(); 
    v.push(1);

    while i.pow(2) <= n {
        if i.pow(2) == n {
            v.push(i);
            break; 
        }else if n % i == 0 {
            v.push(i);
            map.insert(i, n / i); 
        }
        i += 1; 
    }

    for i in (1..v.len()).rev() {
        if let Some((_, value)) = map.get_key_value(&v[i]) {
            v.push(*value);
        }
    }

    if n != 1{
        v.push(n);
    }

    if let Some(value) = v.get((k - 1) as usize){
        print!("{}", value); 
    }else {
        print!("{}", -1); 
    }
}

fn print_factors(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap(); 

    let mut i: u64 = 2; 

    let mut v = Vec::new(); 
    let mut map: HashMap<u64, u64> = HashMap::new(); 
    v.push(1);

    while i.pow(2) <= n {
        if i.pow(2) == n {
            v.push(i);
            break; 
        }else if n % i == 0 {
            v.push(i);
            map.insert(i, n / i); 
        }
        i += 1; 
    }

    for i in (1..v.len()).rev() {
        if let Some((_, value)) = map.get_key_value(&v[i]) {
            v.push(*value);
        }
    }

    if n != 1{
        v.push(n);
    }

    for num in v {
        print!("{} ", num); 
    }
}

fn count_prime(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap(); 
    let mut count = 0; 

    for i in 1..=n {
        let is_prime = is_prime(i); 
        if is_prime {
            count += 1; 
        }else{
            continue;
        }
    }

    println!("{}", count)
}

fn is_prime(n: u64) -> bool{
    if n == 1 {
        return false; 
    }else if n == 2 {
        return true;
    }

    let mut is_prime  = true; 

    let mut i: u64 = 2; 

    while i.pow(2) <= n {

        if i.pow(2) == n {
            is_prime = false;  
            break; 
        }else if n % i == 0 {
            is_prime = false; 
        }

        i += 1; 
    }

    is_prime
}

fn am_i_prime(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap(); 

    if n == 1 {
        println!("{}", "NO"); 
        return;
    }else if n == 2 {
        println!("{}", "YES"); 
        return; 
    }

    let mut is_prime  = true; 

    let mut i: u64 = 2; 

    while i.pow(2) <= n {

        if i.pow(2) == n {
            is_prime = false;  
            break; 
        }else if n % i == 0 {
            is_prime = false; 
        }

        i += 1; 
    }

    if is_prime {
        println!("YES")
    }else {
        println!("NO")
    }
}

fn count_factors(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap(); 

    if n == 1 {
        println!("{}", 1); 
        return;
    }

    let mut count = 2; 

    let mut i: u64 = 2; 

    while i.pow(2) <= n {

        if i.pow(2) == n {
            count += 1; 
            break; 
        }else if n % i == 0 {
            count += 2; 
        }

        i += 1; 
    }

    println!("{}", count); 
}

fn counting_intervals(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t: u32 = input.trim().parse().unwrap(); 

    for _ in 1..=t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let T: u8 = parts.next().unwrap().parse().unwrap();  
        let L: i64 = parts.next().unwrap().parse().unwrap(); 
        let R: i64 = parts.next().unwrap().parse().unwrap(); 

        if R >= L {   
            let count = match T {
                1 => if R == L {
                    0
                }else {
                    R - L - 1
                },
                2 => R - L,
                3 => R - L,
                4 => if R == L {
                    1
                }else {
                    R - L + 1
                },
                _ => 0
            }; 
            println!("{}", count);  
        }else {
            println!("{}", 0); 
        }
    }
}

fn interval_sum(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t: u32 = input.trim().parse().unwrap(); 

    for _ in 1..=t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let L: u64 = parts.next().unwrap().parse().unwrap(); 
        let R: u64 = parts.next().unwrap().parse().unwrap(); 

        let sum1: u64 = ((L - 1) * (L)) / 2; 
        let sum2: u64 = (R * ( R + 1)) / 2 ; 

        let sum = sum2 - sum1; 
        println!("{}", sum);  
    }
}

fn sum(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap(); 

    let sum = (n * (n + 1)) / 2 ; 

    print!("{}", sum);  
}