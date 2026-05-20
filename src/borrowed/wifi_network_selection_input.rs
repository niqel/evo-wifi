#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiNetworkSelectionInputBorrowed<'a> {
    pub raw: &'a str,
}
