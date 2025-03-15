use std::path::PathBuf;

use moose_utils::paths::resolve_path;

pub fn get_deps_configs(file: &str) -> PathBuf {
    resolve_path("deps/configs").join(file)
}
