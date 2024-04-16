use serde::Deserialize;
use swc_ecma_ast as ast;
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub preserve_import_extension: bool,
    // pub packages: HashMap<String, PackageConfig>,
}

fn replace_ts_extension(src: &ast::Str, config: &Config) -> Option<ast::Str> {
    if src.value.ends_with(".ts") && !src.value.ends_with(".d.ts") {
        if let Some(file) = src.value.strip_suffix(".ts") {
            return Some(format!("{}.js", file).into());
        }
    } else if src.value.ends_with(".mts") && !src.value.ends_with(".d.mts") {
        if let Some(file) = src.value.strip_suffix(".mts") {
            if config.preserve_import_extension {
                return Some(format!("{}.mjs", file).into());
            } else {
                return Some(format!("{}.js", file).into());
            }
        }
    } else if src.value.ends_with(".cts") && !src.value.ends_with(".d.cts") {
        if let Some(file) = src.value.strip_suffix(".cts") {
            if config.preserve_import_extension {
                return Some(format!("{}.cjs", file).into());
            } else {
                return Some(format!("{}.js", file).into());
            }
        }
    }

    None
}

struct AllowImportingTSExtensions {
    config: Config
}

impl VisitMut for AllowImportingTSExtensions {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ast::ImportDecl) {
        n.visit_mut_children_with(self);

        if let Some(replaced) = replace_ts_extension(&n.src, &self.config) {
            n.src = Box::new(replaced);
        }
    }

    fn visit_mut_export_all(&mut self, n: &mut ast::ExportAll) {
        n.visit_mut_children_with(self);

        if let Some(replaced) = replace_ts_extension(&n.src, &self.config) {
            n.src = Box::new(replaced);
        }
    }

    fn visit_mut_named_export(&mut self, n: &mut ast::NamedExport) {
        n.visit_mut_children_with(self);

        if let Some(src) = &n.src {
            if let Some(replaced) = replace_ts_extension(src, &self.config) {
                n.src = Some(Box::new(replaced));
            }
        }
    }

    fn visit_mut_call_expr(&mut self, n: &mut ast::CallExpr) {
        n.visit_mut_children_with(self);

        if n.callee.is_import() && !n.args.is_empty() {
            if let Some(value) = n.args.first_mut() {
                if let ast::Expr::Lit(ast::Lit::Str(src)) = value.expr.as_ref() {
                    if let Some(replaced) = replace_ts_extension(src, &self.config) {
                        value.expr = Box::new(ast::Expr::Lit(ast::Lit::Str(replaced)));
                    }
                }
            }
        }
    }
}

pub fn init(config: Config) -> impl VisitMut {
    AllowImportingTSExtensions { config }
}
