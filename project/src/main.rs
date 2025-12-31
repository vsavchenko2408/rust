//use std::io;

fn main()
{
let mut arr = [5,4,3,2,1];
sort(&mut arr);
for i in arr
{
    println!("arr {}:  ", i);
}
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

fn sort(arr: &mut [i32;5])
{
    for x in arr
    {
        for y in arr
        {
            if x < y
            {
                swap(x, y);
            }
        }
    }
}