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
    flag: Vec<String>,
}

impl fmt::Debug for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Column {{ name: {}, data_type: {}, flag: {:?} }}",
            self.name, self.data_type, self.flag
        )
    }
}

impl Column {
    fn to_mermaid_erd_text(&self) -> String {
        format!("{} {} {}", self.data_type, self.name, self.flag.join(", "))
    }
}

impl Table {
    pub fn to_mermaid_erd_text(&self) -> String {
        format!("
{} {{\n\t{}\n}}
", self.name, self.columns.iter().map(|column| column.to_mermaid_erd_text()).collect::<Vec<String>>().join("\n\t")
        )
    }

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

        let mut columns = Vec::new();
        if let syn::Fields::Named(named_fields) = item.fields {
            for field in named_fields.named {
                let name = field.ident.unwrap().to_string();
                let data_type = match &field.ty {
                    syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
                    _ => "Unknown".to_string(),
                };
                let mut flag = Vec::new();
                
                for attr in &field.attrs {
                    if let syn::Meta::List(MetaList { path, tokens, .. }) = &attr.meta {
                        if path.is_ident("sea_orm") {
                            tokens.clone().into_iter().for_each(|token| {
                                if let TokenTree::Ident(ident) = token {
                                    let flag_str = ident.to_string();
                                    if flag_str == "primary_key" {
                                        flag.push("PK".to_string());
                                    } else {
                                        flag.push(flag_str);
                                    }
                                }
                            });
                        }
                    }
                }

                columns.push(Column { name, data_type, flag });
            }
        }

        Table {
            namespace: attr_map.get("namespace").cloned(),
            name: attr_map.get("table_name").cloned().unwrap(),
            columns,
            description: attr_map.get("description").cloned(),
        }
    }
}
