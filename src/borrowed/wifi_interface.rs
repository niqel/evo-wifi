#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WifiInterfaceBorrowed<'a> {
    pub name: &'a str,
}
