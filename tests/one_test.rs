use regex::Regex;
fn main() {
}

#[test]

fn test_bash_script() {
    let content = r#"
        #!/bin/bash
        echo "Hello, World!"
    "#;

    std::fs::write("test_script.sh", content).unwrap();

    let output = Command::new("bash")
        .arg("test_script.sh")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let re = Regex::new(r"Hello, World!").unwrap();
    assert!(re.is_match(&stdout), "Unexpected stdout: {}", stdout);
    assert_eq!(stderr, "", "Unexpected stderr: {}", stderr);
}
