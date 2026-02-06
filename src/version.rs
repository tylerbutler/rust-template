//! Version string construction with rich git info for development builds.
//!
//! In CI builds, displays a clean version like `0.1.0`.
//! In local/dev builds, displays a rich version like `0.1.0-main (abc1234) (dirty)`.

use std::sync::LazyLock;

/// Build version string with git info for local builds.
static VERSION: LazyLock<String> = LazyLock::new(|| {
    let version = env!("CARGO_PKG_VERSION");
    let is_ci = option_env!("RUST_TEMPLATE_CI_BUILD") == Some("true");

    // CI builds just show the version
    if is_ci {
        return version.to_string();
    }

    // Local builds show: {version}-{branch} ({sha}) or {version}-{branch} ({sha}) (dirty)
    let sha = option_env!("VERGEN_GIT_SHA").map(|s| &s[..7.min(s.len())]);
    let branch = option_env!("VERGEN_GIT_BRANCH");
    let dirty = option_env!("VERGEN_GIT_DIRTY") == Some("true");

    match (sha, branch, dirty) {
        (Some(sha), Some(branch), true) => format!("{version}-{branch} ({sha}) (dirty)"),
        (Some(sha), Some(branch), false) => format!("{version}-{branch} ({sha})"),
        (Some(sha), None, true) => format!("{version} ({sha}) (dirty)"),
        (Some(sha), None, false) => format!("{version} ({sha})"),
        (None, _, _) => version.to_string(),
    }
});

/// Returns the version string for use in CLI output.
pub fn version_string() -> &'static str {
    &VERSION
}
