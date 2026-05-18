# US-008: Olvidar contrasena WiFi guardada

## Historia de Usuario

Como usuario de Void Linux,
quiero olvidar una contrasena WiFi guardada,
para que el sistema deje de almacenarla para conexiones futuras.

## Criterios de Aceptacion

- Dado que una red WiFi tiene una contrasena guardada, cuando elijo olvidar esa contrasena, entonces la aplicacion elimina la contrasena guardada.
- Dado que la contrasena guardada fue eliminada, cuando intento conectarme nuevamente a esa red, entonces la aplicacion solicita la contrasena si la red esta protegida.
- Dado que no existe una contrasena guardada para la red seleccionada, cuando elijo olvidarla, entonces la aplicacion informa que no hay una contrasena guardada para eliminar.
