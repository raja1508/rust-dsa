use std::{collections::{HashMap, HashSet}, io}; 
fn main() {
    // max_subarray_sum_of_size_k();
    // count_vowels();
    // count_number_of_distinct_integer();
    count_good_numbers();
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