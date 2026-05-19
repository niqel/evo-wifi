use crate::borrowed_data::WifiSavedSecretView;

pub trait WifiSecretRenderPresentationContract {
    fn provide(&self, secret: WifiSavedSecretView<'_>);
}
