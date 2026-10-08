use std::io;

fn main() {
    // powers_of_two();
    // breakdown_n();
    // decimal_to_binary();
    // binary_to_decimal();
    // bitwise_operator();
    // bitwise_operator_2();
    bitwise_queries();
}

fn bitwise_queries(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let q: u64 = parts.next().unwrap().parse().unwrap(); 
}

fn bitwise_operator_2(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap(); 

    if n == 1 {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        let a: u64 = input.trim().parse().unwrap(); 
        print!("{0} {0} {0}", a); 
        return;
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();
    
    let mut v = Vec::new(); 
    for _ in 0..n {
        let a: u64 = parts.next().unwrap().parse().unwrap(); 
        v.push(a);
    }

    let mut bitwise_and = v[0] & v[1]; 
    let mut bitwise_or = v[0] | v[1];
    let mut bitwise_xor = v[0] ^ v[1];  
    let mut j = 2; 
    while j < v.len(){
        bitwise_and &= v[j]; 
        bitwise_or |= v[j]; 
        bitwise_xor ^= v[j]; 
        j += 1; 
    }

    print!("{} {} {} ", bitwise_and, bitwise_or, bitwise_xor); 
        
}

fn bitwise_operator(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let a: u64 = parts.next().unwrap().parse().unwrap(); 
    let b: u64 = parts.next().unwrap().parse().unwrap(); 

    print!("{} ", a & b); 
    print!("{} ", a | b); 
    print!("{} ", a ^ b);
}

fn binary_to_decimal(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let binary_str: String = input.trim().parse().unwrap();

    // let binary_chars: Vec<char> = binary_str.chars().collect(); 
    let mut integer: u64 = 0; 

    for (i, binary_) in binary_str.chars().enumerate(){
        if binary_== '1' {
            let k = binary_str.len() - i - 1; 
            let mask = 1 << k; 
            integer += mask; 
        }
    }

    println!("{}", integer)


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