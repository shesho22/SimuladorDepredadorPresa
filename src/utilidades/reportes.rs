use crate::entidades::organismo::Organismo;
use crate::entidades::{Presa, Depredador, Especie, EstadoSalud};
use crate::utilidades::csv::EstadisticasDiarias;

/// Compila un reporte diario a partir del estado actual de presas y depredadores
pub fn compilar_reporte_diario(
    presas: &[Presa],
    depredadores: &[Depredador],
    muertes_pred: u32,
    muertes_enf: u32,
    nuevos_infectados: u32,
    recuperaciones: u32,
    reproducciones: u32,
    dia: u32,
) -> EstadisticasDiarias {
    let mut conteo = [0usize; 3];

    for p in presas {
        if p.esta_vivo() {
            match p.especie() {
                Especie::Conejo => conteo[0] += 1,
                Especie::Raton => conteo[1] += 1,
                Especie::Ardilla => conteo[2] += 1,
            }
        }
    }

    let dep_enfermos = depredadores.iter()
        .filter(|d| d.esta_vivo() && d.salud() == EstadoSalud::Enfermo)
        .count();

    EstadisticasDiarias {
        dia,
        conteo_conejos: conteo[0],
        conteo_ratones: conteo[1],
        conteo_ardillas: conteo[2],
        conteo_total: conteo.iter().sum(),
        muertes_por_predacion: muertes_pred,
        muertes_por_enfermedad: muertes_enf,
        nuevos_infectados,
        recuperaciones,
        reproducciones,
        depredadores_enfermos: dep_enfermos,
        depredadores_vivos: depredadores.iter().filter(|d| d.esta_vivo()).count(),
    }
}
