# US-007: Mostrar contrasena de la conexion actual

## Historia de Usuario

Como usuario de Void Linux,
quiero ver la contrasena guardada de la conexion WiFi actual,
para poder consultarla cuando la necesite.

## Criterios de Aceptacion

- Dado que estoy conectado a una red WiFi con contrasena guardada, cuando solicito la contrasena de la conexion actual, entonces la aplicacion muestra la contrasena guardada.
- Dado que no estoy conectado a una red WiFi, cuando solicito la contrasena de la conexion actual, entonces la aplicacion informa que no hay una conexion WiFi actual.
- Dado que la conexion actual no tiene contrasena guardada, cuando solicito la contrasena, entonces la aplicacion informa que no existe una contrasena guardada.
