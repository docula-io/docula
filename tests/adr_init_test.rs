use std::error::Error;
use assert_cmd::Command;

#[test]
fn test_adr_init() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir::TempDir::new("adr_test")?;

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/bar", "--name", "foobar"])
        .assert()
        .success();

    let docula_path = tmp.path().join(".docula");
    let adr_path = tmp.path().join("foo/bar");

    assert!(docula_path.exists());
    assert!(adr_path.exists());

    tmp.close()?;

    Ok(())
}

#[test]
fn test_adr_init_existing_name() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir::TempDir::new("adr_test")?;

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/bar", "--name", "foobar"])
        .assert()
        .success();

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/bar/boo", "--name", "foobar"])
        .assert()
        .failure();

    tmp.close()?;

    Ok(())
}

#[test]
fn test_adr_init_existing_path() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir::TempDir::new("adr_test")?;

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/bar", "--name", "foobar"])
        .assert()
        .success();

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/bar", "--name", "hello"])
        .assert()
        .failure();

    tmp.close()?;

    Ok(())
}

#[test]
fn test_adr_init_bad_index_type() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir::TempDir::new("adr_test")?;

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/bar", "--name", "foobar", "--index-type", "foob"])
        .assert()
        .failure();

    tmp.close()?;

    Ok(())
}


#[test]
fn test_adr_init_sequential_ordering() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir::TempDir::new("adr_test")?;

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/bar", "--name", "foobar", "--index-type", "sequential"])
        .assert()
        .success();

    tmp.close()?;

    Ok(())
}

#[test]
fn test_adr_init_multiple_dirs() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir::TempDir::new("adr_test")?;

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/bye", "--name", "bye"])
        .assert()
        .success();

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "foo/hello", "--name", "hello"])
        .assert()
        .success();

    assert!(tmp.path().join("foo/bye").exists());
    assert!(tmp.path().join("foo/hello").exists());

    tmp.close()?;

    Ok(())
}

#[test]
fn test_creating_parent_dir() -> Result<(), Box<dyn Error>> {
    let tmp = tempdir::TempDir::new("adr_test")?;

    let child = tmp.path().join("foo/bar");
    std::fs::create_dir_all(&child)?;

    let docula_path = tmp.path().join(".docula");
    std::fs::write(docula_path, "")?;

    Command::cargo_bin("docula")?
        .current_dir(child)
        .args(["adr", "init", "../baa", "--name", "bye"])
        .assert()
        .success();

    Command::cargo_bin("docula")?
        .current_dir(tmp.path())
        .args(["adr", "init", "../", "--name", "hello"])
        .assert()
        .failure();

    assert!(tmp.path().join("foo/baa").exists());

    tmp.close()?;

    Ok(())
}
