use std::{collections::HashMap, todo};

pub struct Table<R,C,V> {
    rows: HashMap<R, usize>,
    columns: HashMap<C, usize>,
    storage: Vec<Vec<Option<V>>> 
}

impl<R,C,V> Table<R,C,V> {
    pub fn new(rows: usize, cols: usize) -> Table<R,C,V> {
        todo!()
    }
    pub fn lookup(&self) -> Option<&V> {
        todo!()
    }
    pub fn insert(&mut self) {
        todo!()
    }
}

pub struct SimpleTable {
    storage: Vec<Vec<usize>>
}
