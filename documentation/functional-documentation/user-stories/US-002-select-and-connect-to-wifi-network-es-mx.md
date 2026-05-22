# US-002: Seleccionar y conectarse a una red WiFi

## Historia de Usuario

Como usuario Linux con wpa_supplicant,
quiero seleccionar una red WiFi disponible y conectarme a ella,
para poder acceder a internet mediante la red seleccionada.

## Criterios de Aceptacion

- Dado que se muestran redes WiFi disponibles, cuando selecciono una red, entonces la aplicacion prepara el flujo de conexion para esa red.
- Dado que la red seleccionada puede resolverse para conexion, cuando confirmo la conexion, entonces la aplicacion intenta conectarse a ella.
- Dado que la conexion es exitosa, cuando termina el proceso, entonces el sistema queda conectado a la red WiFi seleccionada.
- Dado que la conexion falla, cuando termina el proceso, entonces la aplicacion reporta la falla.
