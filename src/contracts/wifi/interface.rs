use crate::borrowed::WifiInterfaceBorrowed;

pub trait WifiInterfaceContract {
    fn provide(&self) -> Option<WifiInterfaceBorrowed<'_>>;
}
