use evo_wifi_nu_plugin::EvoWifiPlugin;
use nu_plugin::{MsgPackSerializer, serve_plugin};

fn main() {
    serve_plugin(&EvoWifiPlugin, MsgPackSerializer);
}
