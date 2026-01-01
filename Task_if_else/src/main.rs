use std::io;

fn readline() -> i32
{
        println!("Введіть число: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Невірний ввід!");
    let number: i32 = input
    .trim()
    .parse()
    .expect("Будь ласка, введить правильне число: ");
number
}


fn main()
{

    let num = readline();

    println!("Number is {} ", num);

    if num%2 == 0
    {
        println!("Це парне число!");
    } 
    if num% 2 != 0
    {
        println!("Це непарне число!");
    }
    if num % 3 == 0
    {
        println!("Це число ділиться на 3!");
    }
    if num > 0
    {
        println!("Це додатне число!");       
    }
    if num < 0
    {
        println!("Це від'ємне число!");
    }
    if num == 0
    {
        println!("Це нуль!");
    }
    if num < 100
    {
        println!("Це число меньше 100!");
    }
    if num > 100
    {
        println!("Це число більше 100!");  
    }


}