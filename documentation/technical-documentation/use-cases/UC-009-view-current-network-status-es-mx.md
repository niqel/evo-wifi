# UC-009: Ver Estado Actual de la Red

## Historia de Usuario Relacionada

US-009: Ver estado actual de la red

## Proposito

Mostrar si el sistema esta desconectado o conectado a una red WiFi, incluyendo el SSID actual y el estado cuando esten disponibles.

Este caso de uso es el primer candidato de implementacion tecnica porque valida la arquitectura con un flujo pequeno y completo.

## Actor Principal

Usuario de Void Linux

## Disparador

El usuario solicita el estado actual de la red desde la aplicacion.

## Precondiciones

- La aplicacion puede acceder al provider de presentacion.
- La aplicacion puede acceder al provider WiFi del sistema.
- El provider WiFi del sistema puede proveer una interfaz WiFi mediante el contrato de interfaz.

## Flujo Principal

1. El actor solicita el estado actual de la red.
2. El sujeto agente inicia el flujo para mostrar el estado.
3. El resolver de interfaz WiFi resuelve una interfaz WiFi utilizable.
4. El resolver de estado de conexion WiFi resuelve el estado actual de conexion para esa interfaz.
5. El resolver de salida del estado de conexion WiFi envia la vista prestada del estado al contrato de presentacion.
6. El provider de presentacion provee la salida del estado actual de la red.

## Flujos Alternativos

### No Se Puede Resolver Una Interfaz WiFi

1. El resolver de interfaz WiFi no puede resolver una interfaz WiFi.
2. El resolver de salida envia un mensaje de estado mediante el contrato de presentacion.
3. El provider de presentacion provee una salida indicando que no se encontro una interfaz WiFi utilizable.

### No Se Puede Resolver El Estado Actual

1. La interfaz WiFi fue resuelta.
2. El resolver de estado de conexion WiFi no puede resolver el estado actual.
3. El resolver de salida envia un mensaje de estado mediante el contrato de presentacion.
4. El provider de presentacion provee una salida indicando que no se pudo determinar el estado actual de la red.

### No Hay Conexion WiFi Activa

1. La interfaz WiFi fue resuelta.
2. El resolver de estado de conexion WiFi resuelve un estado desconectado.
3. El resolver de salida envia la vista prestada del estado al contrato de presentacion.
4. El provider de presentacion provee una salida indicando que no hay una conexion WiFi activa.

## Datos Prestados

- `WifiInterfaceView<'a>`
  - `name: &'a str`
- `WifiConnectionStatusView<'a>`
  - `ssid: &'a str`
  - `status: &'a str`

## Mapeo Tecnico

### Sujeto Agente

- `agent_subjects::wifi_connection_status_shower::show`

### Pipeline de Resolvers

```text
agent_subjects::wifi_connection_status_shower::show
  -> wifi_interface_resolver::resolve
  -> wifi_connection_status_resolver::resolve
  -> wifi_connection_status_output_resolver::resolve
```

### Resolvers

- `wifi_interface_resolver::resolve`
- `wifi_connection_status_resolver::resolve`
- `wifi_connection_status_output_resolver::resolve`

### Contratos WiFi del Sistema

- `WifiInterfaceContract`
- `WifiStatusContract`

### Provider WiFi del Sistema

- `VoidWifiProvider`

### Contrato de Presentacion

- `WifiStatusOutputContract`

### Provider de Presentacion

- `TerminalOutputProvider`

### Regla de Funcion en Contratos

- Todo contrato expone una unica operacion llamada `provide`.
- Los providers proveen.
- Los resolvers resuelven.
- Los sujetos agente coordinan su pipeline de resolvers.

## Candidatos a Tareas de Desarrollo

- Definir `WifiInterfaceView<'a>`.
- Definir `WifiConnectionStatusView<'a>`.
- Definir `WifiInterfaceContract`.
- Definir `WifiStatusContract`.
- Definir `WifiStatusOutputContract`.
- Implementar el comportamiento de estado en `VoidWifiProvider`.
- Implementar la provision de salida de estado en `TerminalOutputProvider`.
- Implementar `wifi_interface_resolver::resolve`.
- Implementar `wifi_connection_status_resolver::resolve`.
- Implementar `wifi_connection_status_output_resolver::resolve`.
- Implementar `agent_subjects::wifi_connection_status_shower::show`.
- Agregar pruebas para rutas de resuelto, desconectado, interfaz no resuelta y estado no resuelto.

## Notas de Aceptacion

- El sujeto agente no debe llamar providers directamente.
- El sujeto agente debe trabajar mediante el pipeline de resolvers.
- La salida de presentacion debe pasar por el contrato de presentacion.
- El acceso al WiFi del sistema debe pasar por los contratos WiFi del sistema.
- Los valores de estado permanecen como strings prestados y no se convierten a enums en esta etapa.
