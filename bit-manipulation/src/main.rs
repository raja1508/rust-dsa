use std::io::{self, BufWriter, Read, Write};

fn main() {
    // powers_of_two();
    // breakdown_n();
    // decimal_to_binary();
    // binary_to_decimal();
    // bitwise_operator();
    // bitwise_operator_2();
    // bitwise_queries();
    // is_power_of_two();
    // odd_or_even_without_modulo();
    // count_set_bits();
    // single_number1();
    // single_number2();
    // print_all_subset();
    print_all_subset_buffer();
}


fn print_all_subset_buffer() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap(); 


    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let mut v: Vec<u64> = Vec::new(); 
    for _ in 0..n {
        let num: u64 = parts.next().unwrap().parse().unwrap(); 
        v.push(num);
    } 

    let stdout = io::stdout();
    let mut out = BufWriter::with_capacity(1 << 16, stdout.lock());

    for mask in 1..(1u64 << n) {
        let mut first = true;
        for i in 0..n {
            if mask & (1u64 << i) != 0 {
                if !first {
                    out.write_all(b" ").unwrap();
                }
                write!(out, "{}", v[i as usize]).unwrap();
                first = false;
            }
        }
        out.write_all(b"\n").unwrap();
    }
}

fn print_all_subset(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap(); 


    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v: Vec<u64> = Vec::new(); 
    for _ in 0..n {
        let num: u64 = parts.next().unwrap().parse().unwrap(); 
        v.push(num);
    } 

    if v.len() == 1 {
        println!("{}", v[0]); 
        return; 
    }

    for j in 1..1u64 << n{
        let mut str = String::new(); 
        let num = j ; 
        for i in 0..n as usize{
            if num & 1u64 << i == 0 {
                continue;
            }else{
                str.push_str(&v[i].to_string());
                str.push(' ');
            }
        }
        println!("{}", str); 
    }
}

fn single_number2(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap(); 


    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v = Vec::new(); 
    for _ in 0..n {
        let num: i64 = parts.next().unwrap().parse().unwrap(); 
        v.push(num);
    } 

    if v.len() == 1 {
        println!("{}", v[0]); 
        return; 
    }

    let mut single = v[0] ^ v[1]; 

    for j in 2..n{
        single ^= v[j as usize]; 
    }
    
    println!("{}", single); 
}

fn single_number1(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap(); 


    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v = Vec::new(); 
    for _ in 0..n {
        let num: i64 = parts.next().unwrap().parse().unwrap(); 
        v.push(num);
    } 

    if v.len() == 1 {
        println!("{}", v[0]); 
        return; 
    }

    let mut single = v[0] ^ v[1]; 

    for j in 2..n{
        single ^= v[j as usize]; 
    }
    
    println!("{}", single); 
}

fn count_set_bits(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u64 = input.trim().parse().unwrap(); 

    if n == 0 {
        println!("0");
        return; 
    }
    
    let mut count = 1; 
    while n & (n - 1) != 0 {
        count += 1; 
        n = n & (n - 1); 
    }

    println!("{}", count); 
}

fn odd_or_even_without_modulo(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap(); 

    if n & 1 << 0 == 0 {
        println!("Even")
    }else {
        println!("Odd")
    }
}

fn is_power_of_two(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap(); 

    if n == 0 {
        println!("NO");
        return; 
    }
    
    if n & (n - 1)  == 0 {
        println!("YES")
    }else {
        println!("NO")
    }
}

fn bitwise_queries(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let mut n: u64 = parts.next().unwrap().parse().unwrap(); 
    let q: u64 = parts.next().unwrap().parse().unwrap();

    for _ in 0..q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        let mut parts = input.split_whitespace(); 
        let t: u64 = parts.next().unwrap().parse().unwrap(); 
        let i: u64 = parts.next().unwrap().parse().unwrap();

        let mask = 1 << i;
        match t {
            1 => {
                if n & mask == 0 {println!("NO")}else{println!("YES")}},
            2 => {n |= mask; println!("{}", n)},
            3 => {n = n & !mask; println!("{}", n)},
            4 => {n = n ^ mask; println!("{}", n)},
            _ => {}
        }
    } 
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