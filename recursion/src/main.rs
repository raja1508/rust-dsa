use std::io;

fn main() {
    let mut input=   String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap(); 
    // let mut parts = input.split_whitespace(); 
    // let x = parts.next().unwrap().parse().unwrap(); 
    // let n = parts.next().unwrap().parse().unwrap(); 
    // print_decreasing(n);
    // print_increasing(n);
    // print_zig_zag(n);
    // print_number_in_reverse(n);
    // let ans = factorial(n);
    // println!("{}", ans);
    // let ans = sum_of_first_n_natural_numbers(n);
    // println!("{}", ans);
    // let ans = power(x, n);
    // println!("{}", ans);
    // let ans = sum_of_digits(n);
    // println!("{}", ans);
    let ans = nth_fibonacci_number(n);
    println!("{}", ans);
}

fn nth_fibonacci_number(n: u64) -> u64 {
    if n < 3 {
        return n - 1; 
    }

    let ans = nth_fibonacci_number(n - 1) + nth_fibonacci_number(n - 2); 
    return  ans;
}

fn sum_of_digits(n: u64) -> u64 {
    if n == 0 {
        return 0; 
    }

    let ans = n % 10 + sum_of_digits(n / 10);
    return  ans;
}

fn power(x: u64, n: u64) -> u64 {
    if n == 0 {
        return 1; 
    }

    let ans = x * power(x, n - 1);
    return  ans;
}

fn sum_of_first_n_natural_numbers(n: u64) -> u64 {
    if n == 1 {
        return 1; 
    }

    let ans = n +  sum_of_first_n_natural_numbers(n - 1);
    return  ans;
}

fn factorial(n: u64) -> u64 {
    if n < 2 {
        return 1; 
    }

    let ans = n * factorial(n - 1);
    return  ans;
}

fn print_number_in_reverse(n: u64){
    if n < 10 {
        print!("{}", n);
        return;
    }

    let rem = n % 10; 
    print!("{}", rem); 
    print_number_in_reverse(n / 10);

}

fn print_zig_zag(n: u64){
    if n == 1 {
        println!("{}", 1); 
        return; 
    }

    println!("{}", n); 
    print_zig_zag(n - 1);
    println!("{}", n); 
}

fn print_increasing(n: u64){
    if n == 1 {
        println!("{}", 1); 
        return; 
    }
    print_increasing(n - 1);
    println!("{}", n); 
}

fn print_decreasing(n: u64){
    if n == 1 {
        println!("{}", 1); 
        return; 
    }
    println!("{}", n); 
    print_decreasing(n - 1);
}