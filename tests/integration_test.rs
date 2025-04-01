use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::tempdir;

#[test]
fn test_simple_output() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let template_path = dir.path().join("template.j2");

    fs::write(&template_path, "Hello {{NAME}}!")?;

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(template_path)
        .env("NAME", "World")
        .output()?;

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?, "Hello World!\n");

    Ok(())
}

#[test]
fn test_filters() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let template_path = dir.path().join("template.j2");

    fs::write(
        &template_path,
        "{% for item in ITEMS | split(',') -%}
-{{ item }}
{% endfor %}",
    )?;

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(template_path)
        .env("ITEMS", "a,b,c")
        .output()?;

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?, "-a\n-b\n-c\n\n");

    Ok(())
}

#[test]
fn test_stdin_input() -> anyhow::Result<()> {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--")
        .env("NAME", "World")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"Hello {{NAME}}!")?;

    let output = child.wait_with_output()?;
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout)?, "Hello World!\n");

    Ok(())
}
