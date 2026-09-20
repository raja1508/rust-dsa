use std::io;

fn main() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n = input.trim().parse().unwrap(); 
    print_evens(n);

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let l = parts.next().unwrap().parse().unwrap(); 
    let r = parts.next().unwrap().parse().unwrap(); 
    print_evens(n);
    print_left_to_right(l, r);
    print_letters();
    count_numbers();
    factorial();
    exponential();
    reverse_number();
    sum_of_digits();
    palindrome();
}

fn palindrome() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let input = input.trim();
    let mut i = 0;
    let mut j = input.len() - 1;
    let mut flag = true; 
    while i <= j {
        let l = input.chars().nth(i); 
        let r = input.chars().nth(j); 

        if l == r {
            i += 1; 
            j -= 1; 
            continue
        }else {
            flag = false; 
            break
        }
    }

    if flag {
        println!("Yes")
    }else {
        println!("No")
    }
}

fn sum_of_digits() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut n: u64 = input.trim().parse().unwrap(); 

    let mut sum  = 0u64;  

    while n > 0 {
        let r = n % 10; 
        sum += r; 
        n = n / 10
    }

    println!("{}", sum); 
}

fn reverse_number() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut n: u64 = input.trim().parse().unwrap(); 

    let mut ans = String::new(); 

    if n == 0 {
        ans.push_str(&"0".to_string());
        println!("{}", ans); 
        return 
    }
    while n > 0 {
        let r = n % 10 ;
        if r != 0 {
            ans.push_str(&r.to_string());
        } 
        n = n / 10
    }

    println!("{}", ans); 
}


fn exponential() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let l: i64 = parts.next().unwrap().parse().unwrap(); 
    let r = parts.next().unwrap().parse().unwrap();  
    let mut expo = 1; 
    for _ in  1..=r {
        if r != 0 {
            expo *= l ; 
        }
    }
    println!("{}", expo); 
}

fn factorial(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap(); 
    let mut fact: u64 = 1; 
    for i in 1..=n {
        fact *= i; 
    } 

    println!("{}", fact); 
}

fn count_numbers() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u32 = input.trim().parse().unwrap();

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace();

    let mut positive_count = 0u32; 
    let mut negative_count = 0u32; 
    let mut even_count = 0u32; 
    let mut odd_count = 0u32; 
    for _ in 0..n {
        let num: i64 = parts.next().unwrap().parse().unwrap(); 
        if num % 2 == 0 {
            even_count += 1
        }else {
            odd_count += 1 
        }

        if num == 0 {
            continue;
        }

        if num > 0 {
            positive_count += 1; 
        }else {
            negative_count += 1; 
        }
    }

        println!("{}", positive_count); 
        println!("{}", negative_count);
        println!("{}", even_count);
        println!("{}", odd_count);
}

fn print_letters() {
    for letter in 'A'..='Z' {
        print!("{} ", letter)
    }
}

fn print_left_to_right(l:i32, r: i32){
    if l > r {
        return 
    }
    for i in l..=r {
        print!("{} ",i); 
    }
}

fn print_evens(n:u32){
    for i in 1..=n {
        if i % 2 == 0 {
            print!("{} ",i); 
        }
    }
}

