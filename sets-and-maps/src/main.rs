use std::io; 
use std::collections::{HashMap, HashSet};  
fn main() {
    // count_distinct();
    // search_queries();
    // count_distinct_queries();
    // count_distinct_queries_2();
    // union_of_two_arrays();
    // intersection_of_two_arrays();
    // intersection_of_two_arrays_2();
    // two_sum();
    // two_sum_2();
    // two_sum_3();
    // subarray_sum_equals_x();
    find_subarray_with_sum_x();
}

fn find_subarray_with_sum_x(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let x: i64 = parts.next().unwrap().parse().unwrap();  
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
  
    let mut v = Vec::new(); 
    let mut sum = 0; 
    
    for _ in 0..n {
        let value: i64 = parts.next().unwrap().parse().unwrap();
        sum += value; 
        v.push(sum);
 
    }
    
    for i in 0..v.len(){
        if v[i] == x {
            println!("{} {}", 1, i + 1); 
            return; 
        }
    }

    let mut map = HashMap::new(); 
    for j in 0..v.len(){
        if let Some(value) = map.get(&v[j]){
            println!("{} {}", value , j + 1); 
            return;
        }
        let target = x - v[j];
        map.insert(target, j + 1);  
    }
    

    println!("{}", -1); 
    

}

fn subarray_sum_equals_x(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let x: i64 = parts.next().unwrap().parse().unwrap();  
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
  
    let mut v = Vec::new(); 
    let mut sum = 0; 
    
    for _ in 0..n {
        let value: i64 = parts.next().unwrap().parse().unwrap();
        sum += value; 
        v.push(sum);
 
    }

    let mut flag = false; 
    for i in 0..v.len(){
        if v[i] == x {
            flag = true; 
            break; 
        }
    }

    let mut set = HashSet::new(); 
    for j in 0..v.len(){
        if let Some(v) = set.get(&v[j]){
            flag = true; 
            break;
        }
        let target = x - v[j];
        set.insert(target);  
    }
    
    if flag {
        println!("YES")
    }else{
        println!("NO")
    }
}

fn two_sum_3(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let x: i64 = parts.next().unwrap().parse().unwrap();  
 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
 
    let mut map: HashMap<i64, u64> = HashMap::new(); 
 
    let mut count = 0; 
    
    for _ in 0..n {
        let v: i64 = parts.next().unwrap().parse().unwrap();
 
        if let Some(value) = map.get(&v) {
            count += value;     
            // map.remove(&v);        
        }
 
        let f: i64 = x - v ; 
        map.entry(f)
        .and_modify(|v| *v += 1)
        .or_insert(1);  
    }
 
    println!("{}", count); 
}

fn two_sum_2() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let n:u64 = parts.next().unwrap().parse().unwrap(); 
    let x:i64 = parts.next().unwrap().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v = Vec::new(); 
    
    for i in 1..=n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        v.push(value);
    }
    
    let mut map = HashMap::new(); 
    for i in 0..v.len() {
        let target = x - v[i]; 
        if let Some(v) = map.get(&v[i]){
            println!("{} {}", v, i + 1);
            return; 
        }else{
            map.insert(target, i + 1); 
        }
    }

    println!("{}", -1); 

}

fn two_sum() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let n:u64 = parts.next().unwrap().parse().unwrap(); 
    let x:i64 = parts.next().unwrap().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v = Vec::new(); 
    
    for i in 1..=n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        v.push(value);
    }
    
    let mut set = HashSet::new(); 
    for i in 0..v.len() {
        let target = x - v[i]; 
        if let Some(_) = set.get(&v[i]){
            println!("TRUE");
            return; 
        }else{
            set.insert(target); 
        }
    }

    println!("FALSE"); 

}

