fn main()
{
 let mut a = 15;
 let b = 25;
 let c = add(a,b);
 a = 35;
 println!(" {}, {}",c, a);
 
}

fn add(x: i32 , y: i32)-> i32{
    x+y
}