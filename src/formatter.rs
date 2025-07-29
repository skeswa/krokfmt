use anyhow::Result;
use std::collections::{HashMap, HashSet};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitMut, VisitMutWith, VisitWith};

use crate::transformer::{sort_imports, ImportAnalyzer, ImportCategory};

pub struct KrokFormatter;

/// Analyzes exports in a module to determine which members are exported
#[derive(Default)]
pub struct ExportAnalyzer {
    exported_names: HashSet<String>,
}

impl ExportAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&mut self, module: &Module) -> ExportInfo {
        self.exported_names.clear();
        module.visit_with(self);

        ExportInfo {
            exported_names: self.exported_names.clone(),
        }
    }
}

impl Visit for ExportAnalyzer {
    fn visit_module_decl(&mut self, decl: &ModuleDecl) {
        match decl {
            ModuleDecl::ExportDecl(export_decl) => match &export_decl.decl {
                Decl::Fn(fn_decl) => {
                    self.exported_names.insert(fn_decl.ident.sym.to_string());
                }
                Decl::Class(class_decl) => {
                    self.exported_names.insert(class_decl.ident.sym.to_string());
                }
                Decl::Var(var_decl) => {
                    for decl in &var_decl.decls {
                        if let Pat::Ident(ident) = &decl.name {
                            self.exported_names.insert(ident.id.sym.to_string());
                        }
                    }
                }
                Decl::TsInterface(interface) => {
                    self.exported_names.insert(interface.id.sym.to_string());
                }
                Decl::TsTypeAlias(type_alias) => {
                    self.exported_names.insert(type_alias.id.sym.to_string());
                }
                Decl::TsEnum(ts_enum) => {
                    self.exported_names.insert(ts_enum.id.sym.to_string());
                }
                _ => {}
            },
            ModuleDecl::ExportNamed(named_export) => {
                for spec in &named_export.specifiers {
                    match spec {
                        ExportSpecifier::Named(named_spec) => {
                            let name = match &named_spec.orig {
                                ModuleExportName::Ident(ident) => ident.sym.to_string(),
                                ModuleExportName::Str(_) => continue,
                            };
                            self.exported_names.insert(name);
                        }
                        ExportSpecifier::Default(_) => {}
                        ExportSpecifier::Namespace(_) => {}
                    }
                }
            }
            ModuleDecl::ExportDefaultDecl(_) => {
                // Default exports don't add to exported names
            }
            ModuleDecl::ExportDefaultExpr(export) => {
                // Check if the expression is an identifier
                if let Expr::Ident(ident) = export.expr.as_ref() {
                    self.exported_names.insert(ident.sym.to_string());
                }
            }
            _ => {}
        }

        decl.visit_children_with(self);
    }
}

/// Holds information about exported members in a module
pub struct ExportInfo {
    exported_names: HashSet<String>,
}

impl ExportInfo {
    pub fn is_exported(&self, name: &str) -> bool {
        self.exported_names.contains(name)
    }
}

