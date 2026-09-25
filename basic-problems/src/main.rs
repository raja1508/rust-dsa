use std::{collections::{HashMap, HashSet}, io};

fn main(){
    // count_distinct();
    // search_queries();
    // count_distinct_queries();
    // count_distinct_queries_2();
    // union_of_two_arrays();
    // intersection_of_two_arrays();
    // intersection_of_two_arrays_2();
    // two_sum();
    // two_sum_2();
    two_sum_3();
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
        }

        let f: i64 = x - v ; 
        map.entry(f)
        .and_modify(|v| *v += 1)
        .or_insert(1);  
    }

    println!("{}", count); 
}

fn two_sum_2(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let x: i64 = parts.next().unwrap().parse().unwrap();  

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let mut map: HashMap<i64, u32> = HashMap::new(); 
    
    for i in 1..=n {
        let v: i64 = parts.next().unwrap().parse().unwrap();

        if let Some(value) = map.get(&v) {
            println!("{} {}", *value, i);  
            return; 
        }

        let f: i64 = x - v ; 
        map.insert(f, i); 
    }

    println!("{}", -1); 
}

fn two_sum(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: u32 = parts.next().unwrap().parse().unwrap();
    let x: i64 = parts.next().unwrap().parse().unwrap();  

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let mut set: HashSet<i64> = HashSet::new(); 

    let mut flag = false; 
    
    for _ in 0..n {
        let v: i64 = parts.next().unwrap().parse().unwrap();

        if let Some(_) = set.get(&v) {
            flag = true; 
            break; 
        }

        let f = x - v ; 
        set.insert(f); 
    }

    if flag {
        println!("TRUE"); 
    }else {
        println!("FALSE"); 
    }
}

fn intersection_of_two_arrays_2(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u32 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let mut map1: HashMap<i64, u32> = HashMap::new(); 
    
    for _ in 0..n {
        map1.entry(parts.next().unwrap().parse().unwrap())
        .and_modify(|v| *v += 1)
        .or_insert(1);
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n2: u32 = input.trim().parse().unwrap(); 

    let mut map2: HashMap<i64, u32> = HashMap::new(); 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n2 {
        map2.entry(parts.next().unwrap().parse().unwrap())
        .and_modify(|v| *v += 1)
        .or_insert(1);
    }
    let mut v = Vec::new(); 

    for (key, value) in map1 {
        if let Some(value2) = map2.get(&key) {
            let iteration = if value > *value2 {
                *value2
            }else {
                value
            };

            for _ in 0..iteration{
                v.push(key);
            }
        }
    }

    println!("{}", v.len()); 
    v.sort();
    for value in v{
        print!("{} ", value)
    }

    println!()
}

fn intersection_of_two_arrays(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u32 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let mut set1: HashSet<i64> = HashSet::new(); 
    
    for _ in 0..n {
        set1.insert(parts.next().unwrap().parse().unwrap());
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n2: u32 = input.trim().parse().unwrap(); 

    let mut set2: HashSet<i64> = HashSet::new(); 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n2 {
        set2.insert(parts.next().unwrap().parse().unwrap());
    }
    let mut v = Vec::new(); 

    for value in set1 {
        if let Some(_) = set2.get(&value) {
            v.push(value);
        }
    }

    println!("{}", v.len()); 
    v.sort();
    for value in v{
        print!("{} ", value)
    }

    println!()
}


fn union_of_two_arrays(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u32 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let mut set: HashSet<i64> = HashSet::new(); 
    
    for _ in 0..n {
        set.insert(parts.next().unwrap().parse().unwrap());
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n2: u32 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    
    for _ in 0..n2 {
        set.insert(parts.next().unwrap().parse().unwrap());
    }
    let mut v = Vec::new(); 

    for value in set {
        v.push(value); 
    }

    // for i in 0..v.len() {
    //     let mut min = i; 
    //     for j in i + 1..v.len() {
    //         if v[min] > v[j] {
    //             min = j
    //         }
    //     }

    //     let temp = v[i]; 
    //     v[i] = v[min]; 
    //     v[min] = temp; 
    // }

    println!("{}", v.len()); 
    v.sort();
    for value in v{
        print!("{} ", value)
    }

    println!()
}

fn count_distinct_queries_2() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let q: u32 = input.trim().parse().unwrap();

    let mut map: HashMap<i64, u32>  = HashMap::new();   

    for _ in 1..=q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let t: u8 = parts.next().unwrap().parse().unwrap();  
        
        match t {
            1 => {
                let x: i64 = parts.next().unwrap().parse().unwrap();
                if let Some((_, value)) = map.get_key_value(&x) {
                    map.insert(x, *value + 1 ); 
                }else {
                    map.insert(x, 1); 
                }
            },
            2 => {
                let x: i64 = parts.next().unwrap().parse().unwrap();
                if let Some((_, value)) = map.get_key_value(&x) {
                    let new_value = *value - 1; 
                    if new_value > 0 {
                        map.insert(x, *value - 1 ); 
                    }else {
                        map.remove(&x); 
                    }
                }
            },
            3 => {
                println!("{}", map.len())
            },
            4 => {
                let x: i64 = parts.next().unwrap().parse().unwrap();
                if let Some(_) = map.get(&x) {
                    println!("YES")
                }else {
                    println!("NO")
                }; 
            },
            _ => {
                
            }
        }

    }
}

fn count_distinct_queries() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let q: u32 = input.trim().parse().unwrap();

    let mut set: HashSet<i64>  = HashSet::new();   

    for _ in 1..=q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace(); 
        let t: u8 = parts.next().unwrap().parse().unwrap();  
        
        match t {
            1 => {
                let x: i64 = parts.next().unwrap().parse().unwrap();
                set.insert(x);   
            },
            2 => {
                let x: i64 = parts.next().unwrap().parse().unwrap();
                set.remove(&x); 
            },
            3 => {
                println!("{}", set.len())
            },
            4 => {
                let x: i64 = parts.next().unwrap().parse().unwrap();
                if let Some(_) = set.get(&x) {
                    println!("YES")
                }else {
                    println!("NO")
                }; 
            },
            _ => {
                
            }
        }

    }
}

fn search_queries() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u32 = parts.next().unwrap().parse().unwrap();
    let q: u32 = parts.next().unwrap().parse().unwrap();  

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let mut map: HashMap<i64, u32> = HashMap::new(); 
    
    for i in 1..=n {
        map.insert(parts.next().unwrap().parse().unwrap(), i);
    }

    for _ in 1..=q{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();  

        if let Some(value) = map.get(&x){
            println!("{}", *value); 
        }else {
            println!("{}", -1)
        }; 

    }
}

fn count_distinct() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n: u32 = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let mut set: HashSet<i64> = HashSet::new(); 
    
    for _ in 0..n {
        set.insert(parts.next().unwrap().parse().unwrap());
    }

    println!("{}", set.len())
}