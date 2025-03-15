use std::process::Child;

use moose_utils::result::Result;

use super::start_test_validator;

pub struct TestValidator {
    child: Child,
}

impl TestValidator {
    pub async fn new() -> Result<Self> {
        let child = start_test_validator().await?;
        Ok(Self { child })
    }
}

impl Drop for TestValidator {
    fn drop(&mut self) {
        // Ignore errors during cleanup
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
