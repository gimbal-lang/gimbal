mod util;
mod lang;
mod database;

use database::drivers::postgres::Postgres;
use database::drivers::postgres::get_table;

fn main() {
    println!("Hello, world!");
    let db_conn = Postgres::new("localhost", "postgres", "david", "postgres");
    let table = get_table(db_conn, "test__person");
    match table {
        Ok(t) => println!("{}", t.unwrap()),
        Err(e) => println!("{}", e)

    }

}
