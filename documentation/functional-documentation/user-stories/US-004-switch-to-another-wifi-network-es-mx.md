# US-004: Cambiar a otra red WiFi

## Historia de Usuario

Como usuario Linux con wpa_supplicant,
quiero conectarme a otra red WiFi disponible,
para poder cambiar mi conexion actual.

## Criterios de Aceptacion

- Dado que estoy conectado a una red WiFi, cuando selecciono otra red disponible, entonces la aplicacion prepara un cambio de red.
- Dado que la red seleccionada puede resolverse para conexion, cuando confirmo el cambio, entonces la aplicacion intenta conectarse a la red seleccionada.
- Dado que el cambio es exitoso, cuando termina el proceso, entonces el sistema queda conectado a la nueva red WiFi seleccionada.
- Dado que el cambio falla, cuando termina el proceso, entonces la aplicacion reporta la falla.
