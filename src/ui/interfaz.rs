use macroquad::prelude::*;
use crate::entidades::{Presa,Depredador,Especie};
use crate::utilidades::csv::{EstadisticasDiarias,guardar_reportes_csv};

pub fn dibujar_ui(
    dias: u32,
    presas: &Vec<Presa>,
    depredadores: &Vec<Depredador>,
    reportes: &Vec<EstadisticasDiarias>,
) {
    let mut conteo = [0, 0, 0];
    let mut suma_edades = [0u32, 0, 0];
    let mut suma_pesos = [0.0f32, 0.0, 0.0];

    // === Recolección de datos ===
    for p in presas {
        match p.especie() {
            Especie::Conejo => {
                conteo[0] += 1;
                suma_edades[0] += p.edad();
                suma_pesos[0] += p.peso();
            }
            Especie::Raton => {
                conteo[1] += 1;
                suma_edades[1] += p.edad();
                suma_pesos[1] += p.peso();
            }
            Especie::Ardilla => {
                conteo[2] += 1;
                suma_edades[2] += p.edad();
                suma_pesos[2] += p.peso();
            }
        }
    }

    // === Funciones locales para promedios ===
    let promedio = |suma: u32, count: i32| -> f32 {
        if count > 0 {
            suma as f32 / count as f32
        } else {
            0.0
        }
    };
    let promedio_peso = |suma: f32, count: i32| -> f32 {
        if count > 0 {
            suma / count as f32
        } else {
            0.0
        }
    };

    // === Texto de cabecera ===
    draw_text(
        &format!("Día: {} | Esc: para finalizar y generar reporte", dias),
        10.0,
        20.0,
        20.0,
        BLACK,
    );

    // === Información por especie ===
    draw_text(
        &format!(
            "Conejos: {} (edad promedio: {:.1}, peso promedio: {:.1})",
            conteo[0],
            promedio(suma_edades[0], conteo[0]),
            promedio_peso(suma_pesos[0], conteo[0])
        ),
        10.0,
        50.0,
        20.0,
        Especie::Conejo.color(),
    );

    draw_text(
        &format!(
            "Ratones: {} (edad promedio: {:.1}, peso promedio: {:.1})",
            conteo[1],
            promedio(suma_edades[1], conteo[1]),
            promedio_peso(suma_pesos[1], conteo[1])
        ),
        10.0,
        70.0,
        20.0,
        Especie::Raton.color(),
    );

    draw_text(
        &format!(
            "Ardillas: {} (edad promedio: {:.1}, peso promedio: {:.1})",
            conteo[2],
            promedio(suma_edades[2], conteo[2]),
            promedio_peso(suma_pesos[2], conteo[2])
        ),
        10.0,
        90.0,
        20.0,
        Especie::Ardilla.color(),
    );

    // === Información de depredadores ===
    for (i, d) in depredadores.iter().enumerate() {
        draw_text(
            &format!(
                "Depredador {} peso: {:.1} estado: {}",
                i + 1,
                d.reserva(),
                d.salud().nombre()
            ),
            10.0,
            130.0 + i as f32 * 20.0,
            20.0,
            RED,
        );
    }

    // === Guardar reporte con ESC ===
    if is_key_pressed(KeyCode::Escape) {
        if let Err(e) = guardar_reportes_csv(reportes, "reportes.csv") {
            eprintln!("Error guardando CSV: {}", e);
        } else {
            println!("Reportes guardados en reportes.csv");
        }
        std::process::exit(0); // cerrar limpio
    }
}
