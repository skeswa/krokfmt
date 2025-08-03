use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

/// Generates semantic hashes for AST nodes that are stable across transformations.
/// These hashes identify nodes by their semantic properties rather than positions.
#[derive(Default)]
pub struct SemanticHasher {
    /// Current hash being computed
    current_hash: Option<u64>,
}

impl SemanticHasher {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate a semantic hash for any AST node that might have comments
    pub fn hash_node(node: &impl VisitWith<Self>) -> u64 {
        let mut hasher = Self::new();
        node.visit_with(&mut hasher);
        hasher.current_hash.unwrap_or(0)
    }

    /// Generate hash for a module item
    pub fn hash_module_item(item: &ModuleItem) -> Option<(u64, String)> {
        match item {
            ModuleItem::Stmt(stmt) => Self::hash_stmt(stmt),
            ModuleItem::ModuleDecl(decl) => Self::hash_module_decl(decl),
        }
    }

    fn hash_stmt(stmt: &Stmt) -> Option<(u64, String)> {
        match stmt {
            Stmt::Decl(decl) => Self::hash_decl(decl),
            Stmt::Expr(expr_stmt) => {
                let hash = Self::hash_node(&expr_stmt.expr);
                Some((hash, format!("expr_{hash:x}")))
            }
            _ => None,
        }
    }

    fn hash_module_decl(decl: &ModuleDecl) -> Option<(u64, String)> {
        match decl {
            ModuleDecl::Import(import) => {
                let hash = Self::hash_import(import);
                Some((hash, format!("import_{hash:x}")))
            }
            ModuleDecl::ExportDecl(export) => Self::hash_decl(&export.decl),
            ModuleDecl::ExportNamed(export) => {
                let hash = Self::hash_node(export);
                Some((hash, format!("export_named_{hash:x}")))
            }
            ModuleDecl::ExportDefaultDecl(export) => {
                let hash = Self::hash_node(&export.decl);
                Some((hash, format!("export_default_{hash:x}")))
            }
            ModuleDecl::ExportDefaultExpr(export) => {
                let hash = Self::hash_node(&export.expr);
                Some((hash, format!("export_default_expr_{hash:x}")))
            }
            _ => None,
        }
    }

    fn hash_decl(decl: &Decl) -> Option<(u64, String)> {
        match decl {
            Decl::Fn(fn_decl) => {
                let hash = Self::hash_function_decl(fn_decl);
                Some((hash, fn_decl.ident.sym.to_string()))
            }
            Decl::Class(class_decl) => {
                let hash = Self::hash_class_decl(class_decl);
                Some((hash, class_decl.ident.sym.to_string()))
            }
            Decl::Var(var_decl) => {
                let hash = Self::hash_var_decl(var_decl);
                let name = var_decl
                    .decls
                    .first()
                    .and_then(|d| Self::get_pat_name(&d.name))
                    .unwrap_or_else(|| "var".to_string());
                Some((hash, name))
            }
            Decl::TsInterface(interface) => {
                let hash = Self::hash_interface(interface);
                Some((hash, interface.id.sym.to_string()))
            }
            Decl::TsTypeAlias(alias) => {
                let hash = Self::hash_type_alias(alias);
                Some((hash, alias.id.sym.to_string()))
            }
            Decl::TsEnum(ts_enum) => {
                let hash = Self::hash_enum(ts_enum);
                Some((hash, ts_enum.id.sym.to_string()))
            }
            _ => None,
        }
    }

    fn hash_import(import: &ImportDecl) -> u64 {
        let mut hasher = DefaultHasher::new();
        "import".hash(&mut hasher);
        import.src.value.hash(&mut hasher);

        // Sort specifiers for consistent hashing regardless of order
        let mut spec_hashes: Vec<u64> = Vec::new();

        for spec in &import.specifiers {
            let mut spec_hasher = DefaultHasher::new();
            match spec {
                ImportSpecifier::Default(default) => {
                    "default".hash(&mut spec_hasher);
                    default.local.sym.hash(&mut spec_hasher);
                }
                ImportSpecifier::Named(named) => {
                    "named".hash(&mut spec_hasher);
                    named.local.sym.hash(&mut spec_hasher);
                    if let Some(imported) = &named.imported {
                        match imported {
                            ModuleExportName::Ident(ident) => ident.sym.hash(&mut spec_hasher),
                            ModuleExportName::Str(s) => s.value.hash(&mut spec_hasher),
                        }
                    }
                }
                ImportSpecifier::Namespace(ns) => {
                    "namespace".hash(&mut spec_hasher);
                    ns.local.sym.hash(&mut spec_hasher);
                }
            }
            spec_hashes.push(spec_hasher.finish());
        }

        // Sort for consistent ordering
        spec_hashes.sort();
        for spec_hash in spec_hashes {
            spec_hash.hash(&mut hasher);
        }

        hasher.finish()
    }

