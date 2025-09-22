## Diagrama de Secuencia o diagrama de flujo de datos UML
[<- Volvere](README.md)

## Justificacion 
- El diseño del flujo diario sigue un patrón de ciclo central: inicialización → actualización → procesos diarios → reportes → interfaz.
- El paso 0 de inicialización asegura que todas las variables críticas, vectores de entidades y generadores de aleatoriedad estén listos, evitando efectos colaterales en la simulación.
- Cada proceso (enfermedad, movimiento, reproducción, depredación) está aislado y ordenado, permitiendo flexibilidad para modificar la lógica de un paso sin romper los demás.
- Se integra RNG y parámetros externos como fuentes controladas, garantizando que la simulación sea reproducible y configurable.


```mermaid
flowchart LR
    %% ==================== ENTIDADES EXTERNAS ====================
    EX1["Usuario / Pantalla"]
    EX2["Generador Aleatorio (RNG)"]
    EX3["Constantes y Parámetros"]

    %% ==================== ALMACENES DE DATOS ====================
    D1["Vector de Presas"]
    D2["Vector de Depredadores"]
    D3["Reportes diarios"]

    %% ==================== PROCESOS ====================
    P0["0: Inicializar simulador"]
    P1["1: Resetear contadores diarios"]
    P2["2: Actualizar presas diarias"]
    P2_1["2.1: Incrementar edad"]
    P2_2["2.2: Actualizar peso"]
    P2_3["2.3: Activar modo reproducción si cumple la edad"]

    P3["3: Procesar enfermedad de presas"]
    P3_1["3.1: Puede enfermar"]
    P3_2["3.2: Puede recuperar"]
    P3_3["3.3: Si no se recupera, muere"]

    P4["4: Procesar dieta de depredadores"]
    P4_1["4.1: Consumo diario"]
    P4_2["4.2: Consume según umbrales"]
    P4_3["4.3: Si cubre el más alto sana si está enfermo"]
    P4_4["4.4: Si pasa varios días sin sanar, muere"]

    P5["5: Movimiento y reproducción de presas"]
    P5_1["5.1: Busca pareja si está en modo_reproducción"]
    P5_2["5.2: Se mueve aleatoriamente si no"]

    P6["6: Depredadores cazan y comen"]
    P6_1["6.1: Buscar presas que pasen edad de sacrificio"]
    P6_2["6.2: Seleccionar la más pesada"]
    P6_3["6.3: Atacar y comer presa"]

    P7["7: Limpiar presas muertas"]
    P8["8: Actualizar y dibujar UI"]

    %% ==================== FLUJOS DE DATOS ====================
    %% Inicialización
    EX2 --> P0
    EX3 --> P0
    P0 --> D1
    P0 --> D2
    P0 --> D3

    %% Paso 2
    D1 --> P2
    P2 --> P2_1
    P2_1 --> P2_2
    P2_2 --> P2_3
    P2_3 --> D1

    %% Paso 3
    D1 --> P3
    EX2 --> P3
    P3 --> P3_1
    P3_1 --> P3_2
    P3_2 --> P3_3
    P3_3 --> D1

    %% Paso 4
    D2 --> P4
    P4 --> P4_1
    P4_1 --> P4_2
    P4_2 --> P4_3
    P4_3 --> P4_4
    P4_4 --> D2

    %% Paso 5
    D1 --> P5
    EX2 --> P5
    P5 --> P5_1
    P5 --> P5_2
    P5_1 --> D1
    P5_2 --> D1

    %% Paso 6
    D2 --> P6
    D1 --> P6
    P6 --> P6_1
    P6_1 --> P6_2
    P6_2 --> P6_3
    P6_3 --> D2
    P6_3 --> D1

    %% Paso 7 y 8
    P7 --> D1
    D1 --> P8
    D2 --> P8
    P8 --> D3
    P8 --> EX1

    %% Reinicio de contadores diarios
    P1 --> P3
