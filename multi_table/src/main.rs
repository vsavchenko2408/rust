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
    let num :i32 = readline();
    println!("Таблиця помноження для числа {}", num);
    println!("-------------------------------------------");
    for x in 1 ..=10 {
        println!("{} * {} = {}", num, x, num * x);
    }

}