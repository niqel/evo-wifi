# US-009: Ver estado actual de la red

## Historia de Usuario

Como usuario Linux con wpa_supplicant,
quiero ver el estado actual de la red,
para saber si el sistema esta desconectado o conectado a una red WiFi.

## Criterios de Aceptacion

- Dado que el sistema no esta conectado a una red WiFi, cuando solicito el estado actual de la red, entonces la aplicacion informa que no hay una conexion WiFi activa.
- Dado que el sistema esta conectado a una red WiFi, cuando solicito el estado actual de la red, entonces la aplicacion muestra el SSID de la red conectada.
- Dado que el sistema esta conectado a una red WiFi, cuando solicito el estado actual de la red, entonces la aplicacion muestra el estado de la conexion.
- Dado que el estado actual no puede resolverse, cuando solicito el estado actual de la red, entonces la aplicacion informa que no se pudo determinar el estado.