fn intersection_of_two_arrays_2(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<i64> = Vec::new();
    let mut map = HashMap::new();  

    for _ in 0..n {
        let key: i64 =  parts.next().unwrap().parse().unwrap(); 
        map.entry(key).and_modify(|v| *v += 1).or_insert(1); 
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    for _ in 0..n {
        let key: i64 =  parts.next().unwrap().parse().unwrap(); 
        if let Some(value) = map.get(&key){
            if *value == 1 {
                map.remove(&key); 
            }else{
                map.entry(key).and_modify(|v| *v -= 1); 
            }
            v1.push(key);
        }
    }

    v1.sort();
    println!("{}", v1.len()); 
    for i in 0..v1.len(){
        print!("{} ", v1[i]); 
    }
}

fn intersection_of_two_arrays(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<i64> = Vec::new();
    let mut set = HashSet::new();  

    for _ in 0..n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        set.insert(value); 
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    for _ in 0..n {
        let key: i64 =  parts.next().unwrap().parse().unwrap(); 
        if let Some(value) = set.get(&key){
            v1.push(*value);
        }

        set.remove(&key); 
    }

    v1.sort();
    println!("{}", v1.len()); 
    for i in 0..v1.len(){
        print!("{} ", v1[i]); 
    }
}

fn union_of_two_arrays(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<i64> = Vec::new();
    let mut set = HashSet::new();  

    for _ in 0..n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        if let None = set.get(&value){
            v1.push(value);
            set.insert(value); 
        }; 
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    for _ in 0..n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        if let None = set.get(&value){
            v1.push(value);
            set.insert(value); 
        }; 
    }

    v1.sort();

    for i in 0..v1.len(){
        print!("{} ", v1[i]); 
    }
}

fn count_distinct_queries_2() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let q:u64 = input.trim().parse().unwrap(); 

    
    // let mut v1: Vec<i64> = Vec::new();
    let mut map = HashMap::new();  
    
    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let t: u8 =  parts.next().unwrap().parse().unwrap(); 
        
        match t {
            1 => {
                let value: i64 =  parts.next().unwrap().parse().unwrap(); 
                map.entry(value).and_modify(|v| *v += 1).or_insert(1); 
            },
            2 => {
                let value: i64 =  parts.next().unwrap().parse().unwrap();  
                if let Some(v) = map.get(&value){
                    if *v == 1 {
                        map.remove(&value); 
                    }else{
                         map.entry(value).and_modify(|v| *v -= 1);
                    }
                };               
            },
            3 => {
                println!("{}", map.len()); 
            },
            4 => {
                let value: i64 =  parts.next().unwrap().parse().unwrap();  
                if let Some(_) = map.get(&value){
                    println!("YES")
                }else {
                    println!("NO")
                }
            },
            _ => {}
        }
    }
}

fn count_distinct_queries() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let q:u64 = input.trim().parse().unwrap(); 

    
    // let mut v1: Vec<i64> = Vec::new();
    let mut set = HashSet::new();  
    
    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let t: u8 =  parts.next().unwrap().parse().unwrap(); 
        
        match t {
            1 => {
                let value: i64 =  parts.next().unwrap().parse().unwrap(); 
                set.insert(value); 
            },
            2 => {
                let value: i64 =  parts.next().unwrap().parse().unwrap();                  
                set.remove(&value); 
            },
            3 => {
                println!("{}", set.len()); 
            },
            4 => {
                let value: i64 =  parts.next().unwrap().parse().unwrap();  
                if let Some(_) = set.get(&value){
                    println!("YES")
                }else {
                    println!("NO")
                }
            },
            _ => {}
        }
    }
}

fn search_queries() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let n:u64 = parts.next().unwrap().parse().unwrap(); 
    let q:u64 = parts.next().unwrap().parse().unwrap(); 
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    let mut map = HashMap::new(); 

    for i in 1..=n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
            map.insert(value, i); 
    }

    for _ in 0..q {
           let mut input = String::new(); 
           io::stdin().read_line(&mut input).unwrap();
           let num: i64 = input.trim().parse().unwrap();  
           if let Some(v) = map.get(&num) {
                println!("{}", v); 
            }else {
                println!("{}", -1)
            }
    }

}

fn count_distinct() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n:u64 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 

    // let mut v1: Vec<i64> = Vec::new();
    let mut set = HashSet::new();  

    for _ in 0..n {
        let value: i64 =  parts.next().unwrap().parse().unwrap(); 
        if let None = set.get(&value){
            // v1.push(value);
            set.insert(value); 
        }; 
    }

    println!("{}", set.len())

}