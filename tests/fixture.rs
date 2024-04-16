use std::path::PathBuf;

use allow_importing_ts_extensions::plugin::init;
use swc_common::{chain, Mark};
use swc_ecma_parser::{Syntax, TsConfig};
// use swc_ecma_transforms_base::resolver::resolver;
use swc_ecma_transforms_typescript::{typescript, Config};
use swc_ecma_transforms_testing::test_fixture;
use swc_ecma_visit::as_folder;

#[testing::fixture("tests/fixture/**/input.ts")]
fn allow_importing_ts_extensions_fixture(input: PathBuf) {
    // let mark = Mark::fresh(Mark::root());
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Syntax::Typescript(TsConfig::default()),
        &|_tr| chain!(
            typescript(Config::default(), Mark::fresh(Mark::root())),
            as_folder(init())
        ),
        &input,
        &output,
        Default::default(),
    );
}