use crate::borrowed::WifiInterfaceBorrowed;

pub trait WifiInterfaceContract {
    fn provide<R>(&self, next: impl FnOnce(WifiInterfaceBorrowed<'_>) -> R) -> Option<R>;
}
