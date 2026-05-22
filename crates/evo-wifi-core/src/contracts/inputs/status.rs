use crate::borrowed::{WifiConnectionStatusBorrowed, WifiInterfaceBorrowed};

pub trait WifiStatusContract {
    fn provide<R>(
        &self,
        interface: WifiInterfaceBorrowed<'_>,
        next: impl FnOnce(WifiConnectionStatusBorrowed<'_>) -> R,
    ) -> Option<R>;
}
