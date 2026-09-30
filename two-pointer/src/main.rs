use std::{collections::{HashMap, HashSet}, io}; 
fn main() {
    // max_subarray_sum_of_size_k();
    // count_vowels();
    // count_number_of_distinct_integer();
    // count_good_numbers();
    // longest_subarray_with_sum_less_than_k();
    // longest_substring_without_repeating_characters();
    // longest_substring_with_atmost_k_distinct_characters();
    // count_subarrays_with_sum_less_than_k();
    smallest_subarray_with_sum_more_than_k();
}

fn smallest_subarray_with_sum_more_than_k(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u64 = parts.next().unwrap().parse().unwrap();      
    let k: u64 = parts.next().unwrap().parse().unwrap();
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
  
    let mut v = Vec::new(); 
    
    for _ in 0..n {
        let value: u64 = parts.next().unwrap().parse().unwrap(); 
        v.push(value);
 
    }

    let mut l: usize = 0; 
    let mut r = 0; 
    let mut ans = 0; 
    let mut count = 0; 
    let mut sum = 0; 

    // for i in 0..v.len(){
    //     sum += v[i]
    // }

    while r < v.len(){
        while sum <= k {
            sum += v[r]; 
            r += 1;
            count += 1;  
        }

        if count < ans {
            ans = count; 
        }
    }

    println!("{}", ans); 

}


fn count_subarrays_with_sum_less_than_k(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u64 = parts.next().unwrap().parse().unwrap();      
    let k: u64 = parts.next().unwrap().parse().unwrap();    
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
  
    let mut v = Vec::new(); 
    
    for _ in 0..n {
        let value: u64 = parts.next().unwrap().parse().unwrap(); 
        v.push(value);
 
    }


    let mut l: usize = 0; 
    let mut r = 0; 
    let mut ans = 0; 
    let mut count = 0;
    let mut sum = 0;

    for i in 0..v.len() {
        if v[i] < k {
            count += 1; 
        }
    }  

    sum += v[r]; 
    r += 1; 

    while r < v.len(){
        if sum + v[r] < k {
            sum += v[r]; 
            let gap = r - l  + 1; 
            if gap > 1 {
                count += gap - 1; 
            }
            r += 1;
        }else{
            sum -= v[l];     
            l += 1; 
            // let gap = r - l; 
            // if gap > 1 {
            //     count += gap - 1; 
            // }
        }

        if count > ans {
            ans = count ; 
        }
    }

    println!("{}", ans); 

}

fn longest_substring_with_atmost_k_distinct_characters(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u64 = parts.next().unwrap().parse().unwrap();      
    let k: usize = parts.next().unwrap().parse().unwrap();    
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let input_string: String = input.trim().parse().unwrap(); 
    let chars: Vec<char> = input_string.chars().collect(); 


    let mut l: usize = 0; 
    let mut r = 0; 
    let mut ans = 0; 
    let mut count = 0; 

    let mut map: HashMap<char, u64> = HashMap::new(); 
    while r < chars.len(){
        map.entry(chars[r]).and_modify(|v| *v += 1).or_insert(1); 
        // println!("{:?}", map); 
        if map.len() <= k {
            count += 1; 
            r += 1;
        }else{
            map.remove(&chars[r]); 
            if let Some(value) = map.get(&chars[l]){
                if *value == 1 {
                    map.remove(&chars[l]); 
                }else{
                    map.entry(chars[l]).and_modify(|v| *v -= 1);                     
                }
            } 
            count -= 1; 
            l += 1; 
        }

        if count > ans {
            ans = count ; 
        }
    }

    println!("{}", ans); 

}

fn longest_substring_without_repeating_characters(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap();      
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let input_string: String = input.trim().parse().unwrap(); 
    let chars: Vec<char> = input_string.chars().collect(); 


    let mut l: usize = 0; 
    let mut r = 0; 
    let mut ans = 0; 
    let mut count = 0; 

    let mut set: HashSet<char> = HashSet::new(); 
    while r < chars.len(){
        if let Some(_) = set.get(&chars[r]){
            set.remove(&chars[l]); 
            l += 1;
            count -= 1;  
        }else {
            count += 1; 
            set.insert(chars[r]);  
            r += 1;
        }

        if count > ans {
            ans = count ; 
        }
    }

    println!("{}", ans); 

}

