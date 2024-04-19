use assert_cmd::Command;

#[test]
fn test_palindrome_positive() {
    let mut cmd = Command::cargo_bin("palindrome").unwrap();
    cmd.arg("check-palindrome").arg("--text").arg("level");
    cmd.assert().success().stdout("'level' is a palindrome!\n");
}

#[test]
fn test_palindrome_negative() {
    let mut cmd = Command::cargo_bin("palindrome").unwrap();
    cmd.arg("check-palindrome").arg("--text").arg("hello");
    cmd.assert()
        .success()
        .stdout("'hello' is not a palindrome.\n");
}
