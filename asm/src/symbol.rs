use std::collections::HashMap;

pub type SymbolId = u32;

pub struct Symbol {
    pub value: Option<i64>,
}

pub struct SymbolTable {
    map: HashMap<String, SymbolId>,
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            symbols: Vec::new(),
        }
    }

    pub fn intern(&mut self, name: &str) -> SymbolId {
        if let Some(&id) = self.map.get(name) {
            return id;
        }

        let id = self.symbols.len() as SymbolId;
        self.symbols.push(Symbol { value: None });
        self.map.insert(name.to_string(), id);
        id
    }

    pub fn define(&mut self, id: SymbolId, value: i64) -> Result<(), ()> {
        if self.symbols[id as usize].value.is_some() {
            return Err(());
        }
        self.symbols[id as usize].value = Some(value);
        Ok(())
    }

    pub fn get(&self, id: SymbolId) -> Option<i64> {
        self.symbols[id as usize].value
    }
}
