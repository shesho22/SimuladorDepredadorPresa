## Diagrama de Contexto
[<- Volvere](README.md)

## Justificacion 
- Separar los parametros para maximisar el control de la calibracion 
- El sistema se centra en un ciclo diario de simulación, separando claramente la lógica del simulador de las entidades externas y la interfaz.
- Se usan entidades externas como fuentes de datos (presas, depredadores, parámetros, RNG) para mantener al sistema desacoplado y modular.
- La salida de datos hacia la UI y reportes asegura observabilidad sin interferir con la lógica central, favoreciendo mantenimiento y escalabilidad.


```mermaid
graph LR
    %% Sistema central
    subgraph Sistema["Simulador Depredador-Presa"]
        S["Procesa ciclos diarios, salud, depredación, reproducción y reportes"]
    end

    %% Entidades externas
    Ui["Pantalla/Reporte"]
    Presas["Vector de Presas"]
    Depredadores["Vector de Depredadores"]
    Config["Constantes y parámetros del sistema"]
    RNG["Generador Aleatorio"]

    %% Entradas al sistema
    Presas -->|Proporciona estado, posición y salud| S
    Depredadores -->|Proporciona estado, posición, salud y reservas| S
    Config -->|Define constantes, probabilidades y límites| S
    RNG -->|Proporciona valores aleatorios| S

    %% Salidas del sistema
    S -->|Actualiza movimiento, salud y reproducción| Presas
    S -->|Actualiza movimiento, salud y depredación| Depredadores
    S -->|Genera reportes diarios y estadísticas| Ui
    S -->|Actualiza interfaz| Ui

    %% Estilos adicionales
    class Ui,Presas,Depredadores,Config,RNG externo;
    class S sistema;
