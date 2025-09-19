use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct EstadisticasDiarias {
    pub dia: u32,
    pub conteo_conejos: usize,
    pub conteo_ratones: usize,
    pub conteo_ardillas: usize,
    pub conteo_total: usize,
    pub muertes_por_predacion: u32,
    pub muertes_por_enfermedad: u32,
    pub nuevos_infectados: u32,
    pub recuperaciones: u32,
    pub reproducciones: u32,
    pub depredadores_enfermos: usize,
    pub depredadores_vivos: usize,
}

pub fn guardar_reportes_csv(reportes: &Vec<EstadisticasDiarias>, ruta: &str) -> csv::Result<()> {
    let mut wtr = csv::Writer::from_path(ruta)?;
    for rep in reportes {
        wtr.serialize(rep)?; // convierte struct -> fila CSV
    }
    wtr.flush()?;
    Ok(())
}
