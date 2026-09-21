use std::io;

fn main() {
    hollow_vertical_pyramid();
}

fn hollow_vertical_pyramid(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut m: usize = 1; 
    let mut space: usize = 0; 

    let mut row_stars = String::default(); 

    for  i in 1..=(2*n - 1)  {

        if m > 1 {
            row_stars = "* *".to_string(); 
        }else {
            row_stars = "*".to_string(); 
        }

        if i < n {
            row_stars.pop(); 
            row_stars.push_str(&" ".repeat(space));
            row_stars.push('*'); 
            if m >= 2 {
                space += 2; 
            }
            m += 1;
        }else if i == n {
            row_stars.pop(); 
            row_stars.push_str(&" ".repeat(space));
            row_stars.push('*');
            m -= 1;
            space -= 2; 

        }else {
            if m != 1 {
                row_stars.pop(); 
                row_stars.push_str(&" ".repeat(space));
                row_stars.push('*');
                m -= 1;
                if m >=2 {
                    space -= 2; 
                } 
            }
        }
        println!("{}", row_stars); 
    }
}
