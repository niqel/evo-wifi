# Resumen de Arquitectura de evo-wifi

## Proposito

Este documento resume la arquitectura que evo-wifi demuestra en la practica.

El proyecto existe para probar una arquitectura borrowed-first construida sobre responsabilidades claras y ownership minimo.

## Idea Principal

El proyecto se organiza alrededor de estos conceptos:

- `agents`
- `resolvers`
- `contracts`
- `providers`

Y alrededor de esta forma de dominio:

- `borrowed`

El repositorio ahora es un workspace de Cargo:

- `crates/evo-wifi-core` es la libreria reutilizable. Contiene `borrowed`, `contracts`, `resolvers`, `agents` y `commands`.
- `crates/evo-wifi-cli` es el runtime binario. Lee argumentos CLI, crea providers concretos y llama commands del core.
- `crates/evo-wifi-provider-linux-wpa` implementa providers de sistema Linux WPA.
- `crates/evo-wifi-provider-nushell` implementa providers orientados a Nushell para entradas tipadas y salidas estructuradas.
- `crates/evo-wifi-provider-terminal` implementa providers de entrada y salida de terminal.

## Forma en Tiempo de Ejecucion

En tiempo de ejecucion, el proyecto trabaja asi:

```text
evo-wifi-cli main.rs -> providers + evo-wifi-core commands -> agents -> resolvers -> contracts -> providers -> mundo externo
```

Esa es la ruta real de implementacion usada por la CLI.

### Que Hace Cada Pieza

- `evo-wifi-cli/main.rs` selecciona el comando desde los argumentos de CLI.
- `evo-wifi-cli/main.rs` ensambla los providers concretos que necesita cada comando.
- `commands` actua como objetos de comando independientes del runtime dentro de `evo-wifi-core`.
- `agents` coordina el caso de uso encadenando resolvers.
- `resolvers` decide si los datos prestados pueden volverse operables.
- `contracts` define el comportamiento esperado por rol: `inputs`, `outputs` y `actions`.
- los crates de providers implementan una sola responsabilidad de contrato mediante `provide`.
- el mundo externo es entrada/salida de terminal, valores Nushell, WiFi y el sistema operativo.

## Regla Semantica de Operacion

La arquitectura usa una regla estricta de nombre y operacion:

```text
provider.provide
resolver.resolve
sujeto_agente.accion_principal
command.execute
```

Los providers proveen exactamente una responsabilidad de contract.
Los resolvers resuelven si el flujo puede continuar.
Los sujetos agente ejecutan la accion nombrada por su modulo.
Los commands ejecutan el punto de entrada seleccionado por CLI.

Los contracts no se especializan por tecnologia o caso de uso concreto. La especialidad vive en el provider:

- `contracts/inputs` declara contratos de entrada de informacion o seleccion.
- `contracts/outputs` declara contratos de salida.
- `contracts/actions` declara contratos que ejecutan una accion sobre el mundo externo.
- los crates de providers contienen implementaciones concretas como terminal, Nushell o Linux WPA.

En evo-wifi, WiFi es la especialidad del workspace y de sus crates de providers concretos, no una carpeta de contracts separada.
Nushell es una especialidad de presentacion/runtime. Es dueno de sus strings recibidos y del estado de salida estructurado, y despues presta valores borrowed al core mediante `provide`.

Ejemplos:

```text
wifi_interface_resolver.resolve
LinuxWpaWifiInterfaceProvider.provide
wifi_connection_status_shower.show
wifi_network_switcher.switch
```

Esto sigue la regla de sujeto agente:

```text
licuadora.licua
lavadora.lava
network_switcher.switch
connection_status_shower.show
```

Si un nombre no puede defender su accion principal, la pieza debe renombrarse o dividirse.

## Flujo de Datos Borrowed-First

La entrada y la salida se tratan primero como recursos externos.

El sistema prefiere trabajar con formas prestadas como:

- `WifiInterfaceBorrowed`
- `WifiConnectionStatusBorrowed`
- `WifiNetworkBorrowed`
- `WifiSavedNetworkBorrowed`
- `WifiPasswordInputBorrowed`

Esto mantiene el ownership donde pertenece naturalmente y evita wrappers innecesarios o payloads duplicados.

## Modelo de Dominio

El dominio es intencionalmente ligero.

Se representa con:

- `borrowed/` para vistas de datos sin ownership

Esto mantiene el modelo cerca del flujo real de datos y evita materializacion artificial.

## Significado de la Arquitectura

Este proyecto no esta probando solo un diagrama de componentes.

El diagrama de componentes muestra modulos y dependencias.
El resumen de arquitectura muestra como el sistema esta organizado semantica y operacionalmente.

Las reglas arquitectonicas clave son:

- el agent coordina
- el resolver decide
- el contract declara el comportamiento esperado
- el provider provee una sola responsabilidad de contract mediante `provide`
- las formas borrowed se prefieren cuando son suficientes
- los contracts se ordenan por rol operacional, no por tecnologia
- los providers dan la especialidad concreta

## Resultado Practico

Esta arquitectura permite que evo-wifi:

- mantenga la terminal como interfaz funcional
- preserve responsabilidades claras
- evite paquetes intermedios innecesarios
- pruebe el dominio sin providers reales
- reemplace crates de providers sin reescribir el flujo central

## Que Debe Mostrar el Diagrama de Arquitectura

El diagrama de arquitectura del proyecto debe hacer visible:

- punto de entrada CLI
- objetos de comando
- coordinacion de agents
- cadena de resolvers
- frontera de contracts
- crates de providers externos con responsabilidades unicas
- formas borrowed del dominio
- mundo externo

Esa es la arquitectura que el proyecto intenta probar.
