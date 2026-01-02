use std::io;

fn main(){

    let st1: Student = create_student();
    let st2: Student = create_student();
    let st3: Student = create_student();
    let mut students = Vec::<Student>::new();
    students.push(st1);
    students.push(st2);
    students.push(st3);
    for x in &students
    {
        show_students_info(x);
    }
    let mut count: usize = 0;
    let mut max_grade: f32 = 0.0;
    for(index, student) in students.iter().enumerate(){
        if student.grade > max_grade{
            max_grade = student.grade;
            count = index;
        }
    }
    println!("Best grade of: ");
    show_students_info(&students[count]);
}

fn readint() -> u8
{
    //println!("Enter age: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Wrong input!");
    let number: u8 = input
        .trim()
        .parse()
        .expect("Please repeat: ");
number
}
fn readstr() -> String
{
    //println!("Enter name: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Wrong input!");
    input.trim().to_string();
input
}
fn readfloat() -> f32
{
   // println!("Enter grade: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Wrong input!");
    let number: f32 = input
        .trim()
        .parse()
        .expect("Please repeat: ");
number
}

fn create_student() -> Student
{
    println!("Created student: ");
    println!("Enter name: ");
    let st_name = readstr();
    println!("Enter age: ");
    let st_age = readint();
    println!("Enter grade: ");
    let st_grade = readfloat();
    let st = Student{
    name : st_name, age : st_age, grade : st_grade
    };
   return st
}

fn show_students_info(obj :&Student)
{
    println!("----------------------------------------");
    println!("Name: {}", obj.name);
    println!("Age: {}", obj.age);
    println!("Grade: {}", obj.grade);
    println!("----------------------------------------");
    println!();
}
struct Student{
    pub name: String,
    pub age: u8,
    pub grade: f32
}

