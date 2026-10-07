use std::io;

fn main() {
    // powers_of_two();
    // breakdown_n();
    decimal_to_binary();
}

fn binary_to_decimal(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut n: u64 = input.trim().parse().unwrap();

    let mut i: u64 = 0; 
    while 1u64<<i <= n {
        i += 1; 
    }
    if i == 0 {
        println!("{}", i); 
        return; 
    }
    i -= 1; 

    let mut binary = String::new(); 
    for k in (0..=i).rev() {
        let mask = 1u64 << k; 
        if mask <= n {
            n -= mask; 
            binary.push('1');      
        }else {
            binary.push('0');
        }
    }
    println!("{}", binary)
}

fn decimal_to_binary(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut n: u64 = input.trim().parse().unwrap();

    let mut i: u64 = 0; 
    while 1u64<<i <= n {
        i += 1; 
    }
    if i == 0 {
        println!("{}", i); 
        return; 
    }
    i -= 1; 

    let mut binary = String::new(); 
    for k in (0..=i).rev() {
        let mask = 1u64 << k; 
        if mask <= n {
            n -= mask; 
            binary.push('1');      
        }else {
            binary.push('0');
        }
    }
    println!("{}", binary)
}

fn breakdown_n(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut n: u64 = input.trim().parse().unwrap();

    let mut i: u64 = 0; 
    while 1u64<<i <= n {
        i += 1; 
    }
    i -= 1; 

    for k in (0..=i).rev() {
        let mask = 1u64<<k; 
        if mask <= n {
            n -= mask; 
            print!("{} ", mask); 
            
        } 
    }
}

fn powers_of_two(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap(); 

    let mut i: u64 = 0; 
    while 1u64<<i <= n {
        print!("{} ", 1u64<<i); 
        i += 1; 
    }
}