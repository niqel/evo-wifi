# US-006: Usar contrasena guardada cuando exista

## Historia de Usuario

Como usuario Linux con wpa_supplicant,
quiero que la aplicacion use una contrasena guardada cuando exista para la red seleccionada,
para poder conectarme sin volver a escribirla.

## Criterios de Aceptacion

- Dado que selecciono una red WiFi protegida con contrasena guardada, cuando la aplicacion prepara la conexion, entonces resuelve la contrasena guardada.
- Dado que se resuelve una contrasena guardada, cuando confirmo la conexion, entonces la aplicacion intenta conectarse sin pedirme la contrasena otra vez.
- Dado que la contrasena guardada ya no es valida, cuando falla la conexion, entonces la aplicacion reporta la falla.
