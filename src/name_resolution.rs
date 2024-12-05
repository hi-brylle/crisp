use std::collections::HashSet;

use crate::symbol_table::{Symbol, SymbolTable};



pub fn check_redeclarations(symbol_table: &SymbolTable) -> Vec<Symbol>  {
    let mut duped_symbols: Vec<Symbol> = vec![];
    let mut temp: HashSet<String> = HashSet::new();
    
    for symbol in &symbol_table.symbol_table {
        if !temp.insert(symbol.scope_address.clone()) {
            duped_symbols.push(symbol.clone());
        }
    }
    
    duped_symbols
}