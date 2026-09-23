use std::io;

fn main(){
    // row_order();
    // column_order();
    // find_maximum();
    // search_2d();
    // row_minimum();
    // column_sum();
    // row_with_max_1s();
}

fn row_with_max_1s(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let mut parts = input.split_whitespace(); 

    let n= parts.next().unwrap().parse().unwrap(); 
    let m = parts.next().unwrap().parse().unwrap();

    let mut count = vec![0; n]; 

    for i in 0..n{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        for _ in 0..m {
            let current_element: u32  = parts.next().unwrap().parse().unwrap();
            if current_element == 1{
                count[i] += 1; 
            }
        }
    }
    
    let mut row_max = 0; 
    for i in 1..n{
        if count[i] > count[row_max]{
            row_max = i; 
        }
    }

    if count[row_max] == 0 {
        println!("{}", -1)
    }else {
        println!("{}", row_max)
    }
}

fn column_sum(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let mut parts = input.split_whitespace(); 

    let n= parts.next().unwrap().parse().unwrap(); 
    let m = parts.next().unwrap().parse().unwrap();

     let mut matrix: Vec<Vec<i32>> = vec![vec![0; m]; n]; 

    for i in 0..n{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();  

        for j in 0..m {
            matrix[i][j] = parts.next().unwrap().parse().unwrap();
        }
    }

    for i in 0..m{
        let mut sum = 0; 
        for j in 0..n {
            sum += matrix[j][i]; 
        }

        print!("{} ", sum)
    }
}

fn row_minimum(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let mut parts = input.split_whitespace(); 

    let n: i32 = parts.next().unwrap().parse().unwrap(); 

    let m: i32 = parts.next().unwrap().parse().unwrap(); 

    for _ in 0..n{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        let mut min: u32 = parts.next().unwrap().parse().unwrap(); 

        for _ in 0..m - 1{
           let num: u32 = parts.next().unwrap().parse().unwrap();

           if min > num {
                min = num; 
           }

        }

        print!("{} ", min)
    }

}

fn search_2d(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let mut parts = input.split_whitespace(); 

    let n= parts.next().unwrap().parse().unwrap(); 
    let m = parts.next().unwrap().parse().unwrap();
    let target: u32 = parts.next().unwrap().parse().unwrap();

    let mut found = false; 

    for _ in 0..n{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        for _ in 0..m {
           let num = parts.next().unwrap().parse().unwrap();

           if target == num {
                found = true
           }    
        }
    }

    if found {
        println!("true")
    }else {
        println!("false")
    }

}

fn find_maximum(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let mut parts = input.split_whitespace(); 

    let n= parts.next().unwrap().parse().unwrap(); 
    let m = parts.next().unwrap().parse().unwrap();

    let mut max = 0u32; 

    for _ in 0..n{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();

        for _ in 0..m {
           let num = parts.next().unwrap().parse().unwrap();

           if max < num {
                max = num; 
           }    
        }
    }

    println!("{}", max)

}

fn column_order(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let mut parts = input.split_whitespace(); 

    let n= parts.next().unwrap().parse().unwrap(); 
    let m = parts.next().unwrap().parse().unwrap();

     let mut matrix: Vec<Vec<i32>> = vec![vec![0; m]; n]; 

    for i in 0..n{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();  

        for j in 0..m {
            matrix[i][j] = parts.next().unwrap().parse().unwrap();
        }
    }

    for i in 0..m{
        for j in 0..n {
            print!("{} ", matrix[j][i])
        }
    }
}

fn row_order(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let mut parts = input.split_whitespace(); 

    let n = parts.next().unwrap().parse().unwrap(); 
    let m = parts.next().unwrap().parse().unwrap();

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; m]; n]; 

    for i in 0..n{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();  

        for j in 0..m {
            matrix[i][j] = parts.next().unwrap().parse().unwrap();
        }
    }

    for i in 0..matrix.len(){
        for j in 0..matrix[i].len() {
            print!("{} ", matrix[i][j])
        }
    }
}