    fn hash_function_decl(func: &FnDecl) -> u64 {
        let mut hasher = DefaultHasher::new();
        "function".hash(&mut hasher);
        func.ident.sym.hash(&mut hasher);
        Self::hash_function_signature(&func.function, &mut hasher);
        hasher.finish()
    }

    fn hash_class_decl(class: &ClassDecl) -> u64 {
        let mut hasher = DefaultHasher::new();
        "class".hash(&mut hasher);
        class.ident.sym.hash(&mut hasher);

        // Include superclass in hash
        if let Some(super_class) = &class.class.super_class {
            if let Expr::Ident(ident) = super_class.as_ref() {
                ident.sym.hash(&mut hasher);
            }
        }

        hasher.finish()
    }

    fn hash_var_decl(var: &VarDecl) -> u64 {
        let mut hasher = DefaultHasher::new();

        match var.kind {
            VarDeclKind::Const => "const".hash(&mut hasher),
            VarDeclKind::Let => "let".hash(&mut hasher),
            VarDeclKind::Var => "var".hash(&mut hasher),
        }

        // Hash all declarators
        for decl in &var.decls {
            if let Some(name) = Self::get_pat_name(&decl.name) {
                name.hash(&mut hasher);
            }
        }

        hasher.finish()
    }

    fn hash_interface(interface: &TsInterfaceDecl) -> u64 {
        let mut hasher = DefaultHasher::new();
        "interface".hash(&mut hasher);
        interface.id.sym.hash(&mut hasher);

        // Include extends in hash
        for extend in &interface.extends {
            if let Expr::Ident(ident) = extend.expr.as_ref() {
                ident.sym.hash(&mut hasher);
            }
        }

        hasher.finish()
    }

    fn hash_type_alias(alias: &TsTypeAliasDecl) -> u64 {
        let mut hasher = DefaultHasher::new();
        "type".hash(&mut hasher);
        alias.id.sym.hash(&mut hasher);
        hasher.finish()
    }

    fn hash_enum(ts_enum: &TsEnumDecl) -> u64 {
        let mut hasher = DefaultHasher::new();
        "enum".hash(&mut hasher);
        ts_enum.id.sym.hash(&mut hasher);
        hasher.finish()
    }

    fn hash_function_signature(func: &Function, hasher: &mut DefaultHasher) {
        // Include parameter count and types
        func.params.len().hash(hasher);

        for param in &func.params {
            match &param.pat {
                Pat::Ident(ident) => {
                    ident.id.sym.hash(hasher);
                    if let Some(type_ann) = &ident.type_ann {
                        Self::hash_type_annotation(type_ann, hasher);
                    }
                }
                Pat::Array(_) => "array_pattern".hash(hasher),
                Pat::Object(_) => "object_pattern".hash(hasher),
                Pat::Rest(_) => "rest_pattern".hash(hasher),
                _ => "other_pattern".hash(hasher),
            }
        }

        // Include return type
        if let Some(return_type) = &func.return_type {
            Self::hash_type_annotation(return_type, hasher);
        }
    }

    fn hash_type_annotation(type_ann: &TsTypeAnn, hasher: &mut DefaultHasher) {
        // Simplified type hashing - could be expanded
        match type_ann.type_ann.as_ref() {
            TsType::TsKeywordType(keyword) => {
                format!("{:?}", keyword.kind).hash(hasher);
            }
            TsType::TsTypeRef(type_ref) => {
                if let TsEntityName::Ident(ident) = &type_ref.type_name {
                    ident.sym.hash(hasher);
                }
            }
            _ => "complex_type".hash(hasher),
        }
    }

    fn get_pat_name(pat: &Pat) -> Option<String> {
        match pat {
            Pat::Ident(ident) => Some(ident.id.sym.to_string()),
            Pat::Object(obj) => {
                // For object patterns, create a composite name
                let names: Vec<String> = obj
                    .props
                    .iter()
                    .filter_map(|prop| match prop {
                        ObjectPatProp::Assign(assign) => Some(assign.key.sym.to_string()),
                        ObjectPatProp::KeyValue(kv) => Self::get_pat_name(&kv.value),
                        _ => None,
                    })
                    .collect();

                if names.is_empty() {
                    None
                } else {
                    Some(format!("{{{}}}", names.join(",")))
                }
            }
            Pat::Array(arr) => {
                let names: Vec<String> = arr
                    .elems
                    .iter()
                    .filter_map(|elem| elem.as_ref().and_then(Self::get_pat_name))
                    .collect();

                if names.is_empty() {
                    None
                } else {
                    Some(format!("[{}]", names.join(",")))
                }
            }
            _ => None,
        }
    }

