use crate::borrowed_data::WifiSavedSecretView;

pub trait WifiSecretRenderPresentationContract {
    fn render_wifi_secret(&self, secret: WifiSavedSecretView<'_>);
}
