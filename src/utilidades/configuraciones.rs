// ==================== CONFIGURACIONES ====================

// Dia 
pub const DURACION_DIA: f32 = 1.0;

// Presas
pub const MAX_DIAS_SIN_RECUPERAR_PRESA: u32 = 5;

pub const RADIO_PRESA: f32 = 8.0;
pub const VEL_MAX_PRESA: f32 = 1.5;

pub const PROB_ENFERMAR_DIARIA_PRESA: f32 = 0.02;
pub const PROB_RECUPERACION_DIARIA_PRESA: f32 = 0.30;

pub const EDAD_MINIMA_SACRIFICIO_CONEJO: u32 = 6;
pub const EDAD_MINIMA_SACRIFICIO_RATON: u32 = 3;
pub const EDAD_MINIMA_SACRIFICIO_ARDILLA: u32 = 5;

pub const EDAD_MINIMA_REPRODUCCION_CONEJO: u32 = 10;
pub const EDAD_MINIMA_REPRODUCCION_RATON: u32 = 6;
pub const EDAD_MINIMA_REPRODUCCION_ARDILLA: u32 = 8;

pub const POBLACION_MAXIMA_CONEJO: usize = 30;
pub const POBLACION_MAXIMA_RATON: usize = 40;
pub const POBLACION_MAXIMA_ARDILLA: usize = 25;

pub const PROB_CRIAS_CONEJO: [f32; 7] = [0.02, 0.15, 0.40, 0.25, 0.10, 0.06, 0.02];
pub const PROB_CRIAS_RATON: [f32; 5] = [0.10, 0.30, 0.40, 0.15, 0.05];
pub const PROB_CRIAS_ARDILLA: [f32; 4] = [0.20, 0.50, 0.20, 0.10];

pub const RADIO_APARICION_CRIA: f32 = 4.0;
pub const RUIDO_MOVIMIENTO: f32 = 0.05;

// Depredadores
pub const CONSUMO_DIARIO_DEPREDADOR: f32 = 2.5;
pub const UMBRAL_OPTIMO_DEPREDADOR: f32 = 30.0;
pub const UMBRAL_MINIMO_DEPREDADOR: f32 = 12.0;
pub const UMBRAL_DEFICIENTE_DEPREDADOR: f32 = 3.0;
pub const MAX_DIAS_SIN_RECUPERAR_DEPREDADOR: u32 = 7;
pub const DIAS_INMUNIDAD: u32 = 7;
pub const TIEMPO_ESPERA_COMIDA: f32 = 1.0;
pub const RADIO_DEPREDADOR: f32 = 12.0;
pub const VEL_MAX_DEPREDADOR: f32 = 2.0;

// Probabilidades machos
pub const PROB_MACHO_CONEJO: f32 = 0.55;
pub const PROB_MACHO_RATON: f32 = 0.50;
pub const PROB_MACHO_ARDILLA: f32 = 0.45;
