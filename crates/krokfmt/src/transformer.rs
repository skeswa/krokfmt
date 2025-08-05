use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

/// Import categorization strategy based on common JavaScript conventions.
///
/// This three-tier system was chosen after analyzing popular codebases and tools.
/// The order (External → Absolute → Relative) creates a natural reading flow from
/// third-party dependencies to project code to local modules.
#[derive(Debug, Clone, PartialEq)]
pub enum ImportCategory {
    External, // From node_modules
    Absolute, // Starting with @ or ~
    Relative, // Starting with ./ or ../
}

#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub category: ImportCategory,
    pub path: String,
    pub import_decl: ImportDecl,
}

#[derive(Default)]
pub struct ImportAnalyzer {
    imports: Vec<ImportInfo>,
}

impl ImportAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(mut self, module: &Module) -> Vec<ImportInfo> {
        module.visit_with(&mut self);
        self.imports
    }

    /// Determine import category based on path prefix conventions.
    ///
    /// The order matters here - we check relative paths first because they're the most
    /// specific pattern. The @ and ~ prefixes for absolute imports follow the convention
    /// established by webpack/TypeScript path mapping. Everything else is assumed to be
    /// a node_modules reference (including scoped packages like @babel/core).
    pub fn categorize_import(path: &str) -> ImportCategory {
        if path.starts_with("./") || path.starts_with("../") {
            ImportCategory::Relative
        } else if path.starts_with('@') || path.starts_with('~') {
            ImportCategory::Absolute
        } else {
            ImportCategory::External
        }
    }
}

impl Visit for ImportAnalyzer {
    fn visit_import_decl(&mut self, import: &ImportDecl) {
        let path = import.src.value.to_string();
        let category = Self::categorize_import(&path);

        self.imports.push(ImportInfo {
            category,
            path,
            import_decl: import.clone(),
        });
    }
}

/// Sort imports following the External → Absolute → Relative hierarchy.
///
/// Within each category, imports are sorted alphabetically by path. This creates
/// predictable, scannable import sections. The stable sort preserves the original
/// order for identical paths, which matters for side-effect imports.
pub fn sort_imports(mut imports: Vec<ImportInfo>) -> Vec<ImportInfo> {
    imports.sort_by(|a, b| {
        // Numeric ordering enforces our category hierarchy. Lower numbers appear first,
        // creating the flow from third-party to local code that developers expect.
        let category_order = |cat: &ImportCategory| match cat {
            ImportCategory::External => 0,
            ImportCategory::Absolute => 1,
            ImportCategory::Relative => 2,
        };

        match category_order(&a.category).cmp(&category_order(&b.category)) {
            std::cmp::Ordering::Equal => a.path.to_lowercase().cmp(&b.path.to_lowercase()),
            other => other,
        }
    });

    imports
}

/// Re-export information for organization.
///
/// Re-exports follow the same categorization and sorting rules as imports,
/// creating a consistent structure throughout the module header.
#[derive(Debug, Clone)]
pub struct ReExportInfo {
    pub category: ImportCategory,
    pub path: String,
    pub export_decl: ModuleDecl,
}

#[derive(Default)]
pub struct ReExportAnalyzer {
    re_exports: Vec<ReExportInfo>,
}

impl ReExportAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(mut self, module: &Module) -> Vec<ReExportInfo> {
        module.visit_with(&mut self);
        self.re_exports
    }

    /// Categorize re-export paths using the same logic as imports
    pub fn categorize_re_export(path: &str) -> ImportCategory {
        ImportAnalyzer::categorize_import(path)
    }
}

impl Visit for ReExportAnalyzer {
    fn visit_module_decl(&mut self, decl: &ModuleDecl) {
        match decl {
            // Handle named re-exports: export { foo } from './module'
            ModuleDecl::ExportNamed(export) if export.src.is_some() => {
                let path = export.src.as_ref().unwrap().value.to_string();
                let category = Self::categorize_re_export(&path);

                self.re_exports.push(ReExportInfo {
                    category,
                    path,
                    export_decl: decl.clone(),
                });
            }
            // Handle namespace re-exports: export * from './module'
            ModuleDecl::ExportAll(export) => {
                let path = export.src.value.to_string();
                let category = Self::categorize_re_export(&path);

                self.re_exports.push(ReExportInfo {
                    category,
                    path,
                    export_decl: decl.clone(),
                });
            }
            _ => {}
        }

        decl.visit_children_with(self);
    }
}

