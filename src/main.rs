use mysql::*;
use mysql::prelude::*;

fn main() {
    let url = "mysql://user:password@localhost:3306";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    println!("Hello world! I am inside");
}