/// Analyzes dependencies between declarations in a module
#[derive(Default)]
pub struct DependencyAnalyzer {
    /// Maps declaration names to their dependencies
    dependencies: HashMap<String, HashSet<String>>,
    /// Current declaration being analyzed
    current_decl: Option<String>,
    /// All available declaration names in the module
    available_decls: HashSet<String>,
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&mut self, module: &Module) -> DependencyGraph {
        self.dependencies.clear();
        self.available_decls.clear();

        // First pass: collect all declaration names
        for item in &module.body {
            if let Some(name) = Self::get_declaration_name(item) {
                self.available_decls.insert(name);
            }
        }

        // Second pass: analyze dependencies
        for item in &module.body {
            if let Some(name) = Self::get_declaration_name(item) {
                self.current_decl = Some(name.clone());
                self.dependencies.insert(name, HashSet::new());
                item.visit_with(self);
                self.current_decl = None;
            }
        }

        DependencyGraph {
            dependencies: self.dependencies.clone(),
        }
    }

    fn get_declaration_name(item: &ModuleItem) -> Option<String> {
        match item {
            ModuleItem::Stmt(stmt) => Self::get_stmt_declaration_name(stmt),
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                Self::get_decl_name(&export_decl.decl)
            }
            _ => None,
        }
    }

    fn get_stmt_declaration_name(stmt: &Stmt) -> Option<String> {
        match stmt {
            Stmt::Decl(decl) => Self::get_decl_name(decl),
            _ => None,
        }
    }

    fn get_decl_name(decl: &Decl) -> Option<String> {
        match decl {
            Decl::Fn(fn_decl) => Some(fn_decl.ident.sym.to_string()),
            Decl::Class(class_decl) => Some(class_decl.ident.sym.to_string()),
            Decl::Var(var_decl) => {
                // For simplicity, return the first variable name
                var_decl.decls.first().and_then(|decl| {
                    if let Pat::Ident(ident) = &decl.name {
                        Some(ident.id.sym.to_string())
                    } else {
                        None
                    }
                })
            }
            Decl::TsInterface(interface) => Some(interface.id.sym.to_string()),
            Decl::TsTypeAlias(type_alias) => Some(type_alias.id.sym.to_string()),
            Decl::TsEnum(ts_enum) => Some(ts_enum.id.sym.to_string()),
            _ => None,
        }
    }
}

impl Visit for DependencyAnalyzer {
    fn visit_ident(&mut self, ident: &Ident) {
        if let Some(current) = &self.current_decl {
            let name = ident.sym.to_string();
            // Only track dependencies on declarations in this module
            if self.available_decls.contains(&name) && &name != current {
                self.dependencies.get_mut(current).unwrap().insert(name);
            }
        }
        ident.visit_children_with(self);
    }

    fn visit_ts_entity_name(&mut self, entity: &TsEntityName) {
        if let TsEntityName::Ident(ident) = entity {
            self.visit_ident(ident);
        }
        entity.visit_children_with(self);
    }
}

/// Represents the dependency graph of a module
pub struct DependencyGraph {
    pub dependencies: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    /// Returns true if `from` depends on `to`
    pub fn depends_on(&self, from: &str, to: &str) -> bool {
        self.dependencies
            .get(from)
            .map(|deps| deps.contains(to))
            .unwrap_or(false)
    }

    /// Performs a topological sort of the given items based on dependencies
    /// Returns None if there's a circular dependency
    pub fn topological_sort(&self, items: Vec<String>) -> Option<Vec<String>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        for item in &items {
            if !visited.contains(item)
                && !self.visit_node(item, &items, &mut visited, &mut visiting, &mut result)
            {
                return None; // Circular dependency detected
            }
        }

        result.reverse();
        Some(result)
    }

    fn visit_node(
        &self,
        node: &str,
        items: &[String],
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> bool {
        if visiting.contains(node) {
            return false; // Circular dependency
        }

        if visited.contains(node) {
            return true;
        }

        visiting.insert(node.to_string());

        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if items.contains(dep) && !self.visit_node(dep, items, visited, visiting, result) {
                    return false;
                }
            }
        }

        visiting.remove(node);
        visited.insert(node.to_string());
        result.push(node.to_string());

        true
    }
}

impl Default for KrokFormatter {
    fn default() -> Self {
        Self
    }
}

impl KrokFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format(&self, mut module: Module) -> Result<Module> {
        // Step 1: Extract and categorize imports
        let import_infos = ImportAnalyzer::new().analyze(&module);
        let sorted_imports = sort_imports(import_infos);

        // Step 2: Analyze exports and dependencies
        let mut export_analyzer = ExportAnalyzer::new();
        let export_info = export_analyzer.analyze(&module);

        let mut dependency_analyzer = DependencyAnalyzer::new();
        let dependency_graph = dependency_analyzer.analyze(&module);

