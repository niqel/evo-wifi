# Conversacion sobre evo-wifi y la arquitectura

## Contexto

Esta conversacion comenzo con herramientas de terminal, aplicaciones Rust y la idea de nombrar comandos personales con el prefijo `evo-`, como `evo-mdcat` o `evo-xplr`.

Despues evoluciono hacia la construccion de un proyecto real en Rust:

```text
evo-wifi
```

El objetivo inicial del proyecto fue crear una aplicacion minima para Void Linux que permita manejar conexiones WiFi desde terminal.

## Repositorio

Se creo y trabajo sobre el repositorio:

```text
https://github.com/niqel/evo-wifi.git
```

Directorio local:

```text
/home/niqel504/repos/evo-wifi
```

Se agregaron elementos iniciales:

```text
.gitignore
README.md
documentation/
src/
Cargo.toml
```

## Documentacion funcional

Se definieron historias de usuario para una aplicacion minima de WiFi:

1. Ver redes WiFi disponibles.
2. Seleccionar y conectar a una red.
3. Manejar el caso donde ya existe conexion.
4. Cambiar de red.
5. Pedir password cuando no existe guardado.
6. Usar password guardado.
7. Mostrar password/PSK de la conexion actual.
8. Olvidar password o red guardada.
9. Obtener estado actual de la red.

Se decidio que cada historia de usuario viva en su propio archivo Markdown, porque eso permite evolucionarlas, estimarlas y convertirlas en tareas tecnicas de forma independiente.

## Documentacion tecnica

Se crearon y discutieron varios tipos de documentos y diagramas:

```text
documentation/technical-documentation/use-cases
documentation/technical-documentation/borrowed-data-models
documentation/technical-documentation/component-diagrams
documentation/technical-documentation/sequence-diagrams
documentation/technical-documentation/construction-backlog
```

Se uso Graphviz DOT inicialmente y luego D2 para diagramas mas claros, especialmente los UML y de secuencia.

## Conceptos principales de la arquitectura

La arquitectura se fue formalizando alrededor de estas piezas:

```text
commands
agents
resolvers
contracts
providers
borrowed
```

### Commands

Un command es el punto de entrada ejecutable desde terminal.

No contiene logica del caso de uso.

Su responsabilidad es ejecutar el agente correcto.

Ejemplo:

```rust
commands::WifiConnectionStatusShowCommand::execute()
```

### Agents

Un agent representa el sujeto agente de un caso de uso.

No es una entidad de dominio.
No es una herramienta.
No es un provider.

Coordina el pipeline completo de resolvers.

Ejemplo:

```rust
agents::wifi_connection_status_shower::show(...)
```

### Resolvers

Un resolver no es parser, no es renderer y no es solo validador.

Un resolver recibe un dato, puede validar, puede operar o puede decidir si el flujo puede continuar.

Todos los resolvers exponen una unica funcion:

```rust
resolve
```

Si resuelve, el flujo continua.
Si no resuelve, el flujo se detiene o toma otro camino.

Ejemplo:

```text
wifi_interface_resolver::resolve
wifi_connection_status_resolver::resolve
wifi_connection_status_output_resolver::resolve
```

### Contracts

Los contracts son traits.

Representan contratos hacia extremos externos.

Se dividieron en:

```text
contracts/inputs
contracts/outputs
contracts/wifi
```

Todos los contratos exponen una unica funcion:

```rust
provide
```

La regla semantica quedo asi:

```text
providers provide
resolvers resolve
agents show/connect/etc.
commands execute
```

### Providers

Un provider implementa un contract.

Es externo al caso de uso, pero interno al adaptador concreto.

Puede ser:

```text
fake provider
void provider
terminal provider
web provider
database provider
```

Los providers pueden usar `std`, `alloc`, comandos del sistema, terminal, web, bases de datos o cualquier tecnologia externa, porque pertenecen al borde del sistema.

### Borrowed

Los borrowed son datos prestados.

No se llamaron `View` para evitar confusion con:

```text
MVC / MVP / MVVM
database views
presentation views
```

Tampoco se uso `borrowed_data`, sino simplemente:

```text
borrowed
```

Ejemplos:

```rust
pub struct WifiInterfaceBorrowed<'a> {
    pub name: &'a str,
}

pub struct WifiConnectionStatusBorrowed<'a> {
    pub ssid: &'a str,
    pub status: &'a str,
}
```

## Regla de no paquetes

Una de las ideas centrales fue evitar paquetes artificiales.

No se quiso crear una cadena como:

```text
external data
-> DTO
-> entity
-> response
-> view model
-> output
```

En su lugar, la arquitectura busca:

```text
external owner
-> provider
-> borrowed
-> resolver
-> agent
-> output
```

La idea principal:

> Si el extremo externo ya posee, reserva, transforma o administra memoria, el nucleo no debe repetir ese trabajo.

## Ownership y lifetime

Se discutio que el sistema operativo, la terminal, una web o una base de datos ya administran memoria.

El core no debe apropiarse de esos datos si solo necesita verlos para resolver.

Por eso los providers prestan datos usando callbacks:

```rust
pub trait WifiInterfaceContract {
    fn provide<R>(
        &self,
        next: impl FnOnce(WifiInterfaceBorrowed<'_>) -> R,
    ) -> Option<R>;
}
```

El provider posee o crea el dato.
El provider presta el dato dentro del callback.
El resolver usa ese prestamo.
El agente coordina el pipeline.

El prestamo muere al salir del callback.

En el flujo actual:

