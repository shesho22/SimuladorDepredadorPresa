use ::rand::thread_rng;
use macroquad::prelude::*;

mod entidades;
mod sistemas;
mod ui {
    pub mod interfaz;
}
mod utilidades;

// Traer lo necesario desde los re-exports
use crate::entidades::{Presa, Depredador, Especie, Organismo};
use crate::sistemas::*;
use crate::utilidades::*;
use ui::interfaz::dibujar_ui;


// ==================== MAIN ====================
#[macroquad::main("Presa-Depredador")]
async fn main() {
    let mut rng = thread_rng();
    let especies = [Especie::Conejo, Especie::Raton, Especie::Ardilla];

    let mut presas: Vec<Presa> = inicializar_presas(&especies, 50, &mut rng);
    let mut depredadores: Vec<Depredador> = inicializar_depredadores(1, &mut rng);

    let mut dias: u32 = 0;
    let mut tiempo_acumulado: f32 = 0.0;

    // Contadores diarios
    let mut muertes_por_predacion_diarias = 0;
    let mut muertes_por_enfermedad_diarias = 0;
    let mut nuevos_infectados_diarios = 0;
    let mut recuperaciones_diarias = 0;
    let mut reproducciones_diarias = 0;

    let mut reportes: Vec<EstadisticasDiarias> = Vec::new();

    loop {
        clear_background(LIGHTGRAY);
        tiempo_acumulado += get_frame_time();
        if tiempo_acumulado >= DURACION_DIA {
            dias += 1;
            tiempo_acumulado = 0.0;
            // Procesos diarios
            resetear_contadores_diarios(&mut muertes_por_predacion_diarias,&mut muertes_por_enfermedad_diarias,&mut nuevos_infectados_diarios,&mut recuperaciones_diarias);
            actualizar_presas_diarias(&mut presas);
            procesar_enfermedad_presas(&mut presas, &mut rng,&mut nuevos_infectados_diarios,&mut recuperaciones_diarias,&mut muertes_por_enfermedad_diarias);
            procesar_dietas_depredadores(&mut depredadores, dias);
            // Guardar reporte diario
            reportes.push(compilar_reporte_diario(&presas, &depredadores,muertes_por_predacion_diarias,muertes_por_enfermedad_diarias,nuevos_infectados_diarios,recuperaciones_diarias,reproducciones_diarias,dias));
            reproducciones_diarias = 0;
        }
        // ==================== Movimiento inteligente ====================
        movimiento_presas(&mut presas, &mut rng);
        depredadores_buscar_presas(&mut depredadores, &presas);
        // Actualizar y dibujar organismos
        actualizar_y_dibujar(&mut presas);
        actualizar_y_dibujar(&mut depredadores);
        // Reproducción
        reproduccion(&mut presas, &especies, &mut reproducciones_diarias);
        // Depredadores comen
        depredadores_comer(&mut depredadores, &mut presas, &mut muertes_por_predacion_diarias);
        // Limpiar presas muertas
        presas.retain(|p| p.esta_vivo());
        // Interfaz
        dibujar_ui(dias, &presas, &depredadores, &reportes);
        next_frame().await;
    }
}

// ==================== FUNCIONES AUXILIARES ====================
fn actualizar_y_dibujar<T: Organismo>(organismos: &mut [T]) {
    for o in organismos.iter_mut() {
        o.actualizar();
        o.dibujar();
    }
}






