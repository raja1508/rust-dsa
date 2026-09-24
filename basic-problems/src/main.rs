use std::io;

fn main() {
    // range_sum_query();
    // even_sum_query();
    // count_vowels();
    range_sum_of_squares();
}

fn range_sum_of_squares(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  

    let n: u32 = parts.next().unwrap().parse().unwrap(); 

    let q: u32 = parts.next().unwrap().parse().unwrap();
    
    let mut v: Vec<i64> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }
    
    let mut pre_sum = 0; 

    for i in 0..v.len(){
        let curr = v[i]; 
        pre_sum += curr.pow(2); 
        v[i] = pre_sum; 
    }


    for _ in 0..q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();    
        let mut parts = input.split_whitespace();

        let l: u32 =   parts.next().unwrap().parse().unwrap(); 
        let r: u32 = parts.next().unwrap().parse().unwrap(); 

        let mut ans = 0; 
        if l > 1 {
            ans = v[r as usize - 1] - v[l as usize - 2]; 
        }else {
            ans = v[r as usize - 1]
        }
            
        println!("{}", ans); 
    } 
}

fn count_vowels(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n: u32 = input.trim().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let word : String = input.trim().parse().unwrap(); 
    
    let word_array: Vec<char> = word.chars().collect(); 
    
    let mut v: Vec<i64> = vec![0; word_array.len()]; 

    let mut count = 0; 
    
    for (i, ch) in word_array.iter().enumerate() {
        if *ch == 'a' || *ch == 'e' || *ch == 'i' || *ch =='o' || *ch == 'u' {
            count += 1; 
        }

        v[i] = count;
    }
    

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let q: u32 = input.trim().parse().unwrap();

    for _ in 0..q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();    
        let mut parts = input.split_whitespace();

        let l: u32 =   parts.next().unwrap().parse().unwrap(); 
        let r: u32 = parts.next().unwrap().parse().unwrap(); 

        let mut ans = 0; 
        if l > 1 {
            ans = v[r as usize - 1] - v[l as usize - 2]; 
        }else {
            ans = v[r as usize - 1]
        }
            
        println!("{}", ans); 
    } 
}

fn even_sum_query(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n: u32 = input.trim().parse().unwrap(); 

    let mut v: Vec<i64> = Vec::new(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace();  

    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }
    
    let mut pre_sum = 0; 

    for i in 0..v.len(){
        if i % 2 != 0 {
            let curr = v[i]; 
            pre_sum += curr; 
        } 

        v[i] = pre_sum; 
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let q: u32 = input.trim().parse().unwrap();

    for _ in 0..q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();    
        let mut parts = input.split_whitespace();

        let l: u32 =   parts.next().unwrap().parse().unwrap(); 
        let r: u32 = parts.next().unwrap().parse().unwrap(); 

        let mut ans = 0; 
        if l > 1 {
            ans = v[r as usize - 1] - v[l as usize - 2]; 
        }else {
            ans = v[r as usize - 1]
        }
            
        println!("{}", ans); 
    } 
}

fn range_sum_query(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n: u32 = input.trim().parse().unwrap(); 

    let mut v: Vec<i64> = Vec::new(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace();  

    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }
    
    let mut pre_sum = 0; 

    for i in 0..v.len(){
        let curr = v[i]; 
        pre_sum += curr; 
        v[i] = pre_sum; 
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let q: u32 = input.trim().parse().unwrap();

    for _ in 0..q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();    
        let mut parts = input.split_whitespace();

        let l: u32 =   parts.next().unwrap().parse().unwrap(); 
        let r: u32 = parts.next().unwrap().parse().unwrap(); 

        let mut ans = 0; 
        if l > 1 {
            ans = v[r as usize - 1] - v[l as usize - 2]; 
        }else {
            ans = v[r as usize - 1]
        }
            
        println!("{}", ans); 
    } 
}