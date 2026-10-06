use std::{collections::HashSet, io};

fn main() {
    // factors_of_all_numbers();
    // factors_count_queries();
    // print_prime_from_1_to_n();
    // prime_queries();
    // good_number_queries();
    // prime_range_queries();
    // prime_factorisation();
    // unique_prime_factors();
    // prime_factorise_queries();
    almost_prime();
}

fn almost_prime(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap();  
    let mut v: Vec<u64> = vec![0; n as usize + 1]; 

    let mut i: usize = 2; 
    while i < v.len() as usize{
        if v[i] == 0 {
            let mut j = 2*i;
            while j < v.len() as usize {
                v[j] += 1; 
                j += i; 
            } 
        }
            
        i += 1; 
    }

    // println!("{:?}", v); 

    let mut count = 0; 
    for k in 0..v.len(){
        if v[k] == 2{
            count += 1; 
        }
    }

    println!("{}", count)
    
}

fn prime_factorise_queries(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 
    let q: u64 = input.trim().parse().unwrap(); 
    const N: u32 = 10u32.pow(6); 
    let mut v: Vec<u64> = vec![0; N as usize + 1]; 

    let mut i: usize = 2; 

    while i < v.len() as usize{
        if v[i] == 0 as u64{
            let mut j = i;
            while j < v.len() as usize {
                if v[j] == 0 {
                    v[j] = i as u64; 
                }else {
                    if v[j] > i as u64 {
                        v[j] = i as u64
                    }
                }
                j += i; 
            } 
        }
        i += 1; 
    }

    for _ in 0..q{
        let mut input = String::new();  
        io::stdin().read_line(&mut input).unwrap(); 
        let mut t: u64 = input.trim().parse().unwrap(); 
        let mut count: u32 = 0;  
        while t > 1 {
            let p = v[t as usize]; 
            count += 1; 
    
            t /= p; 
            if t % p != 0 {
                print!("{}^{} ", p, count); 
                count = 0; 
            }
        }
        println!()
    }
}

fn unique_prime_factors(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 
    let mut n: u64 = input.trim().parse().unwrap(); 
        // factor out 2

    let mut set = HashSet::new(); 
    while n % 2 == 0 {
        n /= 2;
        set.insert(2); 
    }; 

    // odd divisors only
    let mut p: u64 = 3;
    while p.pow(2) <= n {
        if n % p == 0 {
            set.insert(p); 
            while n % p == 0 {
                n /= p;
            }
        }
        p += 2;
    }

    if n > 1 {
        set.insert(n); 
    }
    println!("{}", set.len());

}

fn prime_factorisation(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 
    let mut n: u64 = input.trim().parse().unwrap(); 
        // factor out 2
    let mut count = 0;
    while n % 2 == 0 {
        n /= 2;
        count += 1;
    }
    if count > 0 {
        print!("2^{} ", count);
    }

    // odd divisors only
    let mut p: u64 = 3;
    while p.pow(2) <= n {
        if n % p == 0 {
            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            print!("{}^{} ", p, count);
        }
        p += 2;
    }

        // leftover is a prime > 1
    if n > 1 {
        print!("{}^1", n);
    }
    println!();
}

fn prime_range_queries(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let n: u64 = parts.next().unwrap().parse().unwrap();  
    let q: u64 = parts.next().unwrap().parse().unwrap(); 

    // const N: u64 = 10u64.pow(6); 
    let mut v: Vec<bool> = vec![true;  n as usize + 1]; 
    v[0] = false; 
    v[1] = false; 

    let mut i: usize = 2; 

    while i.pow(2) <= n as usize{
        if v[i] {
            let mut j = i.pow(2); 
            while j <= n as usize{
                if v[j] {
                    v[j] = false; 
                }
                j += i; 
            }
        }
        i += 1; 
    }

    println!("{:?}", v); 

    let mut v2: Vec<u64> = vec![0;  n as usize]; 
    let mut count = 0; 
    for k in 1..v.len(){
        if v[k]{
            count += 1; 
        }
        v2[k - 1] = count; 
    }

    println!("{:?}", v2); 

    for _ in 0..q {
        let mut input = String::new();  
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let l: u64 = parts.next().unwrap().parse().unwrap();  
        let r: u64 = parts.next().unwrap().parse().unwrap();
        let ans ; 
        if l == 1 {
            ans = v2[r as usize  - 1]; 
        }else{
            ans = v2[r as usize  - 1] - v2[l as usize - 2 ]
        }

        println!("{}", ans); 
    }
}

