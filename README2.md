## Diagrama de Clases UML
[<- Volvere](README.md)

## Justificacion 
- Se modelan Presa y Depredador como entidades independientes, implementando un trait Organismo común para unificar comportamiento (actualizar, dibujar, estado de vida).
- Se usan enums para Especie, Sexo y EstadoSalud, lo que mejora la legibilidad y permite agregar nuevas especies o estados sin cambiar la lógica central.
- La separación de atributos y métodos mantiene cohesión alta, con responsabilidades claras: movimiento, salud, reproducción, alimentación.

```mermaid
classDiagram
direction TB

%% Clases principales en los extremos
class Presa {
    -x: f32
    -y: f32
    -vx: f32
    -vy: f32
    -r: f32
    -viva: bool
    -especie: Especie
    -sexo: Sexo
    -cooldown: f32
    -edad: u32
    -peso: f32
    -modo_reproduccion: bool
    -salud: EstadoSalud
    -dias_enfermo: u32
    +new(x: f32, y: f32, especie: Especie)
    +crear_cria(x: f32, y: f32, especie: Especie)
    +mover_hacia(tx: f32, ty: f32)
    +num_crias(especie: Especie, rng)
    +actualizar()
    +dibujar()
    +x(): f32
    +y(): f32
    +r(): f32
    +especie(): Especie
    +esta_vivo(): bool
    +matar()
    +Getters/Setters...
}

class Depredador {
    -x: f32
    -y: f32
    -vx: f32
    -vy: f32
    -r: f32
    -vivo: bool
    -reserva: f32
    -cooldown: f32
    -salud: EstadoSalud
    -dias_enfermo: u32
    +new(x: f32, y: f32)
    +mover_hacia(tx: f32, ty: f32)
    +actualizar()
    +dibujar()
    +x(): f32
    +y(): f32
    +r(): f32
    +especie(): Especie
    +esta_vivo(): bool
    +matar()
    +Getters/Setters...
}

%% Trait e interfaces en el medio
class Organismo {
    <<trait>>
    +actualizar()
    +dibujar()
    +x(): f32
    +y(): f32
    +r(): f32
    +especie(): Especie
    +esta_vivo(): bool
    +matar()
}

%% Enums
class Especie {
    <<enum>>
    +Conejo
    +Raton
    +Ardilla
    +color()
    +nombre()
    +gompertz_params()
    +probabilidad_macho()
    +poblacion_maxima()
    +edad_sacrificio()
    +edad_reproduccion()
}

class Sexo {
    <<enum>>
    +Macho
    +Hembra
    +nombre()
}

class EstadoSalud {
    <<enum>>
    +Sano
    +Enfermo
    +nombre()
}

%% Relaciones
Presa ..|> Organismo
Depredador ..|> Organismo
Presa --> Especie
Presa --> Sexo
Presa --> EstadoSalud
Depredador --> EstadoSalud
