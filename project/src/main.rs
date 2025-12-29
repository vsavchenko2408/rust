//use std::io;

fn main()
{
    let mut x = 15;

    foo(&mut x);
    show(x);

}

fn foo(x :&mut i32)
{
    *x = *x * 10;
}

fn show(x: i32)
{
    println!("Result: {}", x);
}