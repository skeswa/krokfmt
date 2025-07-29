use anyhow::Result;
use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::transformer::{sort_imports, ImportAnalyzer, ImportCategory};

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
        let (_imports, mut other_items): (Vec<_>, Vec<_>) = module
            .body
            .into_iter()
            .partition(|item| matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))));

        // Step 3: Reconstruct module with organized imports
        let mut new_body = Vec::new();

        // Add imports grouped by category with empty lines between groups
        let mut last_category: Option<ImportCategory> = None;

        for import_info in sorted_imports {
            // Add empty line between different categories
            if let Some(last_cat) = &last_category {
                if std::mem::discriminant(last_cat) != std::mem::discriminant(&import_info.category)
                {
                    // We'll handle empty lines in the codegen phase
                }
            }

            new_body.push(ModuleItem::ModuleDecl(ModuleDecl::Import(
                import_info.import_decl,
            )));
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

    fn sort_object_pattern_props(&self, props: &mut Vec<ObjectPatProp>) {
        props.sort_by(|a, b| {
            let key_a = self.get_object_pat_prop_key(a);
            let key_b = self.get_object_pat_prop_key(b);
            key_a.cmp(&key_b)
        });
    }

    fn get_object_pat_prop_key(&self, prop: &ObjectPatProp) -> String {
        match prop {
            ObjectPatProp::KeyValue(kv) => match &kv.key {
                PropName::Ident(ident) => ident.sym.to_string(),
                PropName::Str(s) => s.value.to_string(),
                PropName::Num(n) => n.value.to_string(),
                _ => String::new(),
            },
            ObjectPatProp::Assign(assign) => assign.key.sym.to_string(),
            ObjectPatProp::Rest(_) => String::from("..."), // Sort rest to the end
        }
    }

    fn sort_class_members(&self, members: &mut Vec<ClassMember>) {
        // Create a custom ordering for class members
        members.sort_by(|a, b| {
            use std::cmp::Ordering;

            // First, categorize members
            let (cat_a, key_a) = self.categorize_class_member(a);
            let (cat_b, key_b) = self.categorize_class_member(b);

            // Compare categories first
            match cat_a.cmp(&cat_b) {
                Ordering::Equal => {
                    // Within the same category, sort alphabetically by key
                    key_a.cmp(&key_b)
                }
                other => other,
            }
        });
    }

    fn categorize_class_member(&self, member: &ClassMember) -> (u8, String) {
        match member {
            ClassMember::ClassProp(prop) => {
                let key = prop
                    .key
                    .as_ident()
                    .map(|ident| ident.sym.to_string())
                    .unwrap_or_default();
                if prop.is_static {
                    (0, key) // Static fields first
                } else {
                    (1, key) // Instance fields second
                }
            }
            ClassMember::Constructor(_) => {
                (2, "constructor".to_string()) // Constructor third
            }
            ClassMember::Method(method) => {
                let key = method
                    .key
                    .as_ident()
                    .map(|ident| ident.sym.to_string())
                    .unwrap_or_default();
                if method.is_static {
                    (3, key) // Static methods fourth
                } else {
                    (4, key) // Instance methods last
                }
            }
            _ => (99, String::new()), // Other members at the end
        }
    }

    fn sort_union_types(&self, types: &mut Vec<Box<TsType>>) {
        types.sort_by(|a, b| {
            let key_a = self.get_type_sort_key(a);
            let key_b = self.get_type_sort_key(b);
            key_a.cmp(&key_b)
        });
    }

    fn sort_intersection_types(&self, types: &mut Vec<Box<TsType>>) {
        types.sort_by(|a, b| {
            let key_a = self.get_type_sort_key(a);
            let key_b = self.get_type_sort_key(b);
            key_a.cmp(&key_b)
        });
    }

    fn get_type_sort_key(&self, ts_type: &TsType) -> String {
        match ts_type {
            TsType::TsTypeRef(type_ref) => {
                match &type_ref.type_name {
                    TsEntityName::Ident(ident) => ident.sym.to_string(),
                    TsEntityName::TsQualifiedName(_) => String::from("~qualified"), // Sort qualified names later
                }
            }
            TsType::TsLitType(lit) => {
                match &lit.lit {
                    TsLit::Str(s) => s.value.to_string(),
                    TsLit::Number(n) => n.value.to_string(),
                    TsLit::Bool(b) => b.value.to_string(),
                    TsLit::BigInt(b) => b.value.to_string(),
                    TsLit::Tpl(_) => String::from("~template"), // Sort template literals later
                }
            }
            TsType::TsKeywordType(keyword) => {
                format!("~keyword:{:?}", keyword.kind) // Sort keywords by their kind
            }
            _ => String::from("~other"), // Sort other types to the end
        }
    }
}

