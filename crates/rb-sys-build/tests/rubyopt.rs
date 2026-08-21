use std::{env, path::PathBuf};

use rb_sys_build::RbConfig;

#[test]
fn current_ignores_rubyopt() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    env::set_var("RUBYLIB", fixture_dir);
    env::set_var("RUBYOPT", "-rrubyopt_stdout");

    let rbconfig = RbConfig::current();

    assert!(rbconfig.get("MAJOR").is_some());
    assert!(rbconfig
        .all_keys()
        .all(|key| !key.contains("RUBYOPT polluted stdout")));
}