    /// Generate hash for class members
    pub fn hash_class_member(member: &ClassMember, class_name: &str) -> Option<(u64, String)> {
        let mut hasher = DefaultHasher::new();
        class_name.hash(&mut hasher);

        match member {
            ClassMember::Constructor(ctor) => {
                "constructor".hash(&mut hasher);
                ctor.params.len().hash(&mut hasher);
                Some((hasher.finish(), "constructor".to_string()))
            }
            ClassMember::Method(method) => {
                "method".hash(&mut hasher);
                method.is_static.hash(&mut hasher);

                let name = match &method.key {
                    PropName::Ident(ident) => ident.sym.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    _ => return None,
                };

                name.hash(&mut hasher);
                Self::hash_function_signature(&method.function, &mut hasher);

                Some((hasher.finish(), name))
            }
            ClassMember::PrivateMethod(method) => {
                "private_method".hash(&mut hasher);
                method.is_static.hash(&mut hasher);
                method.key.id.sym.hash(&mut hasher);
                Self::hash_function_signature(&method.function, &mut hasher);

                Some((hasher.finish(), format!("#{}", method.key.id.sym)))
            }
            ClassMember::ClassProp(prop) => {
                "prop".hash(&mut hasher);
                prop.is_static.hash(&mut hasher);

                let name = match &prop.key {
                    PropName::Ident(ident) => ident.sym.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    _ => return None,
                };

                name.hash(&mut hasher);
                Some((hasher.finish(), name))
            }
            ClassMember::PrivateProp(prop) => {
                "private_prop".hash(&mut hasher);
                prop.is_static.hash(&mut hasher);
                prop.key.id.sym.hash(&mut hasher);

                Some((hasher.finish(), format!("#{}", prop.key.id.sym)))
            }
            _ => None,
        }
    }
}

// Implement Visit trait for completeness (though we mostly use specific functions)
impl Visit for SemanticHasher {
    fn visit_module(&mut self, module: &Module) {
        let mut hasher = DefaultHasher::new();
        "module".hash(&mut hasher);
        module.body.len().hash(&mut hasher);
        self.current_hash = Some(hasher.finish());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;

    fn parse_module(source: &str) -> Module {
        let parser = TypeScriptParser::new();
        parser.parse(source, "test.ts").unwrap()
    }

    #[test]
    fn test_function_hash_stable() {
        let source1 = "function foo(x: number): string { return x.toString(); }";
        let source2 = "function foo(x: number): string { return String(x); }";

        let module1 = parse_module(source1);
        let module2 = parse_module(source2);

        let hash1 = SemanticHasher::hash_module_item(&module1.body[0])
            .unwrap()
            .0;
        let hash2 = SemanticHasher::hash_module_item(&module2.body[0])
            .unwrap()
            .0;

        // Same signature = same hash despite different implementation
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_function_hash_different_signatures() {
        let source1 = "function foo(x: number): string { }";
        let source2 = "function foo(x: string): string { }";

        let module1 = parse_module(source1);
        let module2 = parse_module(source2);

        let hash1 = SemanticHasher::hash_module_item(&module1.body[0])
            .unwrap()
            .0;
        let hash2 = SemanticHasher::hash_module_item(&module2.body[0])
            .unwrap()
            .0;

        // Different signatures = different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_class_hash_with_extends() {
        let source1 = "class Foo extends Bar { }";
        let source2 = "class Foo extends Baz { }";

        let module1 = parse_module(source1);
        let module2 = parse_module(source2);

        let hash1 = SemanticHasher::hash_module_item(&module1.body[0])
            .unwrap()
            .0;
        let hash2 = SemanticHasher::hash_module_item(&module2.body[0])
            .unwrap()
            .0;

        // Different superclass = different hash
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_import_hash() {
        let source1 = "import { foo, bar } from './module';";
        let source2 = "import { bar, foo } from './module';";
        let source3 = "import { foo } from './module';";

        let module1 = parse_module(source1);
        let module2 = parse_module(source2);
        let module3 = parse_module(source3);

        let hash1 = SemanticHasher::hash_module_item(&module1.body[0])
            .unwrap()
            .0;
        let hash2 = SemanticHasher::hash_module_item(&module2.body[0])
            .unwrap()
            .0;
        let hash3 = SemanticHasher::hash_module_item(&module3.body[0])
            .unwrap()
            .0;

        // Order doesn't matter for imports with same specifiers
        assert_eq!(hash1, hash2);
        // Different specifiers = different hash
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_destructuring_pattern_names() {
        let source = "const { foo, bar } = obj;";
        let module = parse_module(source);

        let (_, name) = SemanticHasher::hash_module_item(&module.body[0]).unwrap();
        assert_eq!(name, "{foo,bar}");
    }
}
