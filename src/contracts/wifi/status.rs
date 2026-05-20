use crate::borrowed::{WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};

pub trait WifiStatusContract {
    fn provide(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
    ) -> Option<WifiConnectionStatusBorrowed<'_>>;
}