fn good_number_queries(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 
    let q: u64 = input.trim().parse().unwrap(); 
    const N: u64 = 10u64.pow(6); 
    let mut v: Vec<u64> = vec![0; N as usize + 1]; 

    let mut i:usize = 1; 
    while i <= N as usize{
        let mut j = i; 
        while j <= N as usize{
            v[j] += 1; 
            j += i; 
        }
         i += 1;      
    } 

    let mut v2: Vec<bool> = vec![true;  N as usize + 1]; 
    v2[0] = false; 
    v2[1] = false; 

    let mut i:usize = 2; 

    while i <= N as usize{
        if v2[i] == false{
            i += 1; 
            continue;
        }
        let mut j = i.pow(2); 
        while j <= N as usize{
            if v2[j] == true{
                v2[j] = false; 
            }
            // println!("{:?}", v);
            j += i; 
        }      
        i += 1;
    }

    for _ in 0..q{
        let mut input = String::new();  
        io::stdin().read_line(&mut input).unwrap(); 
        let t: u64 = input.trim().parse().unwrap();  

        let res = v[t as usize]; 
        if v2[res as usize] {
            println!("YES")
        }else{
            println!("NO"); 
        }
    }     
}

fn prime_queries(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 
    let q: u64 = input.trim().parse().unwrap(); 

    const N: u64 = 10u64.pow(6); 
    let mut v: Vec<bool> = vec![true;  N as usize + 1]; 
    v[0] = false; 
    v[1] = false; 

    let mut i:usize = 2; 

    while i <= N as usize{
        if v[i] == false{
            i += 1; 
            continue;
        }
        let mut j = i.pow(2); 
        while j <= N as usize{
            if v[j] == true{
                v[j] = false; 
            }
            // println!("{:?}", v);
            j += i; 
        }      
        i += 1;
    }    

    for _ in 0..q{
        let mut input = String::new();  
        io::stdin().read_line(&mut input).unwrap(); 
        let t: u64 = input.trim().parse().unwrap();
        if v[t as usize] {
            println!("YES")
        }else{
            println!("NO")
        }
    }  
}

fn print_prime_from_1_to_n(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap(); 

    let mut v: Vec<bool> = vec![true; n as usize + 1]; 
    v[0] = false; 
    v[1] = false; 

    let mut i:usize = 2; 
    let mut count = 0; 
    while i <= n as usize{
        if v[i] == false{
            i += 1; 
            continue;
        }
        let mut j = i.pow(2); 
        while j <= n as usize{
            if v[j] == true{
                v[j] = false; 
            }
            // println!("{:?}", v);
            j += i; 
        }

        if v[i] == true{
            count += 1; 
            print!("{} ", i); 
        }

        i += 1;
    }    
    println!(); 
    println!("{}", count);     
}

fn factors_count_queries(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 
    let q: u64 = input.trim().parse().unwrap(); 
    const N: u64 = 10u64.pow(6); 
    let mut v: Vec<u64> = vec![0; N as usize + 1]; 

    let mut i:usize = 1; 
    while i <= N as usize{
        let mut j = i; 
        while j <= N as usize{
            v[j] += 1; 
            j += i; 
        }
         i += 1;      
    } 

    for _ in 0..q{
        let mut input = String::new();  
        io::stdin().read_line(&mut input).unwrap(); 
        let t: u64 = input.trim().parse().unwrap();  

        println!("{} ", v[t as usize]); 
    }   
}

fn factors_of_all_numbers(){
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap(); 

    let n: u64 = input.trim().parse().unwrap(); 
    let mut v: Vec<Vec<u64>> = vec![Vec::new(); n as usize + 1]; 

    let mut i:usize = 1; 
    while i <= n as usize{
        
        let mut j = i; 
        while j <= n as usize{
            v[j].push(i as u64); 
            j += i; 
        }

        for k in 0..v[i].len(){
            print!("{} ", v[i][k])
        }
        i += 1;
        println!() 
        
    }    
}