use crate::borrowed::WifiActionSelectionInputBorrowed;

pub trait WifiActionSelectionInputContract {
    fn provide(&self) -> Option<WifiActionSelectionInputBorrowed<'_>>;
}
