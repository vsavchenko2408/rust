use std::io;

fn main()
{
    let num = readline();
    print!("Прості числа до {}: ", num);
        for i in 2..=num {
        if is_prime(i) {
            print!("{}, ", i);
        }
    }
    
    println!();
}

fn is_prime(num: i32) -> bool
{
    if num < 2
    {
        return false;
    }
    for i in 2 ..num
    {
        if num % i == 0
        {
            return false;
        }
    }

    true
}

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