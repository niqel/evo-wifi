pub trait WifiMessageOutputContract {
    fn provide(&self, message: &str);
}
