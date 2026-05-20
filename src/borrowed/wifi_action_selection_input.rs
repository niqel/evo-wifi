#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiActionSelectionInputBorrowed<'a> {
    pub raw: &'a str,
}
