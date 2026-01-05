//use std::io;

fn main()
{
    let mut dt = Data {
        data: 15,
    };
    dt.set_data(25);
    println!("Data: {}", dt.get_data());
}

struct Data{
    data: i32
}

impl Data {
    fn set_data(&mut self, data:i32)
    {
        self.data = data;
    }
    fn get_data(&self) -> i32
    {
        self.data
    }
}