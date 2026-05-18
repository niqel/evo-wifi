# US-005: Solicitar contrasena cuando sea necesario

## Historia de Usuario

Como usuario de Void Linux,
quiero que la aplicacion solicite la contrasena cuando la red seleccionada no tenga una contrasena guardada,
para poder conectarme a redes WiFi protegidas.

## Criterios de Aceptacion

- Dado que selecciono una red WiFi protegida sin contrasena guardada, cuando la aplicacion prepara la conexion, entonces me solicita la contrasena de la red.
- Dado que ingreso una contrasena, cuando confirmo la conexion, entonces la aplicacion usa esa contrasena para intentar conectarse.
- Dado que cancelo la solicitud de contrasena, cuando termina el proceso, entonces la aplicacion no intenta conectarse.
