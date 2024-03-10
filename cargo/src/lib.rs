#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_lint;
extern crate rustc_session;

#[allow(clippy::no_mangle_with_rust_abi)]
#[no_mangle]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    package_dependencies::register_lints(sess, lint_store);
}
