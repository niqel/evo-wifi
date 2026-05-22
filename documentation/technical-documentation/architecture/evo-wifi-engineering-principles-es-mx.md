# Principios de Ingenieria

## Proposito

Este documento define principios de diseno reutilizables para todos los proyectos.

No describe una tecnologia o framework especifico. Describe una forma de pensar, nombrar y estructurar software.

## Principio Base

Todo proyecto debe construirse sobre principios SOLID.

De todos ellos, el Principio de Responsabilidad Unica es indispensable:

- cada modulo debe tener una responsabilidad dominante
- cada funcion debe expresar una accion clara
- cada tipo debe representar una entidad de dominio comprensible
- si una pieza intenta hacer demasiadas cosas, debe dividirse

## Responsabilidad Unica

Una pieza de software tiene responsabilidad unica cuando su razon principal de cambio es solo una.

Esto implica:

- un modulo no debe mezclar logica de dominio con UI, persistencia, networking o coordinacion externa si esa no es su funcion principal
- una funcion no debe esconder varias transformaciones independientes
- un tipo no debe existir como contenedor generico de comportamientos no relacionados

La responsabilidad unica no es solo una regla de arquitectura. Tambien es una regla de lenguaje.

Si el nombre de una pieza no deja claro lo que hace, probablemente su responsabilidad no esta bien definida.

## Sujeto Agente

El concepto de sujeto agente se adopta como criterio de diseno.

En linguistica, un nombre de agente puede derivarse de un verbo para nombrar al sujeto que realiza una accion.

Ejemplos:

- to blend -> blender
- to run -> runner
- to dry -> dryer
- to profess -> professor
- to build -> builder
- to print -> printer
- to generate -> generator

En software, este concepto se usa para nombrar modulos, tipos, comandos y componentes como sujetos conceptuales definidos por la accion dominante que realizan.

Regla:

- si una pieza recibe un nombre de sujeto agente, debe tener una accion primaria clara y defendible

Ejemplos:

- `Generator` genera
- `Printer` imprime
- `Heatsink` disipa
- `VolumeController` controla volumen

Si una pieza no puede justificar su nombre mediante una accion dominante, su diseno debe revisarse.

La idea de sujeto agente no obliga a crear un `struct`, una clase o un objeto con estado.

Puede expresarse como modulo, tipo, comando, script o componente, dependiendo de la frontera real que el sistema necesita.

## Relacion Entre Objeto, Accion y Sujeto Agente

La siguiente relacion semantica se considera util:

```text
objeto + accion -> sujeto agente
```

Ejemplos conceptuales:

- clothes + wash -> washer
- audio + route -> audio router
- stream + move -> stream mover
- sink + select -> sink selector

Esta relacion ayuda a disenar componentes cuyo nombre revela:

- que recibe
- que transforma
- cual es su funcion principal

En este documento, el objeto en esa relacion se entiende primero como una pieza semantica del dominio, no necesariamente como una entidad poseida por el sistema.

En algunos proyectos, ese objeto sera su propia entidad. En otros, puede ser un recurso prestado, una vista o una capacidad minima definida por un contract o specification.

## Inspiracion Linguistica

Parte de la inspiracion de este enfoque viene del uso de compuestos funcionales en persa, donde la relacion entre objeto y accion puede formar una identidad semantica clara para el sujeto agente.

En este documento, esa idea se adopta como referencia conceptual para mejorar la claridad en nombres, modulos y responsabilidades en software, no como una regla linguistica estricta.

## Principio de Transparencia Semantica

Los nombres del sistema deben acercarse al dominio real y no solo a la mecanica tecnica.

Preferir:

- nombres que expresen funcion
- modulos pequenos y semanticamente cerrados
- verbos claros para operaciones
- tipos que representen entidades de dominio

Evitar:

- nombres genericos como `Manager`, `Helper`, `Utils`, `Thing`, `DataProcessor`
- modulos que solo agrupan codigo sin identidad
- estructuras creadas por costumbre cuando un modulo seria suficiente

## Modulo Como Sujeto Agente

No toda responsabilidad necesita representarse como un `struct`.

Cuando no hay estado propio, una responsabilidad puede vivir correctamente en un modulo.

El modulo puede actuar como el sujeto agente semantico del dominio.

Ejemplo:

```rust
mod washer {
    pub struct Garment {
        pub clean: bool,
        pub material: String,
    }

    pub fn wash(garment: &mut Garment) {
        garment.clean = true;
    }
}
```

Lectura semantica:

- `Garment` es la entidad que recibe la transformacion
- `wash` es la accion
- `washer` es el contexto funcional que da identidad al acto

La existencia del modulo ya establece una frontera de responsabilidad.

De acuerdo con la arquitectura del proyecto, ese objeto puede representarse como su propia entidad o como un recurso prestado bajo una forma minima suficiente para la operacion.

Un `struct Washer` no debe crearse si no hay estado, coordinacion o comportamiento persistente que lo justifique.

## Regla de Diseno

Antes de crear una pieza nueva, responde estas preguntas:

1. Cual es la accion dominante?
2. Que entidad recibe la accion?
3. El sujeto agente necesita estado o solo un contexto semantico?
4. El nombre refleja claramente su responsabilidad?
5. La pieza tiene una sola razon principal para cambiar?

Si estas preguntas no tienen respuestas claras, el diseno todavia no esta listo.

## Convenciones Recomendadas

- usar nombres con peso semantico real
- preferir modulos pequenos sobre contenedores genericos
- crear tipos solo cuando agreguen identidad real o invariantes
- usar funciones con verbos precisos
- evitar abstracciones vacias
- revisar si cada nombre puede explicarse desde el verbo que le da origen

## Criterio de Revision

Una pieza de codigo debe revisarse si ocurre cualquiera de estas cosas:

- su nombre no explica lo que hace
- hace mas de una cosa principal
- mezcla responsabilidades de niveles distintos
- usa un `struct` donde un modulo seria suficiente
- usa un modulo donde realmente se necesita estado explicito
- el dominio no puede reconstruirse leyendo nombres y firmas

## Aplicacion General

Estos principios aplican a:

- scripts
- crates
- CLIs
- herramientas internas
- automatizacion
- utilidades de sistema

## Sintesis

La arquitectura no solo se organiza con tipos y dependencias. Tambien se organiza con gramatica.

Un buen diseno puede leerse como una relacion clara entre:

- entidad
- accion
- sujeto agente

Cuando el nombre, la responsabilidad y el comportamiento coinciden, el sistema gana claridad, mantenibilidad y coherencia.