        // Step 3: Separate imports and exports from other items
        let (import_export_items, other_items): (Vec<_>, Vec<_>) =
            module.body.into_iter().partition(|item| {
                matches!(
                    item,
                    ModuleItem::ModuleDecl(ModuleDecl::Import(_))
                        | ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(_))
                        | ModuleItem::ModuleDecl(ModuleDecl::ExportAll(_))
                        | ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(_))
                        | ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(_))
                )
            });

        // Further separate imports from exports
        let (_imports, exports): (Vec<_>, Vec<_>) = import_export_items
            .into_iter()
            .partition(|item| matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))));

        // Step 4: Prioritize exports while preserving dependencies
        let prioritized_items =
            self.prioritize_exports(other_items, &export_info, &dependency_graph)?;

        // Step 5: Reconstruct module with organized imports and prioritized declarations
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

        // Add prioritized items
        new_body.extend(prioritized_items);

        // Add exports at the end
        new_body.extend(exports);

        module.body = new_body;

        // Apply other transformations
        let mut formatter = FormatterVisitor::new();
        module.visit_mut_with(&mut formatter);

        Ok(module)
    }

    fn prioritize_exports(
        &self,
        items: Vec<ModuleItem>,
        export_info: &ExportInfo,
        dependency_graph: &DependencyGraph,
    ) -> Result<Vec<ModuleItem>> {
        // Create ordered lists and a map for lookup
        let mut ordered_items = Vec::new();
        let mut name_to_item: HashMap<String, ModuleItem> = HashMap::new();
        let mut other_items = Vec::new();

        // Maintain original order while building the map
        for item in items {
            if let Some(name) = Self::get_item_name(&item) {
                ordered_items.push(name.clone());
                name_to_item.insert(name, item);
            } else {
                other_items.push(item);
            }
        }

        // Separate exported and non-exported names while preserving order
        let mut exported_names = Vec::new();
        let mut non_exported_names = Vec::new();

        for name in &ordered_items {
            if export_info.is_exported(name) {
                exported_names.push(name.clone());
            } else {
                non_exported_names.push(name.clone());
            }
        }

        // Build the final list, respecting dependencies
        let mut result = Vec::new();
        let mut added = HashSet::new();

        // Helper function to add an item and its dependencies
        fn add_with_dependencies(
            name: &str,
            name_to_item: &mut HashMap<String, ModuleItem>,
            dependency_graph: &DependencyGraph,
            result: &mut Vec<ModuleItem>,
            added: &mut HashSet<String>,
            ordered_items: &[String],
        ) {
            if added.contains(name) || !name_to_item.contains_key(name) {
                return;
            }

            // First add dependencies
            if let Some(deps) = dependency_graph.dependencies.get(name) {
                let mut sorted_deps: Vec<_> = deps.iter().cloned().collect();
                // Sort dependencies by their original order
                sorted_deps.sort_by_key(|dep| {
                    ordered_items
                        .iter()
                        .position(|n| n == dep)
                        .unwrap_or(usize::MAX)
                });

                for dep in sorted_deps {
                    add_with_dependencies(
                        &dep,
                        name_to_item,
                        dependency_graph,
                        result,
                        added,
                        ordered_items,
                    );
                }
            }

            // Then add the item itself
            if let Some(item) = name_to_item.remove(name) {
                result.push(item);
                added.insert(name.to_string());
            }
        }

        // Add exported items with their dependencies
        for name in &exported_names {
            add_with_dependencies(
                name,
                &mut name_to_item,
                dependency_graph,
                &mut result,
                &mut added,
                &ordered_items,
            );
        }

        // Add non-exported items with their dependencies
        for name in &non_exported_names {
            add_with_dependencies(
                name,
                &mut name_to_item,
                dependency_graph,
                &mut result,
                &mut added,
                &ordered_items,
            );
        }

        // Add remaining items (like expression statements)
        result.extend(other_items);

        Ok(result)
    }

    fn get_item_name(item: &ModuleItem) -> Option<String> {
        match item {
            ModuleItem::Stmt(Stmt::Decl(decl)) => DependencyAnalyzer::get_decl_name(decl),
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                DependencyAnalyzer::get_decl_name(&export_decl.decl)
            }
            _ => None,
        }
    }
}

struct FormatterVisitor;

impl FormatterVisitor {
    fn new() -> Self {
        Self
    }

