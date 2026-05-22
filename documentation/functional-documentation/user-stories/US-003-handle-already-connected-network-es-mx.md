# US-003: Manejar red ya conectada

## Historia de Usuario

Como usuario Linux con wpa_supplicant,
quiero que la aplicacion detecte cuando selecciono la red a la que ya estoy conectado,
para evitar reconectarme innecesariamente.

## Criterios de Aceptacion

- Dado que ya estoy conectado a una red WiFi, cuando selecciono la misma red, entonces la aplicacion detecta la conexion existente.
- Dado que la red seleccionada ya esta conectada, cuando intento conectarme, entonces la aplicacion muestra un mensaje indicando que ya tengo conexion a esa red.
- Dado que la red seleccionada ya esta conectada, cuando se muestra el mensaje, entonces no se realiza un intento de reconexion.
