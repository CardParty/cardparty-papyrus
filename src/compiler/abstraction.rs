use std::collections::HashMap;
use std::hash::Hash;

use super::deserialization::Tables;
use super::error::RegisterError;
use super::refrences::{StateIdent, TableRefrence};

#[derive(Clone)]
pub struct TableValue {
    weight: i32,
    value: String,
}
#[derive(Clone)]
pub struct TableCache {
    cache: HashMap<String, Vec<TableValue>>,
}

#[derive(Clone)]
pub struct TableQuery {
    data: TableCache,
}

pub enum Query {
    Filter(String),
}

impl TableQuery {
    pub fn execute(mut self, query: Query) -> Vec<TableValue> {
        match query {
            Query::Filter(tag) => {
                if let Some(values) = self.data.cache.remove(&tag) {
                    return values;
                } else {
                    Vec::new()
                }
            }
        }
    }
}

pub struct TableRegister {
    table_lookup: HashMap<String, TableRefrence>,
    tables: Vec<TableCache>,
}

impl TableRegister {
    pub fn generate_register(raw_tables: Tables) -> Self {
        let mut table_lookup = HashMap::new();
        let mut tables = Vec::with_capacity(raw_tables.len());

        for (index, table) in raw_tables.into_iter().enumerate() {
            let mut cache = HashMap::new();

            for value in table.values {
                let table_value = TableValue {
                    weight: value.weight as i32,
                    value: value.value,
                };

                for tag in value.tags {
                    cache
                        .entry(tag)
                        .or_insert_with(Vec::new)
                        .push(table_value.clone());
                }
            }

            table_lookup.insert(table.name, TableRefrence::new(index));
            tables.push(TableCache { cache });
        }

        TableRegister {
            table_lookup,
            tables,
        }
    }

    /// More Expensive due to use of a copy.
    pub fn query_table(&self, refrence: TableRefrence) -> TableQuery {
        let table = self.tables[refrence.unpack()].clone();
        TableQuery { data: table }
    }
    /// Important for when you just need to see data from a table.
    pub fn peek_table(&self, refrence: TableRefrence, tag: String) -> Option<&Vec<TableValue>> {
        let ref table = self.tables[refrence.unpack()];
        table.cache.get(&tag)
    }

    pub fn get_table_refrence(&self, name: String) -> Result<TableRefrence, RegisterError> {
        if let Some(refrence) = self.table_lookup.clone().remove(&name) {
            return Ok(refrence);
        } else {
            return Err(RegisterError::TableLookupError);
        }
    }
}

pub struct State {
    value: i64,
    min: i64,
    max: i64,
}

pub struct HookedState {
    value: i64,
    min: i64,
    max: i64,
}

pub struct StateRegister {
    cache: HashMap<String, State>,
}