    fn sort_object_props(&self, props: &mut [PropOrSpread]) {
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

    fn sort_object_pattern_props(&self, props: &mut [ObjectPatProp]) {
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

    fn sort_class_members(&self, members: &mut [ClassMember]) {
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

    fn sort_union_types(&self, types: &mut [Box<TsType>]) {
        types.sort_by(|a, b| {
            let key_a = self.get_type_sort_key(a);
            let key_b = self.get_type_sort_key(b);
            key_a.cmp(&key_b)
        });
    }

    fn sort_intersection_types(&self, types: &mut [Box<TsType>]) {
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

    fn is_string_enum(&self, members: &[TsEnumMember]) -> bool {
        // An enum is a string enum if ALL members have string literal initializers
        // or if no members have numeric initializers
        let mut has_string_init = false;
        let mut has_numeric_init = false;
        let mut has_no_init = false;

        for member in members {
            match &member.init {
                Some(init) => match &**init {
                    Expr::Lit(Lit::Str(_)) => has_string_init = true,
                    Expr::Lit(Lit::Num(_)) => has_numeric_init = true,
                    _ => return false, // Complex expression, don't sort
                },
                None => has_no_init = true,
            }
        }

        // Don't sort if we have both string and numeric initializers
        if has_string_init && has_numeric_init {
            return false;
        }

        // Don't sort if we have numeric initializers or implicit numeric values
        if has_numeric_init || has_no_init {
            return false;
        }

        // Only sort if all members have string initializers
        has_string_init
    }

    fn sort_enum_members(&self, members: &mut [TsEnumMember]) {
        members.sort_by(|a, b| {
            let key_a =
                a.id.as_ident()
                    .map(|ident| ident.sym.to_string())
                    .unwrap_or_default();
            let key_b =
                b.id.as_ident()
                    .map(|ident| ident.sym.to_string())
                    .unwrap_or_default();
            key_a.cmp(&key_b)
        });
    }

    fn sort_jsx_attributes(&self, attrs: &mut [JSXAttrOrSpread]) {
        attrs.sort_by(|a, b| {
            let (cat_a, key_a) = self.categorize_jsx_attr(a);
            let (cat_b, key_b) = self.categorize_jsx_attr(b);

            match cat_a.cmp(&cat_b) {
                std::cmp::Ordering::Equal => key_a.cmp(&key_b),
                other => other,
            }
        });
    }

    fn categorize_jsx_attr(&self, attr: &JSXAttrOrSpread) -> (u8, String) {
        match attr {
            JSXAttrOrSpread::JSXAttr(jsx_attr) => {
                match &jsx_attr.name {
                    JSXAttrName::Ident(ident) => {
                        let name = ident.sym.to_string();
                        match name.as_str() {
                            "key" => (0, name), // key always first
                            "ref" => (1, name), // ref second
                            s if s.starts_with("on")
                                && s.len() > 2
                                && s.chars().nth(2).unwrap().is_uppercase() =>
                            {
                                (3, name) // Event handlers grouped
                            }
                            _ => (2, name), // Regular props alphabetically
                        }
                    }
                    _ => (2, String::new()),
                }
            }
            JSXAttrOrSpread::SpreadElement(_) => (4, String::from("...")), // Spreads at the end
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
        if let TsType::TsUnionOrIntersectionType(union_or_intersection) = ts_type {
            match union_or_intersection {
                TsUnionOrIntersectionType::TsUnionType(union) => {
                    self.sort_union_types(&mut union.types);
                }
                TsUnionOrIntersectionType::TsIntersectionType(intersection) => {
                    self.sort_intersection_types(&mut intersection.types);
                }
            }
        }
        ts_type.visit_mut_children_with(self);
    }

    fn visit_mut_ts_enum_decl(&mut self, ts_enum: &mut TsEnumDecl) {
        // Only sort if it's a string enum
        if self.is_string_enum(&ts_enum.members) {
            self.sort_enum_members(&mut ts_enum.members);
        }
        ts_enum.visit_mut_children_with(self);
    }

    fn visit_mut_jsx_opening_element(&mut self, jsx_opening: &mut JSXOpeningElement) {
        self.sort_jsx_attributes(&mut jsx_opening.attrs);
        jsx_opening.visit_mut_children_with(self);
    }

    // TODO: Add more visit methods for other sortable elements
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;

    fn format_source(source: &str) -> Result<Module> {
        let parser = TypeScriptParser::new();
        // Detect JSX and use .tsx extension if needed
        let filename = if source.contains("<") && (source.contains("/>") || source.contains("</")) {
            "test.tsx"
        } else {
            "test.ts"
        };
        let module = parser.parse(source, filename)?;
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

    #[test]
    fn test_enum_member_sorting_string_enum() {
        let source = r#"
enum Status {
    Pending = "pending",
    Active = "active",
    Disabled = "disabled",
    Archived = "archived"
}

enum Color {
    Red = "RED",
    Blue = "BLUE",
    Green = "GREEN",
    Yellow = "YELLOW"
}
"#;

        let formatted = format_source(source).unwrap();

        // Find the enums
        let enums: Vec<_> = formatted
            .body
            .iter()
            .filter_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::TsEnum(ts_enum))) => Some(ts_enum),
                _ => None,
            })
            .collect();

        assert_eq!(enums.len(), 2);

        // Check Status enum members are sorted
        let status_members: Vec<String> = enums[0]
            .members
            .iter()
            .map(|member| member.id.as_ident().unwrap().sym.to_string())
            .collect();
        assert_eq!(
            status_members,
            vec!["Active", "Archived", "Disabled", "Pending"]
        );

        // Check Color enum members are sorted
        let color_members: Vec<String> = enums[1]
            .members
            .iter()
            .map(|member| member.id.as_ident().unwrap().sym.to_string())
            .collect();
        assert_eq!(color_members, vec!["Blue", "Green", "Red", "Yellow"]);
    }

    #[test]
    fn test_enum_member_sorting_numeric_enum_preserved() {
        let source = r#"
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4
}

enum HttpStatus {
    NotFound = 404,
    OK = 200,
    ServerError = 500,
    BadRequest = 400
}
"#;

        let formatted = format_source(source).unwrap();

        // Find the enums
        let enums: Vec<_> = formatted
            .body
            .iter()
            .filter_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::TsEnum(ts_enum))) => Some(ts_enum),
                _ => None,
            })
            .collect();

        assert_eq!(enums.len(), 2);

        // Check Priority enum members are NOT sorted (preserved original order)
        let priority_members: Vec<String> = enums[0]
            .members
            .iter()
            .map(|member| member.id.as_ident().unwrap().sym.to_string())
            .collect();
        assert_eq!(priority_members, vec!["Low", "Medium", "High", "Critical"]);

        // Check HttpStatus enum members are NOT sorted (preserved original order)
        let status_members: Vec<String> = enums[1]
            .members
            .iter()
            .map(|member| member.id.as_ident().unwrap().sym.to_string())
            .collect();
        assert_eq!(
            status_members,
            vec!["NotFound", "OK", "ServerError", "BadRequest"]
        );
    }

    #[test]
    fn test_enum_member_sorting_mixed_enum() {
        let source = r#"
enum Mixed {
    First,
    Second = 10,
    Third,
    Fourth = "fourth",
    Fifth = "fifth"
}
"#;

        let formatted = format_source(source).unwrap();

        // Find the enum
        let ts_enum = formatted
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::TsEnum(ts_enum))) => Some(ts_enum),
                _ => None,
            })
            .unwrap();

        // Mixed enums should not be sorted to preserve value assignments
        let members: Vec<String> = ts_enum
            .members
            .iter()
            .map(|member| member.id.as_ident().unwrap().sym.to_string())
            .collect();
        assert_eq!(members, vec!["First", "Second", "Third", "Fourth", "Fifth"]);
    }

    #[test]
    fn test_jsx_property_sorting_basic() {
        let source = r#"
const Component = () => {
    return (
        <div 
            className="container"
            onClick={handleClick}
            id="main"
            style={styles}
            key="unique"
            ref={divRef}
            data-testid="test"
        />
    );
};
"#;

        let formatted = format_source(source).unwrap();

        // Find the JSX element
        let jsx_element = find_jsx_element(&formatted);

        // Get prop names in order
        let prop_names: Vec<String> = jsx_element
            .opening
            .attrs
            .iter()
            .filter_map(|attr| match attr {
                JSXAttrOrSpread::JSXAttr(jsx_attr) => match &jsx_attr.name {
                    JSXAttrName::Ident(ident) => Some(ident.sym.to_string()),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        // key and ref should be first, then alphabetically sorted, then event handlers
        assert_eq!(
            prop_names,
            vec![
                "key",
                "ref",
                "className",
                "data-testid",
                "id",
                "style",
                "onClick"
            ]
        );
    }

    #[test]
    fn test_jsx_property_sorting_event_handlers() {
        let source = r#"
const Button = () => (
    <button
        type="submit"
        onMouseEnter={handleEnter}
        className="btn"
        onClick={handleClick}
        disabled={false}
        onMouseLeave={handleLeave}
        aria-label="Submit"
        onChange={handleChange}
        key="btn1"
    />
);
"#;

        let formatted = format_source(source).unwrap();

        // Find the JSX element
        let jsx_element = find_jsx_element(&formatted);

        // Get prop names in order
        let prop_names: Vec<String> = jsx_element
            .opening
            .attrs
            .iter()
            .filter_map(|attr| match attr {
                JSXAttrOrSpread::JSXAttr(jsx_attr) => match &jsx_attr.name {
                    JSXAttrName::Ident(ident) => Some(ident.sym.to_string()),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        // key first, then alphabetically sorted with event handlers grouped
        assert_eq!(
            prop_names,
            vec![
                "key",
                "aria-label",
                "className",
                "disabled",
                "type",
                "onChange",
                "onClick",
                "onMouseEnter",
                "onMouseLeave"
            ]
        );
    }

    #[test]
    fn test_jsx_property_sorting_with_spread() {
        let source = r#"
const Card = (props) => (
    <div
        {...defaultProps}
        className="card"
        id={props.id}
        {...props}
        style={customStyle}
        ref={cardRef}
        key={props.key}
    />
);
"#;

        let formatted = format_source(source).unwrap();

        // Find the JSX element
        let jsx_element = find_jsx_element(&formatted);

        // Check attribute order - key/ref first, regular props sorted, spreads at end
        let attrs: Vec<String> = jsx_element
            .opening
            .attrs
            .iter()
            .map(|attr| match attr {
                JSXAttrOrSpread::JSXAttr(jsx_attr) => match &jsx_attr.name {
                    JSXAttrName::Ident(ident) => ident.sym.to_string(),
                    _ => "".to_string(),
                },
                JSXAttrOrSpread::SpreadElement(_) => "...spread".to_string(),
            })
            .collect();

        assert_eq!(
            attrs,
            vec![
                "key",
                "ref",
                "className",
                "id",
                "style",
                "...spread",
                "...spread"
            ]
        );
    }

    fn find_jsx_element(module: &Module) -> &JSXElement {
        for item in &module.body {
            if let ModuleItem::Stmt(stmt) = item {
                if let Some(jsx) = find_jsx_in_stmt(stmt) {
                    return jsx;
                }
            }
        }
        panic!("No JSX element found");
    }

    fn find_jsx_in_stmt(stmt: &Stmt) -> Option<&JSXElement> {
        match stmt {
            Stmt::Decl(Decl::Var(var_decl)) => {
                for decl in &var_decl.decls {
                    if let Some(init) = &decl.init {
                        if let Some(jsx) = find_jsx_in_expr(init) {
                            return Some(jsx);
                        }
                    }
                }
            }
            Stmt::Return(ret_stmt) => {
                if let Some(arg) = &ret_stmt.arg {
                    if let Some(jsx) = find_jsx_in_expr(arg) {
                        return Some(jsx);
                    }
                }
            }
            _ => {}
        }
        None
    }

    fn find_jsx_in_expr(expr: &Expr) -> Option<&JSXElement> {
        match expr {
            Expr::JSXElement(jsx) => Some(jsx),
            Expr::Paren(paren) => find_jsx_in_expr(&paren.expr),
            Expr::Arrow(arrow) => match &*arrow.body {
                BlockStmtOrExpr::Expr(expr) => find_jsx_in_expr(expr),
                BlockStmtOrExpr::BlockStmt(block) => {
                    for stmt in &block.stmts {
                        if let Some(jsx) = find_jsx_in_stmt(stmt) {
                            return Some(jsx);
                        }
                    }
                    None
                }
            },
            _ => None,
        }
    }

    #[test]
    fn test_export_detection_functions() {
        let source = r#"
export function publicFunc() {}
function privateFunc() {}
export const publicArrow = () => {};
const privateArrow = () => {};
"#;

        let module = TypeScriptParser::new().parse(source, "test.ts").unwrap();
        let mut analyzer = ExportAnalyzer::new();
        let export_info = analyzer.analyze(&module);

        assert!(export_info.is_exported("publicFunc"));
        assert!(!export_info.is_exported("privateFunc"));
        assert!(export_info.is_exported("publicArrow"));
        assert!(!export_info.is_exported("privateArrow"));
    }

    #[test]
    fn test_export_detection_classes() {
        let source = r#"
export class PublicClass {}
class PrivateClass {}
export interface PublicInterface {}
interface PrivateInterface {}
"#;

        let module = TypeScriptParser::new().parse(source, "test.ts").unwrap();
        let mut analyzer = ExportAnalyzer::new();
        let export_info = analyzer.analyze(&module);

        assert!(export_info.is_exported("PublicClass"));
        assert!(!export_info.is_exported("PrivateClass"));
        assert!(export_info.is_exported("PublicInterface"));
        assert!(!export_info.is_exported("PrivateInterface"));
    }

    #[test]
    fn test_export_detection_types_and_enums() {
        let source = r#"
export type PublicType = string;
type PrivateType = number;
export enum PublicEnum { A, B }
enum PrivateEnum { X, Y }
"#;

        let module = TypeScriptParser::new().parse(source, "test.ts").unwrap();
        let mut analyzer = ExportAnalyzer::new();
        let export_info = analyzer.analyze(&module);

        assert!(export_info.is_exported("PublicType"));
        assert!(!export_info.is_exported("PrivateType"));
        assert!(export_info.is_exported("PublicEnum"));
        assert!(!export_info.is_exported("PrivateEnum"));
    }

    #[test]
    fn test_export_detection_named_exports() {
        let source = r#"
const a = 1;
const b = 2;
function c() {}
class D {}

export { a, b as bee, c };
"#;

        let module = TypeScriptParser::new().parse(source, "test.ts").unwrap();
        let mut analyzer = ExportAnalyzer::new();
        let export_info = analyzer.analyze(&module);

        assert!(export_info.is_exported("a"));
        assert!(export_info.is_exported("b"));
        assert!(export_info.is_exported("c"));
        assert!(!export_info.is_exported("D"));
    }

    #[test]
    fn test_export_detection_default_export() {
        let source = r#"
function myFunc() {}
export default myFunc;

const obj = { x: 1 };
export { obj as default };
"#;

        let module = TypeScriptParser::new().parse(source, "test.ts").unwrap();
        let mut analyzer = ExportAnalyzer::new();
        let export_info = analyzer.analyze(&module);

        assert!(export_info.is_exported("myFunc"));
        assert!(export_info.is_exported("obj"));
    }

    #[test]
    fn test_export_prioritization_basic() {
        let source = r#"
function privateFunc() {
    return "private";
}

export function publicFunc() {
    return privateFunc();
}

const privateConst = 10;

export const publicConst = privateConst + 5;
"#;

        let formatted = format_source(source).unwrap();

        // Find all function and variable declarations
        let mut declarations = Vec::new();
        for item in &formatted.body {
            match item {
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) => {
                    declarations.push(fn_decl.ident.sym.to_string());
                }
                ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => {
                    for decl in &var_decl.decls {
                        if let Pat::Ident(ident) = &decl.name {
                            declarations.push(ident.id.sym.to_string());
                        }
                    }
                }
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                    match &export_decl.decl {
                        Decl::Fn(fn_decl) => {
                            declarations.push(fn_decl.ident.sym.to_string());
                        }
                        Decl::Var(var_decl) => {
                            for decl in &var_decl.decls {
                                if let Pat::Ident(ident) = &decl.name {
                                    declarations.push(ident.id.sym.to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Exported items should come before non-exported ones, but dependencies must be preserved
        // privateFunc must come before publicFunc (dependency)
        // privateConst must come before publicConst (dependency)
        assert_eq!(
            declarations,
            vec!["privateFunc", "publicFunc", "privateConst", "publicConst"]
        );
    }

    #[test]
    fn test_export_prioritization_preserves_dependencies() {
        let source = r#"
const helper = () => "help";

export const publicFunc = () => helper();

function util() {
    return helper();
}

export function main() {
    return util();
}
"#;

        let formatted = format_source(source).unwrap();

        // Helper should stay before publicFunc because publicFunc depends on it
        // util should stay before main because main depends on it
        let mut declarations = Vec::new();
        for item in &formatted.body {
            match item {
                ModuleItem::Stmt(Stmt::Decl(decl)) => match decl {
                    Decl::Fn(fn_decl) => {
                        declarations.push(fn_decl.ident.sym.to_string());
                    }
                    Decl::Var(var_decl) => {
                        for decl in &var_decl.decls {
                            if let Pat::Ident(ident) = &decl.name {
                                declarations.push(ident.id.sym.to_string());
                            }
                        }
                    }
                    _ => {}
                },
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                    match &export_decl.decl {
                        Decl::Fn(fn_decl) => {
                            declarations.push(fn_decl.ident.sym.to_string());
                        }
                        Decl::Var(var_decl) => {
                            for decl in &var_decl.decls {
                                if let Pat::Ident(ident) = &decl.name {
                                    declarations.push(ident.id.sym.to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Dependencies must be preserved
        let helper_idx = declarations.iter().position(|s| s == "helper").unwrap();
        let public_func_idx = declarations.iter().position(|s| s == "publicFunc").unwrap();
        let util_idx = declarations.iter().position(|s| s == "util").unwrap();
        let main_idx = declarations.iter().position(|s| s == "main").unwrap();

        assert!(helper_idx < public_func_idx);
        assert!(util_idx < main_idx);
    }

    #[test]
    fn test_export_prioritization_with_classes_and_types() {
        let source = r#"
interface PrivateInterface {
    x: number;
}

export interface PublicInterface {
    y: string;
}

class PrivateClass {
    value = 10;
}

export class PublicClass extends PrivateClass {
    extra = 20;
}

type PrivateType = string | number;

export type PublicType = PrivateType | boolean;
"#;

        let formatted = format_source(source).unwrap();

        // Collect all declaration names
        let mut declarations = Vec::new();
        for item in &formatted.body {
            match item {
                ModuleItem::Stmt(Stmt::Decl(decl)) => match decl {
                    Decl::Class(class_decl) => {
                        declarations.push(class_decl.ident.sym.to_string());
                    }
                    Decl::TsInterface(interface) => {
                        declarations.push(interface.id.sym.to_string());
                    }
                    Decl::TsTypeAlias(type_alias) => {
                        declarations.push(type_alias.id.sym.to_string());
                    }
                    _ => {}
                },
                ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                    match &export_decl.decl {
                        Decl::Class(class_decl) => {
                            declarations.push(class_decl.ident.sym.to_string());
                        }
                        Decl::TsInterface(interface) => {
                            declarations.push(interface.id.sym.to_string());
                        }
                        Decl::TsTypeAlias(type_alias) => {
                            declarations.push(type_alias.id.sym.to_string());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Ensure dependencies are preserved
        let private_class_idx = declarations
            .iter()
            .position(|s| s == "PrivateClass")
            .unwrap();
        let public_class_idx = declarations
            .iter()
            .position(|s| s == "PublicClass")
            .unwrap();
        let private_type_idx = declarations
            .iter()
            .position(|s| s == "PrivateType")
            .unwrap();
        let public_type_idx = declarations.iter().position(|s| s == "PublicType").unwrap();

        // PublicClass depends on PrivateClass, so PrivateClass must come first
        assert!(private_class_idx < public_class_idx);
        // PublicType depends on PrivateType
        assert!(private_type_idx < public_type_idx);
    }
}
