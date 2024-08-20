use std::fmt;

use proc_macro2::{Literal, TokenTree};
use syn::{ItemStruct, MetaList};

pub struct Table {
    namespace: Option<String>,
    name: String,
    columns: Vec<Column>,
    description: Option<String>,
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Table {{ namespace: {:?}, name: {}, columns: {:?}, description: {:?} }}",
            self.namespace, self.name, self.columns, self.description
        )
    }
}

pub struct Column {
    name: String,
    data_type: String,
    nullable: bool,
    flag: String,
}

impl fmt::Debug for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Column {{ name: {}, data_type: {}, nullable: {}, flag: {} }}",
            self.name, self.data_type, self.nullable, self.flag
        )
    }
}

impl Table {
    pub fn from_item_struct(item: ItemStruct) -> Table {
        use std::collections::HashMap;

        let mut attr_map = HashMap::new();

        for attr in &item.attrs {
            match attr.meta.clone() {
                syn::Meta::List(MetaList { path, tokens, .. }) if path.is_ident("sea_orm") => {
                    let mut table_name: String = String::new();
                    tokens.into_iter().all(|token| match token {
                        TokenTree::Ident(ident) if ident.to_string() == "table_name" => true,
                        TokenTree::Literal(lit) => {
                            table_name = Literal::to_string(&lit);
                            true
                        }
                        TokenTree::Punct(_) => true,
                        TokenTree::Group(_) => false,
                        _ => false,
                    });
                    attr_map.insert("table_name".to_string(), table_name);
                }
                _ => continue,
            }

        }

        Table {
            namespace: attr_map.get("namespace").cloned(),
            name: attr_map.get("table_name").cloned().unwrap(),
            columns: Vec::new(),
            description: attr_map.get("description").cloned(),
        }
    }
}
