use anyhow::Result;
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::transformer::{ImportAnalyzer, ImportCategory, sort_imports};

pub struct KrokFormatter;

impl KrokFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format(&self, mut module: Module) -> Result<Module> {
        // Step 1: Extract and categorize imports
        let import_infos = ImportAnalyzer::new().analyze(&module);
        let sorted_imports = sort_imports(import_infos);
        
        // Step 2: Separate imports from other items
        let (imports, mut other_items): (Vec<_>, Vec<_>) = module.body.into_iter()
            .partition(|item| matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))));
        
        // Step 3: Reconstruct module with organized imports
        let mut new_body = Vec::new();
        
        // Add imports grouped by category with empty lines between groups
        let mut last_category: Option<ImportCategory> = None;
        
        for import_info in sorted_imports {
            // Add empty line between different categories
            if let Some(last_cat) = &last_category {
                if std::mem::discriminant(last_cat) != std::mem::discriminant(&import_info.category) {
                    // We'll handle empty lines in the codegen phase
                }
            }
            
            new_body.push(ModuleItem::ModuleDecl(ModuleDecl::Import(import_info.import_decl)));
            last_category = Some(import_info.category);
        }
        
        // Add remaining items
        new_body.append(&mut other_items);
        
        module.body = new_body;
        
        // Apply other transformations
        let mut formatter = FormatterVisitor::new();
        module.visit_mut_with(&mut formatter);
        
        Ok(module)
    }
}

struct FormatterVisitor;

impl FormatterVisitor {
    fn new() -> Self {
        Self
    }
    
    fn sort_object_props(&self, props: &mut Vec<PropOrSpread>) {
        props.sort_by(|a, b| {
            let key_a = self.get_prop_key(a);
            let key_b = self.get_prop_key(b);
            key_a.cmp(&key_b)
        });
    }
    
    fn get_prop_key(&self, prop: &PropOrSpread) -> String {
        match prop {
            PropOrSpread::Prop(prop) => match &**prop {
                Prop::Shorthand(ident) => ident.sym.to_string(),
                Prop::KeyValue(kv) => match &kv.key {
                    PropName::Ident(ident) => ident.sym.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Num(n) => n.value.to_string(),
                    _ => String::new(),
                },
                _ => String::new(),
            },
            PropOrSpread::Spread(_) => String::from("..."), // Sort spreads to the end
        }
    }
}

impl VisitMut for FormatterVisitor {
    fn visit_mut_object_lit(&mut self, obj: &mut ObjectLit) {
        self.sort_object_props(&mut obj.props);
        obj.visit_mut_children_with(self);
    }
    
    // TODO: Add more visit methods for other sortable elements
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;

    fn format_source(source: &str) -> Result<Module> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts")?;
        KrokFormatter::new().format(module)
    }

    #[test]
    fn test_format_imports_grouped_and_sorted() {
        let source = r#"
import { z } from './utils';
import React from 'react';
import { Button } from '@ui/Button';
import axios from 'axios';
import { helper } from '../helper';
"#;
        
        let formatted = format_source(source).unwrap();
        
        // Verify imports are in correct order
        let imports: Vec<_> = formatted.body.iter()
            .filter_map(|item| match item {
                ModuleItem::ModuleDecl(ModuleDecl::Import(import)) => Some(import),
                _ => None,
            })
            .collect();
        
        assert_eq!(imports.len(), 5);
        assert_eq!(imports[0].src.value, "axios");
        assert_eq!(imports[1].src.value, "react");
        assert_eq!(imports[2].src.value, "@ui/Button");
        assert_eq!(imports[3].src.value, "../helper");
        assert_eq!(imports[4].src.value, "./utils");
    }

    #[test]
    fn test_format_object_properties_sorted() {
        let source = r#"
const obj = {
    zebra: 1,
    apple: 2,
    banana: 3,
    cat: 4
};
"#;
        
        let formatted = format_source(source).unwrap();
        
        // Find the object literal
        let obj_lit = formatted.body.iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => {
                    var_decl.decls.first().and_then(|decl| {
                        decl.init.as_ref().and_then(|init| match &**init {
                            Expr::Object(obj) => Some(obj),
                            _ => None,
                        })
                    })
                }
                _ => None,
            })
            .unwrap();
        
        // Verify properties are sorted
        let keys: Vec<_> = obj_lit.props.iter()
            .filter_map(|prop| match prop {
                PropOrSpread::Prop(prop) => match &**prop {
                    Prop::KeyValue(kv) => match &kv.key {
                        PropName::Ident(ident) => Some(ident.sym.to_string()),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            })
            .collect();
        
        assert_eq!(keys, vec!["apple", "banana", "cat", "zebra"]);
    }

    #[test]
    fn test_imports_remain_at_top() {
        let source = r#"
const x = 1;
import React from 'react';
const y = 2;
import { useState } from 'react';
"#;
        
        let formatted = format_source(source).unwrap();
        
        // First two items should be imports
        assert!(matches!(
            &formatted.body[0],
            ModuleItem::ModuleDecl(ModuleDecl::Import(_))
        ));
        assert!(matches!(
            &formatted.body[1],
            ModuleItem::ModuleDecl(ModuleDecl::Import(_))
        ));
        
        // Rest should be statements
        assert!(matches!(
            &formatted.body[2],
            ModuleItem::Stmt(_)
        ));
        assert!(matches!(
            &formatted.body[3],
            ModuleItem::Stmt(_)
        ));
    }
}