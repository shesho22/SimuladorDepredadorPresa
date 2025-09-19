use ::rand::Rng;
use ::rand::thread_rng;
use macroquad::prelude::*;

mod entidades;
mod sistemas;
mod ui {
    pub mod interfaz;
}
mod utilidades {
    pub mod csv;
    pub mod configuraciones;
}

use crate::entidades::{Presa, Depredador, Especie, EstadoSalud,Organismo,Sexo};
use ui::interfaz::dibujar_ui;
use utilidades::csv::{EstadisticasDiarias, guardar_reportes_csv};
use utilidades::configuraciones::*;
use crate::sistemas::colisiones::colision;


// ==================== MAIN ====================
#[macroquad::main("Presa-Depredador")]
async fn main() {
    let mut rng = thread_rng();
    let especies = [Especie::Conejo, Especie::Raton, Especie::Ardilla];

    let mut presas: Vec<Presa> = (0..50)
        .map(|_| {
            let especie = especies[rng.gen_range(0..especies.len())];
            Presa::new(
                rng.gen_range(0.0..screen_width()),
                rng.gen_range(0.0..screen_height()),
                especie,
            )
        })
        .collect();

    let mut depredadores: Vec<Depredador> = (0..1)
        .map(|_| Depredador::new(
            rng.gen_range(0.0..screen_width()),
            rng.gen_range(0.0..screen_height())
        ))
        .collect();

    let mut dias: u32 = 0;
    let mut tiempo_acumulado: f32 = 0.0;

    // Contadores diarios (se reinician cada día)
    let mut muertes_por_predacion_diarias: u32 = 0;
    let mut muertes_por_enfermedad_diarias: u32 = 0;
    let mut nuevos_infectados_diarios: u32 = 0;
    let mut recuperaciones_diarias: u32 = 0;
    let mut reproducciones_diarias: u32 = 0;

    let mut reportes: Vec<EstadisticasDiarias> = Vec::new();

    loop {
        clear_background(LIGHTGRAY);

        tiempo_acumulado += get_frame_time();
        if tiempo_acumulado >= 1.0 {
            dias += 1;
            tiempo_acumulado = 0.0;

                        // resetear contadores diarios
            muertes_por_predacion_diarias = 0;
            muertes_por_enfermedad_diarias = 0;
            nuevos_infectados_diarios = 0;
            recuperaciones_diarias = 0;
            // --- procesos diarios relacionados con presas ---
            for p in presas.iter_mut() {
                p.set_edad(p.edad() + 1);
                // Fórmula Gompertz para actualizar peso
                let (a, b, c) = p.especie().gompertz_params();
                p.set_peso(a * (-b * f32::exp(-c * p.edad() as f32)).exp());
                if p.edad() >= p.especie().edad_reproduccion() {
                    p.set_modo_reproduccion(true);
                }
            }



            // --- procesos diarios relacionados con enfermedades en presas ---
            for p in presas.iter_mut() {
                if !p.esta_vivo() { continue; }
                match p.salud() {
                    EstadoSalud::Sano => {
                        // posibilidad diaria de enfermar
                        if rng.gen_range(0.0..1.0) < PROB_ENFERMAR_DIARIA_PRESA {
                            p.set_salud(EstadoSalud::Enfermo);
                            p.reset_dias_enfermo();
                            nuevos_infectados_diarios += 1;
                        }
                    }
                    EstadoSalud::Enfermo => {
                        p.incrementar_dias_enfermo();
                        // posibilidad de recuperación diaria
                        if rng.gen_range(0.0..1.0) < PROB_RECUPERACION_DIARIA_PRESA {
                            p.set_salud(EstadoSalud::Sano);
                            p.reset_dias_enfermo();
                            recuperaciones_diarias += 1;
                        } else if p.dias_enfermo() >= MAX_DIAS_SIN_RECUPERAR_PRESA {
                            // muere por enfermedad
                            p.matar();
                            muertes_por_enfermedad_diarias += 1;
                        }
                    }
                }
            }

            // --- procesos diarios relacionados con depredadores: consumo de reservas y consecuencias ---
            for d in depredadores.iter_mut() {
                if !d.esta_vivo() { continue; }
                // consumir reservas por metabolismo
                if d.reserva() >= CONSUMO_DIARIO_DEPREDADOR {
                    d.set_reserva(d.reserva() - CONSUMO_DIARIO_DEPREDADOR);
                } else {
                    // si no tiene suficiente, se queda en 0
                    d.set_reserva((d.reserva() - CONSUMO_DIARIO_DEPREDADOR).max(0.0));

                }
                    // 🔹 aplicar inmunidad en los primeros días
                if dias <= DIAS_INMUNIDAD {
                    // durante los días de inmunidad no se enferma
                    d.set_salud(EstadoSalud::Sano);
                    d.reset_dias_enfermo();
                    continue; // saltar al siguiente depredador
                }

                // evaluar estado según umbrales
                if d.reserva() >= UMBRAL_OPTIMO_DEPREDADOR {
                    // óptimo: recupera si estaba enfermo
                    if d.salud() == EstadoSalud::Enfermo {
                        d.set_salud(EstadoSalud::Sano);
                        d.reset_dias_enfermo();
                    } else {
                        d.reset_dias_enfermo();
                    }
                } else if d.reserva() >= UMBRAL_MINIMO_DEPREDADOR {
                    // mínimo: se mantiene sano, pero no recupera enfermos (por regla dada)
                    d.reset_dias_enfermo();
                    d.set_salud(EstadoSalud::Sano);
                } else if d.reserva() >= UMBRAL_DEFICIENTE_DEPREDADOR {
                    // entre deficiente y mínimo: riesgo; acumula días en estado precario
                    d.incrementar_dias_enfermo();
                    // no se declara enfermo inmediatamente, pero si se prolonga, pasa a enfermo
                    if d.dias_enfermo() > 2 {
                        d.set_salud(EstadoSalud::Enfermo);
                    }
                    if d.dias_enfermo() >= MAX_DIAS_SIN_RECUPERAR_DEPREDADOR {
                        d.matar();
                    }
                } else {
                    // por debajo de deficiente: se enferma y puede morir si se prolonga
                    d.set_salud(EstadoSalud::Enfermo);
                    d.incrementar_dias_enfermo();
                    if d.dias_enfermo() >= MAX_DIAS_SIN_RECUPERAR_DEPREDADOR {
                        d.matar();
                    }
                }
            }

            // --- compilar y guardar reporte del día anterior (conteos actuales)
            let mut conteo = [0usize, 0usize, 0usize];
            let mut dep_enfermos = 0usize;
            for p in &presas {
                if p.esta_vivo() {
                    match p.especie() {
                        Especie::Conejo => conteo[0] += 1,
                        Especie::Raton => conteo[1] += 1,
                        Especie::Ardilla => conteo[2] += 1,
                    }
                }
            }
            for d in &depredadores {
                if d.esta_vivo() && d.salud() == EstadoSalud::Enfermo {
                    dep_enfermos += 1;
                }
            }

            reportes.push(EstadisticasDiarias {
                dia: dias,
                conteo_conejos: conteo[0],
                conteo_ratones: conteo[1],
                conteo_ardillas: conteo[2],
                conteo_total: conteo.iter().sum(),
                muertes_por_predacion: muertes_por_predacion_diarias,
                muertes_por_enfermedad: muertes_por_enfermedad_diarias,
                nuevos_infectados: nuevos_infectados_diarios,
                recuperaciones: recuperaciones_diarias,
                reproducciones: reproducciones_diarias,
                depredadores_enfermos: dep_enfermos,
                depredadores_vivos: depredadores.iter().filter(|d| d.esta_vivo()).count(),

            });
            reproducciones_diarias = 0;
        }
        // ==================== Movimiento inteligente y modo cuando se alcanza máxima población ====================
        // Movimiento inteligente: buscar pareja
        for i in 0..presas.len() {
            if !presas[i].esta_vivo() { continue; }
            if !presas[i].modo_reproduccion() { continue; }
            if presas[i].cooldown() > 0.0 { continue; }

            let mut pareja_index: Option<usize> = None;
            let mut dist_min = f32::MAX;

            for j in 0..presas.len() {
                if i == j || !presas[j].esta_vivo() { continue; }
                if presas[i].especie() != presas[j].especie() { continue; }
                if presas[i].sexo() == presas[j].sexo() { continue; }
                if !presas[j].modo_reproduccion() || presas[j].cooldown() > 0.0 { continue; }

                let dx = presas[j].x() - presas[i].x();
                let dy = presas[j].y() - presas[i].y();
                let dist = (dx*dx + dy*dy).sqrt();

                if dist < dist_min {
                    dist_min = dist;
                    pareja_index = Some(j);
                }
            }

            if let Some(j) = pareja_index {
                let pareja = &presas[j];

                // Vector hacia la pareja + dispersión aleatoria
                let mut dx = pareja.x() - presas[i].x() + rng.gen_range(-RADIO_PRESA..RADIO_PRESA);
                let mut dy = pareja.y() - presas[i].y() + rng.gen_range(-RADIO_PRESA..RADIO_PRESA);
                let dist = (dx*dx + dy*dy).sqrt();

                if dist > 0.0 {
                    presas[i].set_vx(dx / dist * VEL_MAX_PRESA);
                    presas[i].set_vy(dy / dist * VEL_MAX_PRESA);
                } else {
                    // Si no hay pareja, moverse aleatoriamente
                    // Obtener los valores actuales
                    let mut vx = presas[i].vx();
                    let mut vy = presas[i].vy();
                    // Añadir ruido
                    vx += rng.gen_range(-RUIDO_MOVIMIENTO..RUIDO_MOVIMIENTO);
                    vy += rng.gen_range(-RUIDO_MOVIMIENTO..RUIDO_MOVIMIENTO);
                    // Limitar velocidad máxima
                    let vel = (vx.powi(2) + vy.powi(2)).sqrt();
                    if vel > VEL_MAX_PRESA {
                        vx = vx / vel * VEL_MAX_PRESA;
                        vy = vy / vel * VEL_MAX_PRESA;
                    }
                    // Aplicar valores calculados usando setters
                    presas[i].set_vx(vx);
                    presas[i].set_vy(vy);


                }
            }
        }



        // Depredador busca presa más grande, pero sólo entre las presas que superen la edad de sacrificio
        for d in depredadores.iter_mut() {
            if let Some(obj) = presas.iter()
                .filter(|p| p.esta_vivo() && p.edad() >= p.especie().edad_sacrificio())
                .min_by(|a, b| {
                    // Primero comparamos por peso (mayor peso primero)
                    let cmp_peso = b.peso_actual().partial_cmp(&a.peso_actual()).unwrap();
                    if cmp_peso == std::cmp::Ordering::Equal {
                        // Si el peso es igual, comparamos por distancia al depredador (más cercano primero)
                        let dist_a = (a.x() - d.x()).powi(2) + (a.y() - d.y()).powi(2);
                        let dist_b = (b.x() - d.x()).powi(2) + (b.y() - d.y()).powi(2);
                        dist_a.partial_cmp(&dist_b).unwrap()
                    } else {
                        cmp_peso
                    }
                })
            {
                d.mover_hacia(obj.x(), obj.y());
            }
        }


        

        // Actualizar y dibujar presas
        for p in presas.iter_mut() {
            p.actualizar();
            p.dibujar();
        }

        // Actualizar y dibujar depredadores
        for d in depredadores.iter_mut() {
            d.actualizar();
            d.dibujar();
        }

        // Reproducción
        // ----------------- REPRODUCCIÓN (reemplaza tu bloque actual) -----------------
        let mut nuevas_presas: Vec<Presa> = Vec::new();
        let mut rng2 = thread_rng();

        // Guardamos qué índices deben tener cooldown
        let mut parejas_repro: Vec<(usize, usize, Especie)> = Vec::new();
        let mut propuestas: Vec<(Especie, Presa)> = Vec::new();

        for i in 0..presas.len() {
            for j in (i + 1)..presas.len() {
                if !presas[i].esta_vivo() || !presas[j].esta_vivo() { continue; }

                if presas[i].especie() == presas[j].especie()
                    && colision(&presas[i], &presas[j])
                    && presas[i].cooldown() <= 0.0
                    && presas[j].cooldown() <= 0.0
                    && presas[i].edad() >= presas[i].especie().edad_reproduccion()
                    && presas[j].edad() >= presas[j].especie().edad_reproduccion()
                {
                    // asegurar pareja macho/hembra
                    if (presas[i].sexo() == Sexo::Macho && presas[j].sexo() == Sexo::Hembra)
                        || (presas[i].sexo() == Sexo::Hembra && presas[j].sexo() == Sexo::Macho)
                    {
                        let especie = presas[i].especie();
                        let count_actual = presas.iter().filter(|p| p.especie() == especie).count();
                        let max_pobl = especie.poblacion_maxima();

                        // muestrear cuántas crías nacerán
                        let n_crias = Presa::num_crias(especie, &mut rng2);

                        // cuántas se pueden añadir sin pasar el límite
                        let espacio = if max_pobl > count_actual { max_pobl - count_actual } else { 0 };
                        let n_a_crear = n_crias.min(espacio);
                        if n_a_crear > 0 {
                            reproducciones_diarias += 1;
                        }

                        for _ in 0..n_a_crear {
                            let dx = rng2.gen_range(-RADIO_APARICION_CRIA..RADIO_APARICION_CRIA);
                            let dy = rng2.gen_range(-RADIO_APARICION_CRIA..RADIO_APARICION_CRIA);
                            nuevas_presas.push(Presa::crear_cria(
                                presas[i].x() + dx,
                                presas[i].y() + dy,
                                especie,
                                &mut rng2
                            ));
                        }

                        // Guardar la pareja para aplicar cooldown luego
                        parejas_repro.push((i, j, especie));
                    }
                }
                // Filtrar propuestas respetando límites
                for especie in &especies {
                    let count_actual = presas.iter().filter(|p| p.especie() == *especie).count();
                    let max_pobl = especie.poblacion_maxima();
                    let espacio = max_pobl.saturating_sub(count_actual);

                    // solo dejar tantas como espacio haya
                    let mut propuestas_especie: Vec<_> =
                        propuestas.iter().filter(|(esp, _)| *esp == *especie).collect();

                    propuestas_especie.truncate(espacio);

                    for (_, cria) in propuestas_especie {
                        nuevas_presas.push(cria.clone());
                    }
                }
            }
        }

        // Agregar las crías
        presas.extend(nuevas_presas);

        // Ahora sí, aplicar cooldown a los progenitores
        for (i, j, _) in parejas_repro {
            presas[i].set_cooldown(2.0);
            presas[j].set_cooldown(2.0);
        }



        // Depredador come presa (sólo si la presa cumple edad de sacrificio y si el depredador no está en cooldown)
        for d in depredadores.iter_mut() {
            if d.cooldown() <= 0.0 {
                for p in presas.iter_mut() {
                    if p.esta_vivo() && p.edad() >= p.especie().edad_sacrificio() && colision(d, p) {
                        d.set_reserva(d.reserva() + p.peso_actual());
                        p.matar();
                        muertes_por_predacion_diarias += 1;
                        d.set_cooldown(TIEMPO_ESPERA_COMIDA); // tiempo de espera entre comidas
                        break;
                    }
                }
            }
        }

        presas.retain(|p| p.esta_vivo());

        // Interfaz gráfica
        dibujar_ui(dias, &presas, &depredadores, &reportes);
        

        next_frame().await;
    }
}

