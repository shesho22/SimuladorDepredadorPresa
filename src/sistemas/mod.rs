pub mod inicializacion;
pub mod diario;
pub mod movimiento;
pub mod reproduccion;
pub mod depredacion;
pub mod colision;

pub use inicializacion::{inicializar_presas, inicializar_depredadores};
pub use diario::{resetear_contadores_diarios, actualizar_presas_diarias, procesar_enfermedad_presas, procesar_dietas_depredadores};
pub use movimiento::{movimiento_presas, depredadores_buscar_presas};
pub use reproduccion::reproduccion;
pub use depredacion::depredadores_comer;
