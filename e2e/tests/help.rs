#[cfg(test)]
mod help_tests {
    use std::process::Stdio;

    use assertor::*;

    fn setup() {
        e2e::build()
    }

    #[test]
    fn show_help() {
        setup();
        let (mut cmd, dir) = e2e::run_app();

        let child = cmd
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute app");

        let actual = child.wait_with_output().expect("failed to wait on child");

        assert_eq!(actual.status.code().unwrap(), 2);
        let stderr = String::from_utf8_lossy(&actual.stderr);

        assert_that!(stderr.to_string()).contains("-h, --help    Print help information")
    }
}
