# UC-004: Cambiar a Otra Red WiFi

## Historia de Usuario Relacionada

US-004: Cambiar a otra red WiFi

## Proposito

Cambiar desde la red WiFi actual a otra red seleccionada coordinando los flujos existentes de estado, red guardada, contrasena y conexion.

## Actor Principal

Usuario Linux con wpa_supplicant

## Disparador

El usuario solicita cambiar a otra red WiFi.

## Precondiciones

- La aplicacion puede acceder al provider de entrada.
- La aplicacion puede acceder al provider WiFi del sistema.
- El provider WiFi del sistema puede proveer una interfaz WiFi utilizable mediante el contrato de interfaz.

## Flujo Principal

1. El actor solicita cambiar a otra red WiFi.
2. El sujeto agente inicia el flujo de cambio.
3. El resolver de interfaz WiFi resuelve una interfaz WiFi utilizable.
4. El resolver de estado de conexion WiFi resuelve el estado actual de conexion para esa interfaz.
5. El resolver de cambio determina si la red seleccionada ya esta conectada.
6. Si la red seleccionada ya esta conectada, el resolver de salida envia un mensaje mediante el contrato de presentacion.
7. Si la red seleccionada no esta conectada y existe una red guardada, el resolver de red guardada la resuelve y el resolver de conexion intenta el cambio.
8. Si la red seleccionada no esta conectada y no existe una red guardada, el resolver de contrasena resuelve una contrasena y el resolver de conexion nueva intenta el cambio.
9. El provider de presentacion proporciona el estado o mensaje resultante.

## Flujos Alternativos

### No Se Puede Resolver Una Interfaz WiFi

1. El resolver de interfaz WiFi no puede resolver una interfaz WiFi.
2. El resolver de salida envia un mensaje mediante el contrato de presentacion.
3. El provider de presentacion informa que no se encontro una interfaz WiFi utilizable.

### La Red Seleccionada Ya Esta Conectada

1. La interfaz WiFi fue resuelta.
2. El resolver de estado de conexion WiFi resuelve un estado cuyo SSID coincide con la red seleccionada.
3. El resolver de cambio enruta al camino de mensaje de ya conectado.
4. El provider de presentacion informa que la red seleccionada ya esta conectada.

### La Red Seleccionada Tiene Una Entrada Guardada

1. La interfaz WiFi fue resuelta.
2. La red seleccionada no esta conectada.
3. El resolver de red guardada resuelve una entrada guardada.
4. El resolver de conexion intenta el cambio usando la entrada guardada.
5. El provider de presentacion proporciona el estado de conexion resultante.

### La Red Seleccionada No Tiene Entrada Guardada

1. La interfaz WiFi fue resuelta.
2. La red seleccionada no esta conectada.
3. El resolver de red guardada no puede resolver una entrada guardada.
4. El resolver de contrasena resuelve una contrasena desde la entrada.
5. El resolver de conexion nueva intenta el cambio usando la contrasena suministrada.
6. El provider de presentacion proporciona el estado de conexion resultante.

### No Se Proporciona Contrasena Para Una Conexion Nueva

1. La interfaz WiFi fue resuelta.
2. La red seleccionada no esta conectada.
3. El resolver de red guardada no puede resolver una entrada guardada.
4. El resolver de contrasena no puede resolver la entrada de contrasena.
5. El resolver de salida envia un mensaje mediante el contrato de presentacion.
6. El provider de presentacion informa que se requiere una contrasena para conectarse a la red seleccionada.

## Datos Prestados

- `WifiInterfaceBorrowed<'a>`
  - `name: &'a str`
- `WifiConnectionStatusBorrowed<'a>`
  - `ssid: &'a str`
  - `state: WifiConnectionState`
- `WifiNetworkSelectionInputBorrowed<'a>`
  - `raw: &'a str`
- `WifiPasswordInputBorrowed<'a>`
  - `raw: &'a str`
- `WifiSavedNetworkBorrowed<'a>`
  - `ssid: &'a str`
  - `network_id: u32`

## Mapeo Tecnico

### Sujeto Agente

- `agents::wifi_network_switcher::switch`

### Cadena de Resolvers

```text
agents::wifi_network_switcher::switch
  -> wifi_network_selection_input_resolver::resolve
  -> wifi_interface_resolver::resolve
  -> wifi_connection_status_resolver::resolve
  -> wifi_network_switch_resolver::resolve
```

### Resolvers

- `wifi_network_selection_input_resolver::resolve`
- `wifi_interface_resolver::resolve`
- `wifi_connection_status_resolver::resolve`
- `wifi_network_switch_resolver::resolve`
- `wifi_already_connected_network_resolver::resolve`
- `wifi_saved_network_resolver::resolve`
- `wifi_connect_resolver::resolve`
- `wifi_password_input_resolver::resolve`
- `wifi_new_network_connect_resolver::resolve`
- `wifi_connection_status_output_resolver::resolve`
- `wifi_message_output_resolver::resolve`

### Contratos WiFi del Sistema

- `WifiNetworkSelectionInputContract`
- `WifiInterfaceContract`
- `WifiStatusContract`
- `WifiSavedNetworkContract`
- `WifiConnectContract`
- `WifiPasswordInputContract`
- `WifiNewNetworkConnectContract`

### Providers WiFi del Sistema

- `LinuxWpaWifiInterfaceProvider`
- `LinuxWpaWifiStatusProvider`
- `LinuxWpaWifiSavedNetworkProvider`
- `LinuxWpaWifiConnectProvider`
- `LinuxWpaWifiNewNetworkConnectProvider`

### Contratos de Presentacion

- `WifiStatusOutputContract`
- `WifiMessageOutputContract`

### Providers de Presentacion

- `TerminalStatusOutputProvider`
- `TerminalMessageOutputProvider`

### Regla de Funcion en Contratos

- Todo contrato expone una unica operacion llamada `provide`.
- Los providers proveen.
- Los resolvers resuelven.
- Los sujetos agente coordinan su cadena de resolvers.

## Candidatos a Tareas de Desarrollo

- Definir `WifiNetworkSelectionInputBorrowed<'a>`.
- Definir `WifiInterfaceBorrowed<'a>`.
- Definir `WifiConnectionStatusBorrowed<'a>`.
- Definir `WifiPasswordInputBorrowed<'a>`.
- Definir `WifiSavedNetworkBorrowed<'a>`.
- Definir `WifiNetworkSelectionInputContract`.
- Definir `WifiInterfaceContract`.
- Definir `WifiStatusContract`.
- Definir `WifiSavedNetworkContract`.
- Definir `WifiConnectContract`.
- Definir `WifiPasswordInputContract`.
- Definir `WifiNewNetworkConnectContract`.
- Definir `WifiStatusOutputContract`.
- Definir `WifiMessageOutputContract`.
- Implementar `wifi_network_switch_resolver::resolve`.
- Implementar `agents::wifi_network_switcher::switch`.
- Agregar pruebas para rutas de ya conectado, red guardada, contrasena de respaldo y contrasena faltante.

## Notas de Aceptacion

- El sujeto agente no debe decidir la ruta por si mismo.
- La decision del cambio debe vivir dentro del resolver.
- El sujeto agente debe solo conectar la cadena de resolvers.
- La implementacion debe reutilizar las cadenas de resolvers de conexion existentes y de conexion nueva.
