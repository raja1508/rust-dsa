use std::io;

fn main() {
    // linear_search();
    binary_search();
}

fn binary_search() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n = input.trim().parse().unwrap(); 

    if n == 0 {
        println!("NO");
        return;
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new();  
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let t: i64 = input.trim().parse().unwrap(); 
    let mut flag = false; 

    let mut start = 0; 
    let mut end = v.len(); 

    while start <= end{
        let mid = start + (end - start) / 2 ; 

        if v[mid] == t {
            flag = true; 
            break; 
        }
        if v[mid] > t{
            if mid == 0 {
                break; 
            }
            end = mid - 1; 
        }else {
            start = mid + 1; 
        }
    }

    if flag {
        println!("YES")
    }else{
        println!("NO")
    }
}

fn linear_search() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n = input.trim().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new();  
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let t: i64 = input.trim().parse().unwrap(); 
    let mut flag = false; 

    for value in v {
        if value == t {
            flag = true; 
            break; 
        }
    }

    if flag {
        println!("YES")
    }else{
        println!("NO")
    }
}