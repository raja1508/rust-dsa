use std::io::{self, Write};

fn main() {
    // range_sum_query();
    // even_sum_query();
    // count_vowels();
    // range_sum_of_squares();
    // if is_digit_sum_k(12345, 15){
    //     println!("YES")
    // }; 
    // count_numbers_with_digit_sum_k();
    // if is_factors_exactly_k(4, 3){
    //     println!("YES")
    // }; 
    // count_numbers_with_exactly_k_factors();
    // alternating_range_sum();
    // print_all_subarrays();
    sum_of_all_subarrays_separately();
    // sum_of_all_subarrays_easy();
    // sum_of_all_subarrays_hard();
    // maximum_subarray_sum_of_size_k();
}

fn maximum_subarray_sum_of_size_k(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts= input.split_whitespace(); 
    let n:u64 = parts.next().unwrap().parse().unwrap(); 
    let k:u64 = parts.next().unwrap().parse().unwrap();     
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new(); 
    for _ in 1..=n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        v.push(value);
    }

    let mut max_sum = 0;

    for i in 0..k as usize {
        max_sum += v[i]
    }; 

    let mut i = 0; 
    let mut j = k as usize; 
    
    let mut sum = max_sum; 
    while j < v.len() {
        sum += v[j];
        sum -= v[i]; 
        i += 1; 
        j += 1; 
        if sum > max_sum {
            max_sum = sum
        }

    }

    println!("{}", max_sum)
}

fn sum_of_all_subarrays_hard(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n:u64 = input.trim().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new(); 
    for _ in 1..=n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        v.push(value);
    }

    let mut sum = 0; 
    for i in 0..v.len(){
        let right_contribution = v.len() - i ; 
        let left_contribution = right_contribution * (v.len() - right_contribution); 
        let contribution = right_contribution + left_contribution; 
        sum += v[i] * contribution as i64; 
    }

    println!("{}", sum)
}

fn sum_of_all_subarrays_easy(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n:u64 = input.trim().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new(); 
    for _ in 1..=n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        v.push(value);
    }

    let mut sum = 0; 
    for i in 0..v.len(){
        let mut pre_sum = 0; 
        for j in i..v.len(){
            pre_sum += v[j];
            sum += pre_sum; 
        }
    }
    println!("{}", sum)
}

fn sum_of_all_subarrays_separately(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n:u64 = input.trim().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v: Vec<u64> = Vec::new(); 
    for _ in 1..=n {
        let value: u64 =  parts.next().unwrap().parse().unwrap(); 
        v.push(value);
    }


    for i in 0..v.len(){
        let mut pre_sum = 0; 
        for j in i..v.len(){
            pre_sum += v[j];
            println!("{}", pre_sum); 
            io::stdout().flush().unwrap(); // Forces the judge to read your output

        }
    }
}

fn print_all_subarrays(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n:u64 = input.trim().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v: Vec<u64> = Vec::new(); 
    for _ in 1..=n {
        let value: u64 =  parts.next().unwrap().parse().unwrap(); 
        v.push(value);
    }


    for i in 0..v.len(){
        let mut v2 = Vec::new(); 
        for j in i..v.len(){
            v2.push(v[j]);
            for k in 0..v2.len(){
                print!("{} ", v2[k])
            }
            println!()
        }
    }
}

fn alternating_range_sum(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();
    let n:u64 = parts.next().unwrap().parse().unwrap(); 
    let q:u64 = parts.next().unwrap().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut even_pre_sum: Vec<i64> = Vec::new(); 
    let mut odd_pre_sum: Vec<i64> = Vec::new(); 
    let mut even_sum: i64 = 0; 
    let mut odd_sum: i64 = 0; 
    for i in 1..=n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        if i % 2 == 0 {
            even_sum += value; 
        }else {
            odd_sum += value; 
        }
        even_pre_sum.push(even_sum);
        odd_pre_sum.push(odd_sum);
    }


    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let l: u64 = parts.next().unwrap().parse().unwrap(); 
        let r: u64 = parts.next().unwrap().parse().unwrap(); 

        let mut ans: i64 ; 
        if l == 1 {
            ans = odd_pre_sum[r as usize - 1] - even_pre_sum[r as usize - 1]
        }else{
            ans = (odd_pre_sum[r as usize - 1] - odd_pre_sum[l as usize - 2] )  
            - (even_pre_sum[r as usize - 1]  - even_pre_sum[l as usize - 2])
        }

        if l % 2 == 0 {
            ans = -ans ; 
        }

        println!("{}", ans)
    }

}

fn is_factors_exactly_k(num: u64, k: u8) -> bool {
    let mut count_factors = 0; 
    let mut i: u64  = 1; 
    while i.pow(2) <= num {
        if num % i == 0 {
            if i.pow(2) == num {
                count_factors += 1; 
            }else{
                count_factors += 2; 
            }
        }

        i += 1; 
    }

    return count_factors == k; 
}

