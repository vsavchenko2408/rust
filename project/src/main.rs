use std::io;

fn main()
{
    let mut temp_a= String::new();
    println!("Input a:");
    io::stdin().read_line(&mut temp_a).expect("Error!");
 
    let mut temp_b= String::new();
    println!("Input b:");
    io::stdin().read_line(&mut temp_b).expect("Error!");
    
    let a:i32 = temp_a.trim().parse().expect("Input a:");
    let b:i32 = temp_b.trim().parse().expect("Input b:");
    let result = add(a,b);

 println!("Result: {}",result);
 
}

fn add(x: i32 , y: i32)-> i32{
    x+y
}