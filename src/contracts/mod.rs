pub mod inputs;
pub mod outputs;
pub mod wifi;

pub use inputs::{
    WifiActionSelectionInputContract, WifiNetworkSelectionInputContract, WifiPasswordInputContract,
};
pub use outputs::{
    WifiMessageOutputContract, WifiNetworkOutputContract, WifiSecretOutputContract,
    WifiStatusOutputContract,
};
pub use wifi::{
    WifiConnectContract, WifiForgetContract, WifiInterfaceContract, WifiNewNetworkConnectContract,
    WifiSavedNetworkContract, WifiSavedSecretContract, WifiScanContract, WifiStatusContract,
};