fn count_numbers_with_exactly_k_factors(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();
    let n:u64 = parts.next().unwrap().parse().unwrap(); 
    let q:u64 = parts.next().unwrap().parse().unwrap(); 
    let k:u8 = parts.next().unwrap().parse().unwrap();     

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<u64> = Vec::new(); 

    let mut count = 0; 
    for _ in 0..n {
        let value: u64 =  parts.next().unwrap().parse().unwrap(); 
        if is_factors_exactly_k(value, k) {
            count += 1; 
        }
        v1.push(count);
    }


    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let l: u64 = parts.next().unwrap().parse().unwrap(); 
        let r: u64 = parts.next().unwrap().parse().unwrap(); 

        let ans; 
        if l == 1 {
            ans = v1[r as usize - 1]
        }else{
            ans = v1[r as usize - 1] - v1[l as usize - 2]
        }
        println!("{}", ans)
    }

}

fn is_digit_sum_k(mut num: u64, k: u8) -> bool {
    let mut digit_sum = 0; 
    while num > 0 {
        let value = num % 10;
        digit_sum += value as u8;  
        num  /= 10; 
    }

    return digit_sum == k; 
}

fn count_numbers_with_digit_sum_k(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();
    let n:u64 = parts.next().unwrap().parse().unwrap(); 
    let q:u64 = parts.next().unwrap().parse().unwrap(); 
    let k:u8 = parts.next().unwrap().parse().unwrap();     

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<u64> = Vec::new(); 

    let mut count = 0; 
    for _ in 0..n {
        let value: u64 =  parts.next().unwrap().parse().unwrap(); 
        if is_digit_sum_k(value, k) {
            count += 1; 
        }
        v1.push(count);
    }


    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let l: u64 = parts.next().unwrap().parse().unwrap(); 
        let r: u64 = parts.next().unwrap().parse().unwrap(); 

        let ans; 
        if l == 1 {
            ans = v1[r as usize - 1]
        }else{
            ans = v1[r as usize - 1] - v1[l as usize - 2]
        }
        println!("{}", ans)
    }

}

fn range_sum_of_squares(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();
    let n:u64 = parts.next().unwrap().parse().unwrap(); 
    let q:u64 = parts.next().unwrap().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<u64> = Vec::new(); 

    let mut sum = 0; 
    for _ in 0..n {
        let value: u64 =  parts.next().unwrap().parse().unwrap(); 
        sum += value.pow(2); 
        v1.push(sum);
    }


    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let l: u64 = parts.next().unwrap().parse().unwrap(); 
        let r: u64 = parts.next().unwrap().parse().unwrap(); 

        let ans; 
        if l == 1 {
            ans = v1[r as usize - 1]
        }else{
            ans = v1[r as usize - 1] - v1[l as usize - 2]
        }
        println!("{}", ans)
    }

}

fn count_vowels(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let word: String = input.trim().parse().unwrap();
    let char_array : Vec<char>= word.chars().collect();   

    let mut v1: Vec<i64> = Vec::new(); 

    let mut sum = 0; 
    for char in char_array {
        if char == 'a' || char == 'e' || char == 'i' || char == 'o'|| char == 'u' {
            sum += 1; 
        }

        v1.push(sum);
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let q:u64 = input.trim().parse().unwrap(); 

    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let l: u64 = parts.next().unwrap().parse().unwrap(); 
        let r: u64 = parts.next().unwrap().parse().unwrap(); 

        let ans; 
        if l == 1 {
            ans = v1[r as usize - 1]
        }else{
            ans = v1[r as usize - 1] - v1[l as usize - 2]
        }
        println!("{}", ans)
    }

}

fn even_sum_query(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<i64> = Vec::new(); 

    let mut sum = 0; 
    for idx in 1..=n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        if idx % 2 == 0 {
            sum += value; 
        }

        v1.push(sum);
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let q:u64 = input.trim().parse().unwrap(); 

    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let l: u64 = parts.next().unwrap().parse().unwrap(); 
        let r: u64 = parts.next().unwrap().parse().unwrap(); 

        let ans; 
        if l == 1 {
            ans = v1[r as usize - 1]
        }else{
            ans = v1[r as usize - 1] - v1[l as usize - 2]
        }
        println!("{}", ans)
    }

}

fn range_sum_query(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<i64> = Vec::new(); 

    let mut sum = 0; 
    for _ in 0..n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        sum += value; 
        v1.push(sum);
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let q:u64 = input.trim().parse().unwrap(); 

    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let l: u64 = parts.next().unwrap().parse().unwrap(); 
        let r: u64 = parts.next().unwrap().parse().unwrap(); 

        let ans; 
        if l == 1 {
            ans = v1[r as usize - 1]
        }else{
            ans = v1[r as usize - 1] - v1[l as usize - 2]
        }
        println!("{}", ans)
    }

}