use std::io;

fn main() {
    // linear_search();
    // binary_search();
    // binary_search_string();
    // selection_sort_trace();
    // bubble_sort_trace();
    merge_two_sorted_arrays();
}

fn merge_two_sorted_arrays() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n = parts.next().unwrap().parse().unwrap(); 
    let m = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v1: Vec<i64> = Vec::new();  
    for _ in 0..n {
        v1.push(parts.next().unwrap().parse().unwrap());
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v2: Vec<i64> = Vec::new();  
    for _ in 0..m {
        v2.push(parts.next().unwrap().parse().unwrap());
    }

    let mut i = 0; 
    let mut j = 0; 
    let mut v3: Vec<i64> = Vec::new(); 
    
    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            v3.push(v1[i]);
            i += 1; 
        }else {
            v3.push(v2[j]);
            j += 1; 
        }
    }

    while i < v1.len() {
        v3.push(v1[i]);
        i += 1; 
    }

    while j < v2.len() {
        v3.push(v2[j]);
        j += 1; 
    }

    for k in 0..v3.len() {
        print!("{} ", v3[k])
    }

    println!(); 

}

fn bubble_sort_trace() {
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

    for i in 0..n{
        let mut swap = 0; 
        for j in 0..n - i - 1 {
            if v[j] > v[j + 1] { 
                let temp = v[j]; 
                v[j] = v[j + 1]; 
                v[j + 1] = temp;
                swap += 1; 
            }
        }
        let mut v_string = String::new(); 

        for i in 0..n {
            v_string.push_str(&v[i].to_string());
            v_string.push_str(" ");
        }
        println!("Pass {}: {}, swaps = {}", i + 1, v_string , swap); 
        if swap != 0 {
            continue;
        }else {
            break; 
        }
    }
}

fn selection_sort_trace() {
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

    for i in 0..n - 1{
        let mut min = i; 
        for j in i + 1..n {
            if v[min] > v[j] {
                min = j; 
            }
        }
        let temp = v[min]; 
        v[min] = v[i]; 
        v[i] = temp; 
        let mut v_string = String::new(); 

        for i in 0..n {
            v_string.push_str(&v[i].to_string());
            v_string.push_str(" ");
        }

        println!("Pass {}: {}, min_selected = {}", i + 1, v_string , v[i] )
    }
}

fn binary_search_string() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n = input.trim().parse().unwrap(); 

    let mut v: Vec<String> = Vec::new();  
    for _ in 0..n {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().parse().unwrap();  
        v.push(name);
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let t: String = input.trim().parse().unwrap(); 
    let mut flag = false; 

    let mut start = 0; 
    let mut end: i64  = (v.len() - 1) as i64 ; 

    while start <= end{
        let mid = start + (end - start) / 2 ; 

        if v[mid as usize] == t {
            flag = true; 
            break; 
        }
        if v[mid as usize] > t{
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
    let mut end: i64  = (v.len() - 1) as i64 ; 

    while start <= end{
        let mid = start + (end - start) / 2 ; 

        if v[mid as usize] == t {
            flag = true; 
            break; 
        }
        if v[mid as usize] > t{
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