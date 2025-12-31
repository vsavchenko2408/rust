//use std::io;

fn main()
{
let mut a = 15;
let mut b = 25;
swap(&mut a,&mut b);
show(a,b);
}

fn foo(x :&mut i32)
{
    *x = *x * 10;
}

fn show(a: i32, b: i32)
{
    println!("A: {}, B: {}", a,b);
}

fn swap( a:&mut i32 , b:&mut i32 )
{
    let temp = *a;
    *a = *b;
    *b = temp;
}