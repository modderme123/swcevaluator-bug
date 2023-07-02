use std::path::PathBuf;

use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{base::resolver, testing::test_fixture},
        visit::as_folder,
    },
};
use swcevaluator::TransformVisitor;
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/code.js")]
fn jsx_dom_expressions_fixture_babel(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        syntax(),
        &|t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                as_folder(TransformVisitor::new(t.comments.clone()))
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
