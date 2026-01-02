use std::io;

fn main(){
    
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

fn create_student() -> Student
{
    let mut st: Student;
    println!("");

   return st
}
struct Student{
    pub name: String,
    pub age: u8,
    pub grade: f32
}