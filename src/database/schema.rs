use std::collections::HashMap;

#[derive(Debug)]
enum DbAction {
    Create,
    Drop,
    None
}



#[derive(Debug)]
pub struct Database {
    name: String,
    connection: String,
    schema: HashMap<String, Schema>
}

#[derive(Debug)]
pub struct Schema {
    name: String,
    tables: HashMap<String, Table>,
    unrefed_tables: HashMap<String, Table>
}

#[derive(Debug)]
pub struct Table {
    name: String,
    pre_action: DbAction,
    post_action: DbAction,
    tables: HashMap<String, Column>
}

#[derive(Debug)]
pub struct Column {
    name: String,
    pre_action: DbAction,
    post_action: DbAction,
    data_type: String
}