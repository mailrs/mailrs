#![cfg(test)]

pub struct TestCommand {}

impl super::Command for TestCommand {
    type Error = TestCommandError;

    const NAME: &str = "test";

    fn execute(&self, args: Vec<String>) -> Result<(), Self::Error> {
        tracing::debug!("Executed: {} {}", Self::NAME, args.join(" "));
        Ok(())
    }

    fn arg_suggestions() -> Vec<String> {
        Vec::new()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Test command error")]
pub struct TestCommandError(());

#[test]
fn execute_test() {
    let mut commander =
        crate::commander::Commander::default().with_command(TestCommand {});
    commander.set_input(vec!["t".to_string()]);

    assert_eq!(commander.suggestions(), vec!["test".to_string()]);
    let res = commander.execute_current_input();

    assert!(res.is_ok());
}