impl VisitMut for FormatterVisitor {
    fn visit_mut_object_lit(&mut self, obj: &mut ObjectLit) {
        self.sort_object_props(&mut obj.props);
        obj.visit_mut_children_with(self);
    }

    fn visit_mut_param(&mut self, param: &mut Param) {
        // Sort object pattern destructuring in function parameters
        if let Pat::Object(obj_pat) = &mut param.pat {
            self.sort_object_pattern_props(&mut obj_pat.props);
        }
        param.visit_mut_children_with(self);
    }

    fn visit_mut_pat(&mut self, pat: &mut Pat) {
        // Handle object patterns in other contexts (like arrow functions)
        if let Pat::Object(obj_pat) = pat {
            self.sort_object_pattern_props(&mut obj_pat.props);
        }
        pat.visit_mut_children_with(self);
    }

    fn visit_mut_class(&mut self, class: &mut Class) {
        // Sort class members according to the rules
        self.sort_class_members(&mut class.body);
        class.visit_mut_children_with(self);
    }

    fn visit_mut_ts_type(&mut self, ts_type: &mut TsType) {
        match ts_type {
            TsType::TsUnionOrIntersectionType(union_or_intersection) => match union_or_intersection
            {
                TsUnionOrIntersectionType::TsUnionType(union) => {
                    self.sort_union_types(&mut union.types);
                }
                TsUnionOrIntersectionType::TsIntersectionType(intersection) => {
                    self.sort_intersection_types(&mut intersection.types);
                }
            },
            _ => {}
        }
        ts_type.visit_mut_children_with(self);
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
        let imports: Vec<_> = formatted
            .body
            .iter()
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
        let obj_lit = formatted
            .body
            .iter()
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
        let keys: Vec<_> = obj_lit
            .props
            .iter()
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
        assert!(matches!(&formatted.body[2], ModuleItem::Stmt(_)));
        assert!(matches!(&formatted.body[3], ModuleItem::Stmt(_)));
    }

