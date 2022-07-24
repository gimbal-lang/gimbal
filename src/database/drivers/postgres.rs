
use crate::database::{DatabaseError, Table, Column, ColumnType};
use postgres::{Client, NoTls, error::DbError as PgDbError, Error as PgError};

#[derive(Debug)]
pub struct Postgres {
    host: String,
    user: String,
    dbname: String,
    password: String
}


impl From<PgError> for DatabaseError {
    fn from(source: PgError) -> Self {
        Self{source_message: format!("{}", source)}
    }
}

impl Postgres {
    pub fn new(host: &str, user: &str, dbname: &str, password: &str) -> Self {
        Postgres{
            host: host.to_string(),
            user: user.to_string(),
            dbname: dbname.to_string(),
            password: password.to_string(),
        }
    }
    pub fn conn_string(&self) -> String {
        format!("host={} user={} dbname={} user={} password={}",
        self.host, self.user, self.dbname, self.user, self.password)
    }
}

pub fn get_table(db: Postgres, table: &str) -> Result<Option<Table>, DatabaseError> {
    let mut client = Client::connect(&db.conn_string(), NoTls)?;
    let t = table.to_lowercase();
    let qry = "SELECT table_name FROM information_schema.tables WHERE table_name = $1";
    let res = client.query(qry, &[&t])?;
    match res.len() {
        0 => Ok(None),
        1 => Ok(Some(Table{name: res[0].get(0), columns: get_columns(db, &t)?})),
        _ => unreachable!()
    }
}

fn get_columns(db: Postgres, table: &str) -> Result<Vec<Column>, DatabaseError> {
    let mut client = Client::connect(&db.conn_string(), NoTls)?;
    let t = table.to_lowercase();
    let qry = "SELECT column_name, udt_name FROM information_schema.columns WHERE table_name = $1";
    let res = client.query(qry, &[&t])?;
    Ok(res.into_iter().map(|c| Column{name: c.get(0), column_type: get_column_type(c.get(1))}).collect::<Vec<Column>>())
}

fn get_column_type(udt_type: &str) -> ColumnType {
    match udt_type {
        "varchar" => ColumnType::String,
        _ => unreachable!()
    }
}
/*
pub fn find_table(db: Postgres, table: &str) -> Result<bool, Error> {
    let mut client = Client::connect(db.conn_string(), NoTls).unwrap();
    let t = table.to_lowercase().unwrap();
    let qry = format!("SELECT table_name FROM information_schema.tables WHERE table_name = {}", t);
    let res = client.query(qry);
    match res.len() {
        0 => Ok(false),
        1 => Ok(true),
        _ => unreachable!
    }
}
*/