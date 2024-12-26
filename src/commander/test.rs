#![cfg(test)]

pub struct TestCommand {}

impl super::Command for TestCommand {
    type Error = TestCommandError;
    type Arg = TestCommandArg;

    const NAME: &str = "test";

    fn execute(&self, args: Vec<Self::Arg>) -> Result<(), Self::Error> {
        let args_strs = args.iter().map(|a| a.0.clone()).collect::<Vec<String>>();
        tracing::debug!("Executed: {} {}", Self::NAME, args_strs.join(" "));
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Test command error")]
pub struct TestCommandError(());

pub struct TestCommandArg(String);

impl super::Argument for TestCommandArg {
    type Error = TestCommandError;

    fn build_from_str(s: &str) -> Result<Option<Self>, Self::Error> {
        Ok(Some(Self(s.to_string())))
    }
}

#[test]
fn execute_test() {
    let mut commander =
        crate::commander::Commander::default().with_command(Box::new(TestCommand {}));
    commander.set_input("t".to_string());

    assert_eq!(commander.suggestions(), vec!["test".to_string()]);
    let res = commander.execute_current_suggestion();

    assert!(res.is_ok());
}
