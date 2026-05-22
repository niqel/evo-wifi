use crate::borrowed::{WifiInterfaceBorrowed, WifiNetworkBorrowed};
use crate::contracts::WifiScanContract;

pub fn resolve<R>(
    provider: &impl WifiScanContract,
    interface: WifiInterfaceBorrowed<'_>,
    next: impl FnOnce(&[WifiNetworkBorrowed<'_>]) -> R,
) -> Option<R> {
    provider.provide(interface, next)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ResolvedScanProvider;

    impl WifiScanContract for ResolvedScanProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            next: impl FnOnce(&[WifiNetworkBorrowed<'_>]) -> R,
        ) -> Option<R> {
            let networks = [WifiNetworkBorrowed {
                bssid: "00:11:22:33:44:55",
                frequency_mhz: 5200,
                signal_dbm: -51,
                flags: "[WPA2-PSK-CCMP][ESS]",
                ssid: "evo-wifi",
            }];

            Some(next(&networks))
        }
    }

    struct UnresolvedScanProvider;

    impl WifiScanContract for UnresolvedScanProvider {
        fn provide<R>(
            &self,
            _interface: WifiInterfaceBorrowed<'_>,
            _next: impl FnOnce(&[WifiNetworkBorrowed<'_>]) -> R,
        ) -> Option<R> {
            None
        }
    }

    #[test]
    fn resolves_scan_from_provider() {
        let provider = ResolvedScanProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };

        let result = resolve(&provider, interface, |networks| networks[0].ssid.to_owned());

        assert_eq!(result.as_deref(), Some("evo-wifi"));
    }

    #[test]
    fn returns_none_when_provider_cannot_provide_scan() {
        let provider = UnresolvedScanProvider;
        let interface = WifiInterfaceBorrowed { name: "wlp2s0" };

        let result = resolve(&provider, interface, |_networks| "should not run");

        assert_eq!(result, None);
    }
}
