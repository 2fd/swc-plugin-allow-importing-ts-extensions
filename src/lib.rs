#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub mod plugin;

#[plugin_transform]
fn transform_imports_plugin(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<plugin::Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for transform-imports"),
    )
    .expect("invalid packages");

    program.visit_mut_with(&mut plugin::init(config));
    program
}