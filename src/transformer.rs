use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

#[derive(Debug, Clone, PartialEq)]
pub enum ImportCategory {
    External,   // From node_modules
    Absolute,   // Starting with @ or ~
    Relative,   // Starting with ./ or ../
}

#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub category: ImportCategory,
    pub path: String,
    pub import_decl: ImportDecl,
}

pub struct ImportAnalyzer {
    imports: Vec<ImportInfo>,
}

impl ImportAnalyzer {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }

    pub fn analyze(mut self, module: &Module) -> Vec<ImportInfo> {
        module.visit_with(&mut self);
        self.imports
    }

    fn categorize_import(path: &str) -> ImportCategory {
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

pub fn sort_imports(mut imports: Vec<ImportInfo>) -> Vec<ImportInfo> {
    // Sort by category first (External < Absolute < Relative)
    // Then alphabetically by path within each category
    imports.sort_by(|a, b| {
        let category_order = |cat: &ImportCategory| match cat {
            ImportCategory::External => 0,
            ImportCategory::Absolute => 1,
            ImportCategory::Relative => 2,
        };
        
        match category_order(&a.category).cmp(&category_order(&b.category)) {
            std::cmp::Ordering::Equal => a.path.cmp(&b.path),
            other => other,
        }
    });
    
    imports
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
}