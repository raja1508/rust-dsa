use std::io;

fn main() {
    // range_sum_query();
    // even_sum_query();
    // count_vowels();
    // range_sum_of_squares();
    // let ans = have_exactly_k_factors(6, 3); 
    // if ans {
    //     print!("YES")
    // }else{
    //     print!("NO")
    // }
    // count_numbers_with_digit_sum_k();
    // count_numbers_with_exactly_k_factors();
    // ** alternating_range_sum();
    // print_all_subarrays();
    // sum_of_all_subarrays_separately();
    // sum_of_all_subarrays_separately_hard();
    // sum_of_all_subarrays_easy();
    // sum_of_all_subarrays_hard();
    maximum_subarray_sum_of_size_k();
    
}

fn maximum_subarray_sum_of_size_k() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let k: u32 = parts.next().unwrap().parse().unwrap();  
    
    let mut v: Vec<i64> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    let mut max_length = 0; 

    for i in 0..v.len() {
        let mut sum = v[i]; 
        let mut count = 1; 
        
        for j in i + 1..v.len() {                                   
            sum += v[j]; 
            count += 1;                                                                             

            if sum as u32 == k {
                if count > max_length {
                    max_length = count; 
                    break; 
                }
            }
        }
    }
    println!("{}", max_length); 
}

fn get_frequency(v: &Vec<i64>, i: usize) -> usize{
    let mut fq = 1; 

    let left_fq = (v.len() - i) * i;
    let righ_fq = v.len() - i  - 1; 

    fq += left_fq; 
    fq += righ_fq;  

    return  fq;
}

fn sum_of_all_subarrays_hard() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  
    let n: u32 = parts.next().unwrap().parse().unwrap(); 
    
    let mut v: Vec<i64> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    let mut sum = 0; 
    for i in 0..v.len() {
        let frequency = get_frequency(&v, i) as i64;  
        sum += v[i] * frequency; 
    }
    println!("{}", sum); 
}

fn sum_of_all_subarrays_easy() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  
    let n: u32 = parts.next().unwrap().parse().unwrap(); 
    
    let mut v: Vec<i64> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    let mut sum = 0; 
    for i in 0..v.len() {
        let mut internal_sum = v[i]; 
        sum += internal_sum; 
        for j in i + 1..v.len() {
            internal_sum += v[j];
            sum += internal_sum;  
        }
    }
    println!("{}", sum); 
}

fn sum_of_all_subarrays_separately_hard() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  
    let n: u32 = parts.next().unwrap().parse().unwrap(); 
    
    let mut v: Vec<i64> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    for i in 0..v.len() {
        let mut sum = v[i]; 
        println!("{}", sum); 
        for j in i + 1..v.len() {
            sum += v[j]; 
            println!("{}", sum); 
        }
    }
}

fn sum_of_all_subarrays_separately() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  
    let n: u32 = parts.next().unwrap().parse().unwrap(); 
    
    let mut v: Vec<u32> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    for i in 0..v.len() {
        let mut  v2 = Vec::new(); 
        v2.push(v[i]);
        println!("{}", v[i]); 
        for j in i + 1..v.len() {
            v2.push(v[j]);
            let mut sum = 0; 
            for k in 0..v2.len(){
                sum += v2[k]; 
            }
            println!("{}", sum); 
        }
    }
}

fn print_all_subarrays() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  
    let n: u32 = parts.next().unwrap().parse().unwrap(); 
    
    let mut v: Vec<u32> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    for i in 0..v.len() {
        let mut  v2 = Vec::new(); 
        v2.push(v[i]);
        for j in i + 1..v.len() {
            for k in 0..v2.len(){
                print!("{} ", v2[k])
            }
            println!(); 
            v2.push(v[j]);
        }
    }
}

fn alternating_range_sum(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  

    let n: u32 = parts.next().unwrap().parse().unwrap(); 

    let q: u32 = parts.next().unwrap().parse().unwrap();
    
    let mut v: Vec<u32> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    for _ in 0..q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();    
        let mut parts = input.split_whitespace();

        let l: u32 =   parts.next().unwrap().parse().unwrap(); 
        let r: u32 = parts.next().unwrap().parse().unwrap(); 

        let mut alternate_sum: i64 = 0; 

        for (idx, i) in (l..=r).enumerate() {
            if idx % 2 == 0 {
                alternate_sum += v[i as usize - 1] as i64; 
            }else {
                alternate_sum -= v[i as usize - 1] as i64;
            }
        }
            
        println!("{}", alternate_sum); 
    } 
}

fn have_exactly_k_factors(n:u32, k: u32) -> bool {
    let mut flag = false; 

    let mut i: u32 = 1;
    let mut count = 0; 

    while i.pow(2) <= n && count <= k{
        if i.pow(2) == n {
            count += 1; 
        }else if n % i == 0 {
            count += 2; 
        }

        i += 1; 
    } 
    
    if count == k {
        flag = true; 
    }
    
    return flag; 
}

fn count_numbers_with_exactly_k_factors(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  

    let n: u32 = parts.next().unwrap().parse().unwrap(); 

    let q: u32 = parts.next().unwrap().parse().unwrap();

    let k: u32 = parts.next().unwrap().parse().unwrap(); 
    
    let mut v: Vec<u32> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }
    
    let mut count = 0; 

    for i in 0..v.len(){
        let curr = v[i];
        if have_exactly_k_factors(curr, k){
            count += 1;  
        }
        v[i] = count; 
        
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

fn sum_of_digit(mut n: u32, k: u8) -> bool {
    let mut flag = false; 

    let mut sum: u8 = 0; 
    while n > 0 && sum <= k{
        let digit = n % 10; 
        sum += digit as u8; 
        n = n / 10; 
    }

    if sum == k {
        flag = true; 
    }

    return flag;
}

fn count_numbers_with_digit_sum_k(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();  

    let n: u32 = parts.next().unwrap().parse().unwrap(); 

    let q: u32 = parts.next().unwrap().parse().unwrap();

    let k: u8 = parts.next().unwrap().parse().unwrap(); 
    
    let mut v: Vec<u32> = Vec::new(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();    
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }
    
    let mut count = 0; 

    for i in 0..v.len(){
        let curr = v[i];
        if sum_of_digit(curr, k){
            count += 1;  
        }
        v[i] = count; 
        
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