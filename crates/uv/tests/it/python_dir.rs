use assert_fs::fixture::{FileWriteStr, PathChild};

use uv_static::EnvVars;

use crate::common::{TestContext, uv_snapshot};

#[test]
fn python_dir() {
    let context = TestContext::new("3.12");

    let python_dir = context.temp_dir.child("python");
    uv_snapshot!(context.filters(), context.python_dir()
    .env(EnvVars::UV_PYTHON_INSTALL_DIR, python_dir.as_os_str()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/python

    ----- stderr -----
    ");
}
#[test]
fn python_dir_config_precedence() {
    let context = TestContext::new("3.12");

    let python_dir_config = context.temp_dir.child("config-python");
    let python_dir_env = context.temp_dir.child("env-python");

    let toml = format!(
        r#"
        [tool.uv]
        python-install-dir = "{}"
        "#,
        python_dir_config.to_str().unwrap().escape_debug()
    );
    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(&toml)
        .unwrap();

    uv_snapshot!(context.filters(), context.python_dir()
    .env(EnvVars::UV_PYTHON_INSTALL_DIR, python_dir_env.as_os_str()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/env-python

    ----- stderr -----
    ");
}

#[test]
fn python_dir_expansion() {
    let context = TestContext::new("3.12");

    let toml = indoc::indoc! { r#"
        [tool.uv]
        python-install-dir = "${PROJECT_ROOT}/.python"
        "#
    };
    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(toml)
        .unwrap();

    uv_snapshot!(context.filters(), context.python_dir().env_remove(EnvVars::UV_PYTHON_INSTALL_DIR), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/.python

    ----- stderr -----
    ");
}
#[test]
fn python_dir_cli_precedence() {
    let context = TestContext::new("3.12");

    let python_dir_cli = context.temp_dir.child("cli-python");
    let python_dir_env = context.temp_dir.child("env-python");

    uv_snapshot!(context.filters(), context.python_dir()
        .env(EnvVars::UV_PYTHON_INSTALL_DIR, python_dir_env.as_os_str())
        .arg("--python-install-dir")
        .arg(python_dir_cli.as_os_str()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/cli-python

    ----- stderr -----
    ");
}

#[test]
fn python_dir_cli_over_config() {
    let context = TestContext::new("3.12");

    let python_dir_cli = context.temp_dir.child("cli-python");
    let python_dir_config = context.temp_dir.child("config-python");

    let toml = format!(
        r#"
        [tool.uv]
        python-install-dir = "{}"
        "#,
        python_dir_config.to_str().unwrap().escape_debug()
    );
    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(&toml)
        .unwrap();

    // Ensure Env is removed to strictly test CLI > Config
    uv_snapshot!(context.filters(), context.python_dir()
        .env_remove(EnvVars::UV_PYTHON_INSTALL_DIR)
        .arg("--python-install-dir")
        .arg(python_dir_cli.as_os_str()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/cli-python

    ----- stderr -----
    ");
}

#[test]
fn python_dir_cli_over_all() {
    let context = TestContext::new("3.12");

    let python_dir_cli = context.temp_dir.child("cli-python");
    let python_dir_env = context.temp_dir.child("env-python");
    let python_dir_config = context.temp_dir.child("config-python");

    let toml = format!(
        r#"
        [tool.uv]
        python-install-dir = "{}"
        "#,
        python_dir_config.to_str().unwrap().escape_debug()
    );
    context
        .temp_dir
        .child("pyproject.toml")
        .write_str(&toml)
        .unwrap();

    uv_snapshot!(context.filters(), context.python_dir()
        .env(EnvVars::UV_PYTHON_INSTALL_DIR, python_dir_env.as_os_str())
        .arg("--python-install-dir")
        .arg(python_dir_cli.as_os_str()), @"
    success: true
    exit_code: 0
    ----- stdout -----
    [TEMP_DIR]/cli-python

    ----- stderr -----
    ");
}
