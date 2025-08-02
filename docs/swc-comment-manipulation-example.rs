// Example: Working with SWC Comments in Practice

use swc_common::{
    comments::{Comment, CommentKind, Comments, SingleThreadedComments},
    BytePos, Span, DUMMY_SP,
};
use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

/// Example 1: Reading all comments from a module
fn print_all_comments(module: &Module, comments: &SingleThreadedComments) {
    println!("=== Analyzing Comments ===");
    
    // Walk through each item in the module
    for item in &module.body {
        let span = item.span();
        
        // Check for leading comments
        if let Some(leading) = comments.get_leading(span.lo) {
            println!("Leading comments at BytePos({}):", span.lo.0);
            for comment in leading {
                println!("  {} comment: '{}'", 
                    match comment.kind {
                        CommentKind::Line => "Line",
                        CommentKind::Block => "Block",
                    },
                    comment.text
                );
            }
        }
        
        // Check for trailing comments
        if let Some(trailing) = comments.get_trailing(span.hi) {
            println!("Trailing comments at BytePos({}):", span.hi.0);
            for comment in trailing {
                println!("  {} comment: '{}'", 
                    match comment.kind {
                        CommentKind::Line => "Line",
                        CommentKind::Block => "Block",
                    },
                    comment.text
                );
            }
        }
    }
}

/// Example 2: Tracking comment associations during transformation
struct CommentTracker {
    comments: SingleThreadedComments,
    // Map from old position to new position
    position_map: std::collections::HashMap<BytePos, BytePos>,
}

impl CommentTracker {
    /// Collect all comments before transformation
    fn collect_comments(&self, span: Span) -> (Vec<Comment>, Vec<Comment>) {
        let leading = self.comments.get_leading(span.lo)
            .map(|c| c.to_vec())
            .unwrap_or_default();
        let trailing = self.comments.get_trailing(span.hi)
            .map(|c| c.to_vec())
            .unwrap_or_default();
        (leading, trailing)
    }
    
    /// Reapply comments at new positions
    fn apply_comments(&self, old_span: Span, new_span: Span, new_comments: &SingleThreadedComments) {
        // Get comments from old position
        let (leading, trailing) = self.collect_comments(old_span);
        
        // Add to new position
        for comment in leading {
            new_comments.add_leading(new_span.lo, comment);
        }
        for comment in trailing {
            new_comments.add_trailing(new_span.hi, comment);
        }
    }
}

/// Example 3: Comment-aware visitor that preserves comments during sorting
struct SortingTransformer {
    comments: SingleThreadedComments,
}

impl VisitMut for SortingTransformer {
    fn visit_mut_module(&mut self, module: &mut Module) {
        // Example: Sort variable declarations alphabetically
        // while preserving their comments
        
        // First, collect all variable declarations with their comments
        let mut var_decls: Vec<(VarDecl, Vec<Comment>, Vec<Comment>)> = vec![];
        
        for item in &module.body {
            if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = item {
                let leading = self.comments.get_leading(var_decl.span.lo)
                    .map(|c| c.to_vec())
                    .unwrap_or_default();
                let trailing = self.comments.get_trailing(var_decl.span.hi)
                    .map(|c| c.to_vec())
                    .unwrap_or_default();
                    
                var_decls.push((var_decl.clone(), leading, trailing));
            }
        }
        
        // Sort by variable name
        var_decls.sort_by(|a, b| {
            let name_a = get_var_name(&a.0);
            let name_b = get_var_name(&b.0);
            name_a.cmp(&name_b)
        });
        
        // NOTE: In a real implementation, you would need to:
        // 1. Calculate new BytePos positions for the sorted declarations
        // 2. Create new spans with these positions
        // 3. Add comments at the new positions
        // This is where the complexity lies!
        
        module.visit_mut_children_with(self);
    }
}

fn get_var_name(var_decl: &VarDecl) -> String {
    var_decl.decls.first()
        .and_then(|d| match &d.name {
            Pat::Ident(ident) => Some(ident.id.sym.to_string()),
            _ => None,
        })
        .unwrap_or_default()
}

/// Example 4: Detecting "floating" comments (not attached to any node)
fn find_floating_comments(
    source_code: &str,
    module: &Module,
    comments: &SingleThreadedComments,
) -> Vec<(BytePos, Comment)> {
    let mut floating = vec![];
    let mut covered_positions = std::collections::HashSet::new();
    
    // Mark all positions that are covered by AST nodes
    struct PositionCollector {
        positions: std::collections::HashSet<BytePos>,
    }
    
    impl swc_ecma_visit::Visit for PositionCollector {
        fn visit_span(&mut self, span: &Span) {
            self.positions.insert(span.lo);
            self.positions.insert(span.hi);
        }
    }
    
    let mut collector = PositionCollector {
        positions: covered_positions,
    };
    use swc_ecma_visit::Visit;
    module.visit_with(&mut collector);
    
    // Check all possible positions for comments
    // (In practice, you'd need to iterate through the comment storage)
    // This is pseudo-code as SingleThreadedComments doesn't expose iteration
    /*
    for (pos, comments) in all_comments {
        if !collector.positions.contains(&pos) {
            for comment in comments {
                floating.push((pos, comment));
            }
        }
    }
    */
    
    floating
}

/// Example 5: Adding comments to generated code
fn add_generated_marker(module: &mut Module, comments: &SingleThreadedComments) {
    // Add a comment at the top of the file
    if let Some(first_item) = module.body.first() {
        comments.add_leading(
            first_item.span().lo,
            Comment {
                kind: CommentKind::Line,
                span: DUMMY_SP,
                text: " @generated - This file was auto-generated".into(),
            }
        );
    }
    
    // Add comments to each generated function
    for item in &mut module.body {
        if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) = item {
            comments.add_leading(
                fn_decl.function.span.lo,
                Comment {
                    kind: CommentKind::Block,
                    span: DUMMY_SP,
                    text: " Generated function - do not edit ".into(),
                }
            );
        }
    }
}

/// Example 6: The core problem - why moving nodes doesn't move comments
fn demonstrate_reordering_problem() {
    // Original AST and comments:
    // Node A at BytePos(10-20), comment at leading[10]
    // Node B at BytePos(30-40), comment at leading[30]
    
    // After swapping nodes in AST:
    // Node B still has span(30-40) - span is immutable!
    // Node A still has span(10-20) - span is immutable!
    
    // So comments remain at:
    // leading[10] - originally for A, but now B is rendered first
    // leading[30] - originally for B, but now A is rendered second
    
    // Result: Comments appear with wrong nodes!
    
    println!("The fundamental issue:");
    println!("1. Node spans are immutable - they record original source positions");
    println!("2. Comments are stored by position, not by node reference");
    println!("3. When nodes are reordered, their spans don't change");
    println!("4. Therefore, comments stay at original positions");
}

/// Example 7: A theoretical solution (not implementable with current SWC)
mod theoretical_solution {
    use super::*;
    
    // What we would need:
    struct NodeWithComments {
        node: ModuleItem,
        leading: Vec<Comment>,
        trailing: Vec<Comment>,
    }
    
    // Then we could:
    // 1. Extract nodes with their comments
    // 2. Sort/reorder the NodeWithComments
    // 3. Recalculate positions
    // 4. Rebuild AST with new spans
    // 5. Rebuild comment map with new positions
    
    // But SWC doesn't support:
    // - Mutable spans
    // - Iterating all comments
    // - Creating spans with arbitrary positions
}

fn main() {
    println!("See examples above for various comment manipulation patterns");
    demonstrate_reordering_problem();
}