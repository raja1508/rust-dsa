use std::io;

fn main() {
    let mut input=   String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: u64 = input.trim().parse().unwrap();

    // let mut input=   String::new(); 
    // io::stdin().read_line(&mut input).unwrap();  
    // let mut parts = input.split_whitespace(); 

    // let mut v: Vec<i64> = Vec::new(); 
    // for _ in 0..n {
    //     let x = parts.next().unwrap().parse().unwrap();
    //     v.push(x);

    // }
    // // v.sort();

    // let mut input=   String::new(); 
    // io::stdin().read_line(&mut input).unwrap(); 
    // let target: i64 = input.trim().parse().unwrap();
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
    // let ans = nth_fibonacci_number(n);
    // println!("{}", ans);
    // print_array_in_reverse(&v, v.len());
    // print_array(&v, v.len());
    // let ans = minimum_element(&v, v.len()); 
    // println!("{}", ans); 
    // let ans = maximum_element(&v, v.len()); 
    // println!("{}", ans); 
    // let ans = sum_of_array(&v, v.len()); 
    // println!("{}", ans); 
    // let ans = is_sorted(&v, v.len()); 
    // if ans {
    //     println!("YES"); 
    // }else {
    //     println!("NO")
    // }
    // let ans = first_occurence_of_x(&v, target, v.len()); 
    // println!("{}", ans); 
    // let ans = last_occurence_of_x(&v, target, v.len()); 
    // println!("{}", ans); 
    // let ans = print_indices_of_x(&v, target, v.len());
    // if !ans {
    //     println!("-1"); 
    // }
    tower_of_hanoi(n);
}

fn tower_of_hanoi(n: u64) {
   if n == 1 {
      println!("Move A to C"); 
      return;
   }

   if n == 2{
      println!("Move A to B");
      // if c is empty 
      tower_of_hanoi(n - 1);
      // else make c empty
      println!("Move C to B"); 
      println!("Move B to C")
   }

   if n == 3 {
       println!("Move A to C"); 
       // if c is empty
       tower_of_hanoi(n - 1);
       // else make c empty
       println!("Move C to A")
   }
}

fn print_indices_of_x(v: &Vec<i64>, target: i64, n: usize) -> bool {
    if n == 0 {
       return false; 
    }

    let res = print_indices_of_x(v, target, n - 1); 
    if v[n - 1] == target{
        print!("{} ", n); 
    }

    return res || v[n-1] == target 

}

fn last_occurence_of_x(v: &Vec<i64>, target: i64, n: usize) -> i64 {
    if n == 0 {
       return -1; 
    }

    let res = last_occurence_of_x(v, target, n - 1); 
    
    let ans = if  v[n - 1] == target{
            n as i64 
    }else {
        -1 
    };

    if res == -1 {
       return ans; 
    }
    if res > ans {
        return res 
    }else{
        return ans;
    }
}

fn first_occurence_of_x(v: &Vec<i64>, target: i64, n: usize) -> i64 {
    if n == 0 {
       return -1; 
    }

    let res = first_occurence_of_x(v, target, n - 1); 
    if res == -1 {
        if v[n - 1] == target{
            return n as i64; 
        }
    }

    return  res;
}

fn binary_search_first_occurence_of_x(v: &Vec<i64>, target: i64, start: i64, end: i64) -> i64 {
    if start > end {
        return -1; 
    }
    
    let mid = start + (end - start)/2 ; 

    if v[mid as usize] == target {

        let res  = binary_search_first_occurence_of_x(v, target, start, mid - 1);   
        if res == -1 {
            return mid as i64 + 1;
        }
        return res; 
        
    }
    
    if v[mid as usize] > target{
        return binary_search_first_occurence_of_x(v, target, start, mid - 1)

    }else{
        return binary_search_first_occurence_of_x(v, target, mid + 1, end)
    }
}

fn is_sorted(v: &Vec<i64>, n: usize) -> bool{
    if n == 1{
        return true; 
    }

    return v[n - 1] >= v[n - 2] && is_sorted(v, n - 1); 
}

fn sum_of_array(v: &Vec<i64>, n: usize) -> i64{
    if n == 1{
        return v[0]; 
    }

    return v[n - 1] + sum_of_array(v, n - 1); 
}

fn maximum_element(v: &Vec<i64>, n: usize) -> i64{
    if n == 1{
        return v[0]; 
    }

    let res = maximum_element(v, n - 1); 
    if v[n - 1] < res {
        return res; 
    }else{
        return v[n - 1]; 
    }
}

fn minimum_element(v: &Vec<i64>, n: usize) -> i64{
    if n == 1{
        return v[0]; 
    }

    let res = minimum_element(v, n - 1); 
    if v[n - 1] < res {
        return v[n - 1]; 
    }else{
        return res
    }
}

fn print_array(v: &Vec<i64>, n: usize){
    if n == 0{
        return; 
    }

    print_array(v, n - 1);
    print!("{} ", v[n - 1]); 
}   

fn print_array_in_reverse(v: &Vec<i64>, n: usize){
    if n == 0{
        return; 
    }

    print!("{} ", v[n - 1]); 
    print_array_in_reverse(v, n - 1);
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