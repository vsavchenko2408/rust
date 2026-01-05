use std::io;

fn readint() -> u16
{
    //println!("Enter age: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Wrong input!");
    let number: u16 = input
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

fn main()
{
    let mut bk = Book::new();
    /* 
    bk.create_book(); 
    bk.show_info();
    bk.order_book();
    bk.show_info();
    bk.back_book();
    bk.show_info();
    */
    let mut books: Vec<Book> = Vec::new();
    books.push(bk);
    
}


struct Book
{
    name: String,
    author: String,
    year: u16,
    is_free: bool
}

impl Book
{
    fn new() -> Book
    {
        Book { 
        name: String::new(),
        author: String::new(),
        year: 0,
        is_free: true
        }
    }
    fn show_info(&self)
    {
        println!("Name: {}", self.name);
        println!("Author: {}", self.author);   
        println!("Year: {}", self.year);
        println!("Free: {}", self.is_free);   
    }
    fn create_book(&mut self) -> &Book
    {
        println!("              Created book:           ");
        println!();

        println!("Enter name of book: ");
        self.name = readstr();
        println!("Enter name author: ");
        self.author = readstr();
        println!("Enter year: ");
        self.year = readint();
        self.is_free = true;
        self
    }
    fn order_book(&mut self) -> &Book
    {
        self.is_free = false;
        self
    }
    fn back_book(&mut self) -> &Book
    {
        self.is_free = true;
        self
    }
}