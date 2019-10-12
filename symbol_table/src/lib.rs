use std::collections::HashMap;

pub struct SymbolTable {
    symbol_map: HashMap<String, i16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbol_map: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, symbol: &str, address: i16) {
        self.symbol_map.insert(String::from(symbol), address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.symbol_map.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<i16> {
        self.symbol_map
            .get(symbol)
            .map_or(None, |v| Some(v.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_entry("SP", 0);
        symbol_table.add_entry("LCL", 1);

        assert_eq!(true, symbol_table.contains("SP"));
        assert_eq!(Some(0), symbol_table.get_address("SP"));

        assert_eq!(true, symbol_table.contains("LCL"));
        assert_eq!(Some(1), symbol_table.get_address("LCL"));

        assert_eq!(false, symbol_table.contains("ABC"));
        assert_eq!(None, symbol_table.get_address("ABC"));
    }
}
