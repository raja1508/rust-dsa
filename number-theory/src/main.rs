use std::io;

fn main() {
    // factors_of_all_numbers();
    // factors_count_queries();
    // print_prime_from_1_to_n();
    // prime_queries();
    good_number_queries();
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