```text
FakeWifiInterfaceProvider::provide
  presta WifiInterfaceBorrowed

  FakeWifiStatusProvider::provide
    presta WifiConnectionStatusBorrowed

    TerminalStatusOutputProvider::provide
      imprime en stdout

    termina callback status
    muere WifiConnectionStatusBorrowed

  termina callback interface
  muere WifiInterfaceBorrowed
```

Lo que queda visible en la terminal ya no es el prestamo.
Es texto materializado por el extremo externo.

## Flujo implementado

Se implemento una primera rebanada funcional:

```text
main
  -> command
  -> agent
  -> resolver interface
  -> provider fake interface
  -> resolver status
  -> provider fake status
  -> resolver output
  -> provider terminal output
```

La ejecucion confirmo:

```text
evo-wifi    connected
```

## Codigo implementado

Se agregaron y conectaron piezas como:

```text
src/commands/
src/agents/
src/resolvers/
src/contracts/
src/providers/
src/borrowed/
```

Archivos relevantes:

```text
src/commands/wifi_connection_status_show_command.rs
src/agents/wifi_connection_status_shower.rs
src/resolvers/wifi_interface_resolver.rs
src/resolvers/wifi_connection_status_resolver.rs
src/resolvers/wifi_connection_status_output_resolver.rs
src/providers/wifi/fake/interface.rs
src/providers/wifi/fake/status.rs
src/providers/outputs/terminal/status.rs
```

## Fake providers

Se crearon providers fake para demostrar el flujo sin depender todavia del sistema real:

```text
FakeWifiInterfaceProvider
FakeWifiStatusProvider
```

Esto permitio probar:

```text
contracts
resolvers
agent
command
output terminal
```

sin usar todavia `wpa_cli`, `iw`, `ip`, `sv` o servicios reales de Void.

## Resultado tecnico

Se valido con:

```text
cargo fmt --check
cargo test
cargo run
```

La salida confirmada fue:

```text
evo-wifi    connected
```

Tambien se hizo commit y push:

```text
6bfc2aa add command driven wifi status flow
```

## Comparacion con Clean Architecture, Onion y DDD

Se concluyo que esas arquitecturas protegen cosas importantes:

```text
reglas estables
limites entre infraestructura y caso de uso
independencia de frameworks
flujo claro
manejo controlado de fallos
evitar contaminacion del nucleo
```

Pero en una implementacion tipica suelen aparecer mas paquetes internos:

```text
DTOs
entities
mappers
services
repositories
responses
view models
factories
```

En esta arquitectura se protegen los mismos puntos usando:

```text
traits/contracts
borrowed data
providers aislados
resolvers
agents
commands
ownership
lifetimes
inmutabilidad
```

La diferencia principal es que aqui no se protege el sistema con mas objetos, sino con ownership, contratos pequenos y prestamos.

## Rendimiento y memoria

Hasta esta rebanada:

```text
no crates externos
no DTOs
no entidades mutables
no mappers
no Vec en el core
no String en los borrowed
no Box<dyn Trait>
no Arc/Mutex
no RefCell
```

Eso puede beneficiar:

```text
tamanho del binario
consumo de memoria
tiempo de arranque
menos allocations
menos copias
menos procesamiento innecesario
```

El core podria acercarse a `no_std`, porque usa principalmente:

```rust
&str
Option
FnOnce
struct
trait
impl
```

Pero los extremos externos, como terminal, sistema operativo, comandos de Void o web, si pueden necesitar `std` o `alloc`.

La regla quedo clara:

> El core no materializa por costumbre; solo los extremos materializan cuando su medio lo exige.

## Base de datos

Se comparo la idea con `DataReader` de .NET.

La arquitectura encaja bien con bases de datos si el provider actua como cursor/reader:

```text
database engine
  -> driver/cursor/row
  -> database provider
  -> Borrowed<'a>
  -> resolver
  -> agent
  -> output provider
```

La base de datos ya puede proteger desde fuera:

```text
usuario solo lectura
vista SQL
permisos por tabla/columna
stored procedure/query controlada
transaccion read-only
```

El provider no tiene que recrear todo como entidades si solo necesita prestar campos.

Si el output es REST, ahi puede materializar JSON.
Si el output es web, ahi puede asignar etiquetas o propiedades.
Si el output es terminal, ahi puede imprimir.

La materializacion pertenece al extremo externo, no al nucleo.

## Evaluacion de la arquitectura

La evaluacion sincera despues de probarla en codigo fue que la arquitectura ya dejo de ser una preferencia personal y empezo a ser una arquitectura real.

Para sistemas como:

```text
CLI
Rust
sistemas minimos
bajo consumo
integracion con sistema operativo
no paquetes
datos prestados
providers intercambiables
```

esta arquitectura compite muy fuerte contra arquitecturas tradicionales.

Se identificaron pendientes para fortalecerla:

```text
probar mas casos de uso
formalizar manejo de errores
documentar reglas de composicion
implementar providers reales de Void
validar con flujos de conexion, password, olvidar red y escaneo
```

## Conclusion

La arquitectura cumplio su objetivo en esta primera rebanada.

Demostro que se puede construir funcionalidad real con:

```text
no paquetes innecesarios
no mutabilidad artificial
no conversiones de tipo sin valor
no entidades de dominio mutables
no dependencias externas
datos prestados
providers externos
resolvers que deciden
agents que coordinan pipelines
commands como entrada ejecutable
outputs que materializan solo en el borde
```

La esencia quedo formulada asi:

> El externo posee.
> El provider presta.
> El resolver decide.
> El agent coordina.
> El output entrega o materializa.

