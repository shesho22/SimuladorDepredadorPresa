## Diagrama de Secuencia o diagrama de flujo de datos UML
[<- Volvere](README.md)

## Justificacion 
- El diseño del flujo diario sigue un patrón de ciclo central: inicialización → actualización → procesos diarios → reportes → interfaz.
- El paso 0 de inicialización asegura que todas las variables críticas, vectores de entidades y generadores de aleatoriedad estén listos, evitando efectos colaterales en la simulación.
- Cada proceso (enfermedad, movimiento, reproducción, depredación) está aislado y ordenado, permitiendo flexibilidad para modificar la lógica de un paso sin romper los demás.
- Se integra RNG y parámetros externos como fuentes controladas, garantizando que la simulación sea reproducible y configurable.


```mermaid
flowchart RL
    %% ==================== ENTIDADES EXTERNAS ====================
    subgraph EX["Entidades Externas"]
        Usuario["Usuario / Pantalla"]
        RNG["Generador Aleatorio"]
        Config["Constantes y Parámetros"]
    end

    %% ==================== ALMACENES DE DATOS ====================
    subgraph DATA["Almacenes de Datos"]
        Presas["Vector de Presas"]
        Depredadores["Vector de Depredadores"]
        Reportes["Reportes diarios"]
    end

    %% ==================== PROCESOS PRINCIPALES ====================
    subgraph PROC["Ciclo Principal Diario"]
        P0["Paso 0: Inicializar simulador (RNG, presas, depredadores, contadores, reportes)"]
        P1["Paso 1: Resetear contadores diarios"]
        P2["Paso 2: Actualizar presas diarias"]
        P3["Paso 3: Procesar enfermedad de presas"]
        P4["Paso 4: Procesar dieta de depredadores"]
        P5["Paso 5: Movimiento inteligente de presas y depredadores"]
        P6["Paso 6: Reproducción de presas"]
        P7["Paso 7: Depredadores comen"]
        P8["Paso 8: Limpiar presas muertas"]
        P9["Paso 9: Actualizar y dibujar UI"]
    end

    %% ==================== FLUJOS DE DATOS ====================
    RNG --> P0
    Config --> P0
    P0 --> Presas
    P0 --> Depredadores
    P0 --> Reportes

    Presas -->|Estado, posición, salud| P2
    P2 --> Presas
    Presas -->|Datos para infección| P3
    RNG -->|Valores aleatorios| P3
    P3 --> Presas
    Depredadores --> P4
    P4 --> Depredadores
    P5 --> Presas
    P5 --> Depredadores
    RNG -->|Movimientos aleatorios| P5
    P6 --> Presas
    RNG -->|Decisiones probabilísticas| P6
    Depredadores -->|Objetivos de depredación| P7
    P7 --> Presas
    P7 --> Depredadores
    P1 -->|Reinicia contadores| P3
    Presas -->|Datos para reportes| Reportes
    Depredadores -->|Datos para reportes| Reportes
    P9 --> Reportes
    P9 --> Usuario
    Config -->|Parámetros del sistema| P2
    Config --> P4
    Config --> P5
    Config --> P6
