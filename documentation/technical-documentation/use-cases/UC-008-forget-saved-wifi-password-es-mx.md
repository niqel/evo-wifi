# UC-008: Olvidar Contraseña WiFi Guardada

## Historia de Usuario Relacionada

US-008: Olvidar contraseña WiFi guardada

## Proposito

Olvidar la contraseña guardada de una red WiFi seleccionada para que el sistema ya no conserve esa credencial para conexiones futuras.

## Actor Principal

Usuario de Void Linux

## Disparador

El usuario solicita olvidar la contraseña guardada de una red WiFi especifica.

## Precondiciones

- La aplicacion puede acceder al provider de entrada.
- La aplicacion puede acceder al provider WiFi del sistema.
- El provider WiFi del sistema puede proveer una interfaz WiFi utilizable mediante el contrato de interfaz.
- La red WiFi seleccionada tiene una entrada guardada cuando el usuario espera que la eliminacion tenga exito.

## Flujo Principal

1. El actor solicita olvidar la contraseña guardada de una red seleccionada.
2. El sujeto agente inicia el flujo de olvido.
3. El resolver de interfaz WiFi resuelve una interfaz WiFi utilizable.
4. El resolver de red guardada resuelve la red guardada para el SSID seleccionado.
5. El resolver de olvido pide al provider WiFi que elimine la red guardada.
6. El resolver de salida envia un mensaje de exito mediante el contrato de presentacion.
7. El provider de presentacion confirma que la contraseña WiFi guardada fue eliminada.

## Flujos Alternativos

### No Se Puede Resolver Una Interfaz WiFi

1. El resolver de interfaz WiFi no puede resolver una interfaz WiFi.
2. El resolver de salida envia un mensaje mediante el contrato de presentacion.
3. El provider de presentacion informa que no se encontro una interfaz WiFi utilizable.

### No Se Puede Resolver La Red Guardada

1. La interfaz WiFi fue resuelta.
2. El resolver de red guardada no puede resolver una red guardada para el SSID seleccionado.
3. El resolver de salida envia un mensaje mediante el contrato de presentacion.
4. El provider de presentacion informa que no existe una contraseña WiFi guardada para eliminar.

### La Red Guardada No Se Puede Eliminar

1. La interfaz WiFi fue resuelta.
2. La red guardada fue resuelta.
3. El resolver de olvido no puede eliminar la red guardada.
4. El resolver de salida envia un mensaje mediante el contrato de presentacion.
5. El provider de presentacion informa que la contraseña WiFi guardada no se pudo eliminar.

## Datos Prestados

- `WifiInterfaceBorrowed<'a>`
  - `name: &'a str`
- `WifiNetworkSelectionInputBorrowed<'a>`
  - `raw: &'a str`
- `WifiSavedNetworkBorrowed<'a>`
  - `ssid: &'a str`
  - `network_id: &'a str`

## Mapeo Tecnico

### Sujeto Agente

- `agents::wifi_saved_network_forgetter::forget`

### Pipeline de Resolvers

```text
agents::wifi_saved_network_forgetter::forget
  -> wifi_network_selection_input_resolver::resolve
  -> wifi_interface_resolver::resolve
  -> wifi_saved_network_resolver::resolve
  -> wifi_forget_resolver::resolve
```

### Resolvers

- `wifi_network_selection_input_resolver::resolve`
- `wifi_interface_resolver::resolve`
- `wifi_saved_network_resolver::resolve`
- `wifi_forget_resolver::resolve`

### Contratos WiFi del Sistema

- `WifiNetworkSelectionInputContract`
- `WifiInterfaceContract`
- `WifiSavedNetworkContract`
- `WifiForgetContract`

### Provider WiFi del Sistema

- `VoidWifiProvider`

### Contrato de Presentacion

- `WifiMessageOutputContract`

### Provider de Presentacion

- `TerminalMessageOutputProvider`

### Regla de Funcion en Contratos

- Todo contrato expone una unica operacion llamada `provide`.
- Los providers proveen.
- Los resolvers resuelven.
- Los sujetos agente coordinan su pipeline de resolvers.

## Candidatos a Tareas de Desarrollo

- Definir `WifiNetworkSelectionInputBorrowed<'a>`.
- Definir `WifiInterfaceBorrowed<'a>`.
- Definir `WifiSavedNetworkBorrowed<'a>`.
- Definir `WifiNetworkSelectionInputContract`.
- Definir `WifiInterfaceContract`.
- Definir `WifiSavedNetworkContract`.
- Definir `WifiForgetContract`.
- Implementar el comportamiento de olvido en `VoidWifiProvider`.
- Implementar los mensajes de olvido en `TerminalMessageOutputProvider`.
- Implementar `wifi_network_selection_input_resolver::resolve`.
- Implementar `wifi_interface_resolver::resolve`.
- Implementar `wifi_saved_network_resolver::resolve`.
- Implementar `wifi_forget_resolver::resolve`.
- Implementar `agents::wifi_saved_network_forgetter::forget`.
- Agregar pruebas para rutas de resuelto, sin red guardada, interfaz no resuelta y eliminacion no resuelta.

## Notas de Aceptacion

- El sujeto agente no debe llamar providers directamente.
- El sujeto agente debe trabajar mediante el pipeline de resolvers.
- La salida de presentacion debe pasar por el contrato de presentacion.
- El olvido de una red guardada debe ser un flujo independiente.
- La red WiFi seleccionada se pasa como parametro de entrada.
