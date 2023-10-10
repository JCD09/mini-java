use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub(super) struct Table<R, C, V> {
    table: HashMap<R, HashMap<C, V>>,
}

impl<R, C, V> Table<R, C, V>
where R: Copy + Eq + Hash,
      C: Copy + Eq + Hash,
      V: Copy
{
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            table: HashMap::with_capacity(capacity)
        }
    }

    pub(super) fn insert(&mut self, row: R, column: C, value: V) {
        let row = self.table.entry(row).or_insert_with(HashMap::new);
        row.insert(column, value);
    }

    pub(super) fn lookup(&self, row: R, column: C) -> Option<V> {
        self.table.get(&row).and_then(|row| row.get(&column).copied())
    }

    // pub(super) fn iter(&self) -> impl Iterator<Item = (&R, &HashMap<C, V>)> {
    //     self.table.iter()
    // }

    // pub(super) fn iter_mut(&mut self) -> impl Iterator<Item = (&R, &mut HashMap<C, V>)> {
    //     self.table.iter_mut()
    // }

    // pub(super) fn iter_over_rows(&self) -> impl Iterator<Item = &R> {
    //     self.table.keys()
    // }

    // pub(super) fn iter_over_columns(&self) -> impl Iterator<Item = &C> {
    //     self.table.values().flat_map(|row| row.keys())
    // }

    // pub(super) fn iter_over_values(&self) -> impl Iterator<Item = &V> {
    //     self.table.values().flat_map(|row| row.values())
    // }

    // pub(super) fn iter_over_rows_and_columns(&self) -> impl Iterator<Item = (&R, &C)> {
    //     self.table.iter().flat_map(|(row, columns)| columns.keys().map(move |column| (row, column)))
    // }

    // pub(super) fn iter_over_rows_and_values(&self) -> impl Iterator<Item = (&R, &V)> {
    //     self.table.iter().flat_map(|(row, columns)| columns.values().map(move |value| (row, value)))
    // }

    // pub(super) fn iter_over_columns_and_values(&self) -> impl Iterator<Item = (&C, &V)> {
    //     self.table.values().flat_map(|row| row.iter())
    // }

    // pub(super) fn iter_over_rows_columns_and_values(&self) -> impl Iterator<Item = (&R, &C, &V)> {
    //     self.table.iter().flat_map(|(row, columns)| columns.iter().map(move |(column, value)| (row, column, value)))
    // }
}

#[derive(Debug, Clone)]
pub struct DFA<S, T> {
    states: Vec<S>,
    transitions: HashMap<S, HashMap<T, S>>,
}

impl<S, T> DFA<S, T> 
where S: Clone + Eq + Hash,
      T: Copy + Eq + Hash
{
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            transitions: HashMap::new(),
        }
    }

    pub fn add_state(&mut self, state: S) {
        self.states.push(state);
    }

    pub fn add_transition(&mut self, from: S, to: S, on: T) {
        let transitions = self.transitions.entry(from).or_insert_with(HashMap::new);
        transitions.insert(on, to);
    }

    pub fn enumerate_states(&self) -> impl Iterator<Item = (usize, &S)> {
        self.states.iter().enumerate()
    }

    pub fn transition_from_on(&self, state: &S, on: T) -> Option< &S> {
        self.transitions
            .get(&state)
            .and_then(|transitions| transitions.get(&on))
    }

    pub fn lookup_state_id(&self, state: &S) -> usize {
        self.states.iter().position(|s| s == state).unwrap()
    }

    // pub fn states(&self) -> impl Iterator<Item = &S> {
    //     self.states.iter()
    // }

    // pub fn transitions(&self) -> impl Iterator<Item = (&S, &HashMap<T, S>)> {
    //     self.transitions.iter()
    // }

    // pub fn transitions_from(&self, state: S) -> impl Iterator<Item = (&T, &S)> {
    //     self.transitions.get(&state).into_iter().flat_map(|transitions| transitions.iter())
    // }



    pub fn number_of_states(&self) -> usize {
        self.states.len()
    }
}
    