use std::io;

fn main(){
    // row_order();
    // column_order();
    // find_maximum();
    // search_2d();
    // row_minimum();
    // column_sum();
    // row_with_max_1s();
    // wave_1();
    // wave_2();
    boundary_traversal();
}

fn boundary_traversal(){
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

    for k in 0..m {
        let row  = 0; 
        let col = k; 
        print!("{} ", matrix[row][col])
    }
    
    for k in 1..n {
        let row  = k; 
        let col = m - 1; 
        print!("{} ", matrix[row][col])
    }
    
    if n > 1 && m > 1 {  
        for k in  (0..=m-2).rev(){
            let row  = n - 1; 
            let col = k; 
            print!("{} ", matrix[row][col])
        }
        
    
        for k in  (1..=n-2).rev(){
            let row  = k; 
            let col = 0; 
            print!("{} ", matrix[row][col])
        }
    }
}

fn wave_2(){
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
            let row = if i % 2 == 0 {
                j
            }else {
                n - 1 - j 
            };
            
            print!("{} ", matrix[row][i])
        }
    }
}

fn wave_1(){
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

    for i in 0..n{
        for j in 0..m {

            let col = if i % 2 == 0 {
                j
            }else {
                m - 1 - j 
            };

            print!("{} ", matrix[i][col])
        }
    }
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