fn longest_subarray_with_sum_less_than_k(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u64 = parts.next().unwrap().parse().unwrap();      
    let k: u64 = parts.next().unwrap().parse().unwrap();
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
  
    let mut v = Vec::new(); 
    
    for _ in 0..n {
        let value: u64 = parts.next().unwrap().parse().unwrap(); 
        v.push(value);
 
    }

    let mut l: usize = 0; 
    let mut r = 0; 
    let mut ans = 0; 
    let mut count = 0; 
    let mut sum = 0; 
    while r < v.len() {
        if sum + v[r] < k {
            sum += v[r]; 
            count += 1;
            r += 1;  
        }else{
            sum -= v[l];  
            count -= 1; 
            l += 1;
        }

        if count > ans {
            ans = count; 
        }
    }

    println!("{}", ans); 

}

fn count_good_numbers(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u64 = parts.next().unwrap().parse().unwrap();
    let m: u64 = parts.next().unwrap().parse().unwrap();       
    let k: u64 = parts.next().unwrap().parse().unwrap();
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
  
    let mut v = Vec::new(); 
    
    for _ in 0..n {
        let value: i64 = parts.next().unwrap().parse().unwrap(); 
        v.push(value);
 
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let mut set = HashSet::new(); 
    
    for _ in 0..m {
        let value: i64 = parts.next().unwrap().parse().unwrap(); 
        set.insert(value);
 
    }

    let mut good_count: u64 = 0;  
    for i in 0..k as usize{
        if let Some(_) = set.get(&v[i]) {
            good_count += 1; 
        }
    }
    print!("{} ", good_count);


    let mut i = 0; 
    for j in k as usize..v.len() {
        if let Some(value) = set.get(&v[j]){
            good_count += 1; 
        }

        if let Some(value) = set.get(&v[i]){
            good_count -= 1; 
        }
        
        i += 1; 
        print!("{} ", good_count); 

    }
}

fn count_number_of_distinct_integer(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u64 = parts.next().unwrap().parse().unwrap();
    let k: u64 = parts.next().unwrap().parse().unwrap();  
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
  
    let mut v = Vec::new(); 
    
    for _ in 0..n {
        let value: i64 = parts.next().unwrap().parse().unwrap(); 
        v.push(value);
 
    }

    let mut map = HashMap::new();  
    for i in 0..k as usize{
        map.entry(v[i]).and_modify(|v| *v += 1).or_insert(1); 
    }
    print!("{} ", map.len());

    let mut i = 0; 
    for j in k as usize..v.len() {
        if let Some(value) = map.get(&v[i]){
            if *value == 1 {
                map.remove(&v[i]); 
            }else {
                 map.entry(v[i]).and_modify(|v| *v -= 1).or_insert(1);                 
            }
        }
        map.entry(v[j]).and_modify(|v| *v += 1).or_insert(1); 
        i += 1; 
        print!("{} ", map.len()); 

    }
}

fn is_vowel(ch: char) -> bool{
    ch == 'a' || ch == 'e'|| ch == 'i'|| ch == 'o'|| ch == 'u'
}

fn count_vowels() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let k: i64 = parts.next().unwrap().parse().unwrap();  
     
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();  
    let input_string: String = input.trim().parse().unwrap();  
    let chars: Vec<char> = input_string.to_ascii_lowercase().chars().collect();

    let mut vowels_count = 0; 
    for i in 0..k as usize{
        if is_vowel(chars[i]){
            vowels_count += 1; 
        };
    }
    print!("{} ", vowels_count); 

    let mut i = 0; 
    for j in k as usize..chars.len(){
        if is_vowel(chars[j]){
            vowels_count += 1; 
        }

        if is_vowel(chars[i]){
            vowels_count -= 1; 
        }
        i += 1;
        print!("{} ", vowels_count); 
    }
}

fn max_subarray_sum_of_size_k(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let k: i64 = parts.next().unwrap().parse().unwrap();  
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
  
    let mut v = Vec::new(); 
    
    for _ in 0..n {
        let value: i64 = parts.next().unwrap().parse().unwrap(); 
        v.push(value);
 
    }

    let mut max_sum: i64 = 0; 
    for i in 0..k as usize{
        max_sum += v[i]
    }

    let mut sum = max_sum; 
    let mut i = 0; 
    for j in k as usize..v.len() {
        sum += v[j];
        sum -= v[i]; 
        i += 1; 

        if sum > max_sum {
            max_sum = sum
        }
    }

    println!("{}", max_sum); 

}