pub mod dis;

#[cfg(test)]
pub mod test {
    /// Turning on `BLESS` makes test write their expected output to files, instead of failing due to mismatches. After
    /// enabling this and running tests, verify your changed files manually for correctness, and turn it off again.
    ///
    /// While this is turned on, test results may be meaningless, so this flag being off is tested below.
    pub const BLESS: bool = false;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    pub fn bless_is_off() {
        assert!(!BLESS, "BLESS must be off for tests to actually test");
    }
}
