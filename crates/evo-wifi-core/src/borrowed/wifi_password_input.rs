#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiPasswordInputBorrowed<'a> {
    pub raw: &'a str,
}