    #[test]
    fn test_function_destructured_params_sorted() {
        let source = r#"
function process({ zebra, apple, banana }: Options) {
    return apple + banana + zebra;
}
"#;

        let formatted = format_source(source).unwrap();

        // Find the function declaration
        let func_decl = formatted
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) => Some(fn_decl),
                _ => None,
            })
            .unwrap();

        // Get the first parameter
        let param = &func_decl.function.params[0];

        // Verify it's an object pattern with sorted keys
        match &param.pat {
            Pat::Object(obj_pat) => {
                let keys: Vec<_> = obj_pat
                    .props
                    .iter()
                    .filter_map(|prop| match prop {
                        ObjectPatProp::KeyValue(kv) => match &kv.key {
                            PropName::Ident(ident) => Some(ident.sym.to_string()),
                            _ => None,
                        },
                        ObjectPatProp::Assign(assign) => Some(assign.key.sym.to_string()),
                        _ => None,
                    })
                    .collect();

                assert_eq!(keys, vec!["apple", "banana", "zebra"]);
            }
            _ => panic!("Expected object pattern"),
        }
    }

    #[test]
    fn test_arrow_function_destructured_params_sorted() {
        let source = r#"
const process = ({ zebra, apple, banana }: Options) => {
    return apple + banana + zebra;
};
"#;

        let formatted = format_source(source).unwrap();

        // Find the arrow function
        let arrow_func = formatted
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => {
                    var_decl.decls.first().and_then(|decl| {
                        decl.init.as_ref().and_then(|init| match &**init {
                            Expr::Arrow(arrow) => Some(arrow),
                            _ => None,
                        })
                    })
                }
                _ => None,
            })
            .unwrap();

        // Get the first parameter
        let param = &arrow_func.params[0];

        // Verify it's an object pattern with sorted keys
        match param {
            Pat::Object(obj_pat) => {
                let keys: Vec<_> = obj_pat
                    .props
                    .iter()
                    .filter_map(|prop| match prop {
                        ObjectPatProp::KeyValue(kv) => match &kv.key {
                            PropName::Ident(ident) => Some(ident.sym.to_string()),
                            _ => None,
                        },
                        ObjectPatProp::Assign(assign) => Some(assign.key.sym.to_string()),
                        _ => None,
                    })
                    .collect();

                assert_eq!(keys, vec!["apple", "banana", "zebra"]);
            }
            _ => panic!("Expected object pattern"),
        }
    }

    #[test]
    fn test_function_mixed_params_preserved() {
        let source = r#"
function process(id: number, { zebra, apple, banana }: Options, callback: Function) {
    return callback(id, apple + banana + zebra);
}
"#;

        let formatted = format_source(source).unwrap();

        // Find the function declaration
        let func_decl = formatted
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) => Some(fn_decl),
                _ => None,
            })
            .unwrap();

        // Verify parameter count
        assert_eq!(func_decl.function.params.len(), 3);

        // Verify middle parameter is sorted object pattern
        match &func_decl.function.params[1].pat {
            Pat::Object(obj_pat) => {
                let keys: Vec<_> = obj_pat
                    .props
                    .iter()
                    .filter_map(|prop| match prop {
                        ObjectPatProp::KeyValue(kv) => match &kv.key {
                            PropName::Ident(ident) => Some(ident.sym.to_string()),
                            _ => None,
                        },
                        ObjectPatProp::Assign(assign) => Some(assign.key.sym.to_string()),
                        _ => None,
                    })
                    .collect();

                assert_eq!(keys, vec!["apple", "banana", "zebra"]);
            }
            _ => panic!("Expected object pattern"),
        }
    }

    #[test]
    fn test_function_nested_destructuring_sorted() {
        let source = r#"
function process({ config: { zebra, apple, banana }, data }: NestedOptions) {
    return apple + banana + zebra;
}
"#;

        let formatted = format_source(source).unwrap();

        // Find the function declaration
        let func_decl = formatted
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) => Some(fn_decl),
                _ => None,
            })
            .unwrap();

        // Get the first parameter
        let param = &func_decl.function.params[0];

        // Verify outer object pattern has sorted keys
        match &param.pat {
            Pat::Object(obj_pat) => {
                let outer_keys: Vec<_> = obj_pat
                    .props
                    .iter()
                    .filter_map(|prop| match prop {
                        ObjectPatProp::KeyValue(kv) => match &kv.key {
                            PropName::Ident(ident) => Some(ident.sym.to_string()),
                            _ => None,
                        },
                        ObjectPatProp::Assign(assign) => Some(assign.key.sym.to_string()),
                        _ => None,
                    })
                    .collect();

                assert_eq!(outer_keys, vec!["config", "data"]);

                // Check nested object pattern
                if let Some(ObjectPatProp::KeyValue(kv)) = obj_pat.props.first() {
                    if let Pat::Object(nested_obj_pat) = kv.value.as_ref() {
                        let inner_keys: Vec<_> = nested_obj_pat
                            .props
                            .iter()
                            .filter_map(|prop| match prop {
                                ObjectPatProp::KeyValue(kv) => match &kv.key {
                                    PropName::Ident(ident) => Some(ident.sym.to_string()),
                                    _ => None,
                                },
                                ObjectPatProp::Assign(assign) => Some(assign.key.sym.to_string()),
                                _ => None,
                            })
                            .collect();

                        assert_eq!(inner_keys, vec!["apple", "banana", "zebra"]);
                    }
                }
            }
            _ => panic!("Expected object pattern"),
        }
    }

    #[test]
    fn test_class_member_sorting_basic() {
        let source = r#"
class User {
    private zebra: string;
    public apple: number;
    protected banana: boolean;
    
    constructor() {}
    
    private writeLog() {}
    public getInfo() {}
    protected checkAccess() {}
}
"#;

        let formatted = format_source(source).unwrap();

        // Find the class declaration
        let class_decl = formatted
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) => Some(class_decl),
                _ => None,
            })
            .unwrap();

        // Get member names in order
        let members: Vec<String> = class_decl
            .class
            .body
            .iter()
            .filter_map(|member| match member {
                ClassMember::ClassProp(prop) => {
                    prop.key.as_ident().map(|ident| ident.sym.to_string())
                }
                ClassMember::Method(method) => {
                    method.key.as_ident().map(|ident| ident.sym.to_string())
                }
                ClassMember::Constructor(_) => Some("constructor".to_string()),
                _ => None,
            })
            .collect();

        // Fields should be sorted alphabetically: apple, banana, zebra
        // Then constructor
        // Then methods sorted alphabetically: checkAccess, getInfo, writeLog
        assert_eq!(
            members,
            vec![
                "apple",
                "banana",
                "zebra",
                "constructor",
                "checkAccess",
                "getInfo",
                "writeLog"
            ]
        );
    }

    #[test]
    fn test_class_static_members_sorting() {
        let source = r#"
class Config {
    static zebra = "z";
    static apple = "a";
    
    instanceZebra = 100;
    instanceApple = 200;
    
    static getZebra() { return this.zebra; }
    static getApple() { return this.apple; }
    
    getInstanceData() { return this.instanceApple; }
}
"#;

        let formatted = format_source(source).unwrap();

        // Find the class
        let class_decl = formatted
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Class(class_decl))) => Some(class_decl),
                _ => None,
            })
            .unwrap();

        // Get members with static info
        let members: Vec<(String, bool)> = class_decl
            .class
            .body
            .iter()
            .filter_map(|member| match member {
                ClassMember::ClassProp(prop) => prop
                    .key
                    .as_ident()
                    .map(|ident| (ident.sym.to_string(), prop.is_static)),
                ClassMember::Method(method) => method
                    .key
                    .as_ident()
                    .map(|ident| (ident.sym.to_string(), method.is_static)),
                _ => None,
            })
            .collect();

        // Order should be:
        // 1. Static fields (sorted): apple, zebra
        // 2. Instance fields (sorted): instanceApple, instanceZebra
        // 3. Static methods (sorted): getApple, getZebra
        // 4. Instance methods: getInstanceData
        assert_eq!(
            members,
            vec![
                ("apple".to_string(), true),
                ("zebra".to_string(), true),
                ("instanceApple".to_string(), false),
                ("instanceZebra".to_string(), false),
                ("getApple".to_string(), true),
                ("getZebra".to_string(), true),
                ("getInstanceData".to_string(), false),
            ]
        );
    }

    #[test]
    fn test_union_type_sorting() {
        let source = r#"
type Status = 'error' | 'success' | 'pending' | 'idle';
type Size = 'xl' | 'sm' | 'lg' | 'md' | 'xs';
"#;

        let formatted = format_source(source).unwrap();

        // Find the type aliases
        let mut type_unions = Vec::new();
        for item in &formatted.body {
            if let ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(ts_type))) = item {
                if let TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsUnionType(
                    union,
                )) = ts_type.type_ann.as_ref()
                {
                    let union_members: Vec<String> = union
                        .types
                        .iter()
                        .filter_map(|t| {
                            if let TsType::TsLitType(lit) = t.as_ref() {
                                if let TsLit::Str(s) = &lit.lit {
                                    return Some(s.value.to_string());
                                }
                            }
                            None
                        })
                        .collect();
                    type_unions.push(union_members);
                }
            }
        }

        assert_eq!(type_unions.len(), 2);
        // Status type should be sorted: error, idle, pending, success
        assert_eq!(type_unions[0], vec!["error", "idle", "pending", "success"]);
        // Size type should be sorted: lg, md, sm, xl, xs
        assert_eq!(type_unions[1], vec!["lg", "md", "sm", "xl", "xs"]);
    }

    #[test]
    fn test_intersection_type_sorting() {
        let source = r#"
type Combined = Writable & Timestamped & Identifiable & Versioned;
"#;

        let formatted = format_source(source).unwrap();

        // Find the type alias with intersection
        let ts_type = formatted
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(ts_type))) => Some(ts_type),
                _ => None,
            })
            .unwrap();

        if let TsType::TsUnionOrIntersectionType(TsUnionOrIntersectionType::TsIntersectionType(
            intersection,
        )) = ts_type.type_ann.as_ref()
        {
            let members: Vec<String> = intersection
                .types
                .iter()
                .filter_map(|t| {
                    if let TsType::TsTypeRef(type_ref) = t.as_ref() {
                        if let TsEntityName::Ident(ident) = &type_ref.type_name {
                            return Some(ident.sym.to_string());
                        }
                    }
                    None
                })
                .collect();

            // Should be sorted: Identifiable, Timestamped, Versioned, Writable
            assert_eq!(
                members,
                vec!["Identifiable", "Timestamped", "Versioned", "Writable"]
            );
        } else {
            panic!("Expected intersection type");
        }
    }
}