/// Sort re-exports following the same External → Absolute → Relative hierarchy as imports.
pub fn sort_re_exports(mut re_exports: Vec<ReExportInfo>) -> Vec<ReExportInfo> {
    re_exports.sort_by(|a, b| {
        let category_order = |cat: &ImportCategory| match cat {
            ImportCategory::External => 0,
            ImportCategory::Absolute => 1,
            ImportCategory::Relative => 2,
        };

        match category_order(&a.category).cmp(&category_order(&b.category)) {
            std::cmp::Ordering::Equal => a.path.to_lowercase().cmp(&b.path.to_lowercase()),
            other => other,
        }
    });

    re_exports
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TypeScriptParser;

    fn parse_and_analyze(source: &str) -> Vec<ImportInfo> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();
        ImportAnalyzer::new().analyze(&module)
    }

    #[test]
    fn test_categorize_imports() {
        assert_eq!(
            ImportAnalyzer::categorize_import("react"),
            ImportCategory::External
        );
        assert_eq!(
            ImportAnalyzer::categorize_import("@utils/helper"),
            ImportCategory::Absolute
        );
        assert_eq!(
            ImportAnalyzer::categorize_import("~/components/Button"),
            ImportCategory::Absolute
        );
        assert_eq!(
            ImportAnalyzer::categorize_import("./components/Button"),
            ImportCategory::Relative
        );
        assert_eq!(
            ImportAnalyzer::categorize_import("../utils/helper"),
            ImportCategory::Relative
        );
        assert_eq!(
            ImportAnalyzer::categorize_import("lodash/debounce"),
            ImportCategory::External
        );
    }

    #[test]
    fn test_import_analysis() {
        let source = r#"
import React from 'react';
import { debounce } from 'lodash';
import { Button } from '@components/Button';
import { helper } from './utils/helper';
import type { User } from '../types';
"#;

        let imports = parse_and_analyze(source);
        assert_eq!(imports.len(), 5);

        assert_eq!(imports[0].category, ImportCategory::External);
        assert_eq!(imports[0].path, "react");

        assert_eq!(imports[1].category, ImportCategory::External);
        assert_eq!(imports[1].path, "lodash");

        assert_eq!(imports[2].category, ImportCategory::Absolute);
        assert_eq!(imports[2].path, "@components/Button");

        assert_eq!(imports[3].category, ImportCategory::Relative);
        assert_eq!(imports[3].path, "./utils/helper");

        assert_eq!(imports[4].category, ImportCategory::Relative);
        assert_eq!(imports[4].path, "../types");
    }

    #[test]
    fn test_sort_imports_by_category() {
        let source = r#"
import { helper } from './helper';
import React from 'react';
import { Button } from '@ui/Button';
import { api } from '../api';
import axios from 'axios';
"#;

        let imports = parse_and_analyze(source);
        let sorted = sort_imports(imports);

        assert_eq!(sorted.len(), 5);

        // External imports should come first
        assert_eq!(sorted[0].path, "axios");
        assert_eq!(sorted[1].path, "react");

        // Then absolute imports
        assert_eq!(sorted[2].path, "@ui/Button");

        // Finally relative imports
        assert_eq!(sorted[3].path, "../api");
        assert_eq!(sorted[4].path, "./helper");
    }

    #[test]
    fn test_sort_imports_alphabetically_within_category() {
        let source = r#"
import zod from 'zod';
import axios from 'axios';
import react from 'react';
import { z } from '@utils/z';
import { a } from '@utils/a';
import { m } from '@utils/m';
"#;

        let imports = parse_and_analyze(source);
        let sorted = sort_imports(imports);

        // External imports sorted alphabetically
        assert_eq!(sorted[0].path, "axios");
        assert_eq!(sorted[1].path, "react");
        assert_eq!(sorted[2].path, "zod");

        // Absolute imports sorted alphabetically
        assert_eq!(sorted[3].path, "@utils/a");
        assert_eq!(sorted[4].path, "@utils/m");
        assert_eq!(sorted[5].path, "@utils/z");
    }

    fn parse_and_analyze_re_exports(source: &str) -> Vec<ReExportInfo> {
        let parser = TypeScriptParser::new();
        let module = parser.parse(source, "test.ts").unwrap();
        ReExportAnalyzer::new().analyze(&module)
    }

    #[test]
    fn test_re_export_analysis() {
        let source = r#"
export { Fragment } from 'react';
export { Button as MyButton } from '@components/Button';
export * from './utils';
export * as helpers from '../helpers';
export { foo, bar } from 'external-lib';
"#;

        let re_exports = parse_and_analyze_re_exports(source);
        assert_eq!(re_exports.len(), 5);

        assert_eq!(re_exports[0].category, ImportCategory::External);
        assert_eq!(re_exports[0].path, "react");

        assert_eq!(re_exports[1].category, ImportCategory::Absolute);
        assert_eq!(re_exports[1].path, "@components/Button");

        assert_eq!(re_exports[2].category, ImportCategory::Relative);
        assert_eq!(re_exports[2].path, "./utils");

        assert_eq!(re_exports[3].category, ImportCategory::Relative);
        assert_eq!(re_exports[3].path, "../helpers");

        assert_eq!(re_exports[4].category, ImportCategory::External);
        assert_eq!(re_exports[4].path, "external-lib");
    }

    #[test]
    fn test_sort_re_exports() {
        let source = r#"
export { helper } from './helper';
export * from 'react-dom';
export { Button } from '@ui/Button';
export * as api from '../api';
export { axios } from 'axios';
"#;

        let re_exports = parse_and_analyze_re_exports(source);
        let sorted = sort_re_exports(re_exports);

        assert_eq!(sorted.len(), 5);

        // External re-exports should come first
        assert_eq!(sorted[0].path, "axios");
        assert_eq!(sorted[1].path, "react-dom");

        // Then absolute re-exports
        assert_eq!(sorted[2].path, "@ui/Button");

        // Finally relative re-exports
        assert_eq!(sorted[3].path, "../api");
        assert_eq!(sorted[4].path, "./helper");
    }
}
