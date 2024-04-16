use std::path::PathBuf;

use allow_importing_ts_extensions::plugin;
use swc_common::{chain, Mark};
use swc_ecma_parser::{Syntax, TsConfig};
// use swc_ecma_transforms_base::resolver::resolver;
use swc_ecma_transforms_typescript::{typescript, Config};
use swc_ecma_transforms_testing::test_fixture;
use swc_ecma_visit::as_folder;

#[testing::fixture("tests/fixture/simple/input.ts")]
#[testing::fixture("tests/fixture/no-preserve-extensions/input.ts")]
fn allow_importing_ts_extensions_default_fixture(input: PathBuf) {
    // let mark = Mark::fresh(Mark::root());
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Syntax::Typescript(TsConfig::default()),
        &|_tr| chain!(
            typescript(Config::default(), Mark::fresh(Mark::root())),
            as_folder(plugin::init(plugin::Config::default()))
        ),
        &input,
        &output,
        Default::default(),
    );
}


#[testing::fixture("tests/fixture/preserve-extensions/input.ts")]
fn allow_importing_ts_extensions_preserved_fixture(input: PathBuf) {
    // let mark = Mark::fresh(Mark::root());
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Syntax::Typescript(TsConfig::default()),
        &|_tr| chain!(
            typescript(Config::default(), Mark::fresh(Mark::root())),
            as_folder(plugin::init(plugin::Config { preserve_import_extension: true}))
        ),
        &input,
        &output,
        Default::default(),
    );
}