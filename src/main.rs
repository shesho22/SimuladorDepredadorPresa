use ::rand::Rng;
use ::rand::thread_rng;
use macroquad::prelude::*;


// ==================== CONFIGURACIONES ====================

pub const MAX_DIAS_SIN_RECUPERAR_PRESA: u32 = 5; // si pasa esto, la presa muere

pub const CONSUMO_DIARIO_DEPREDADOR: f32 = 2.5; // gasto diario de reservas del depredador
pub const UMBRAL_OPTIMO_DEPREDADOR: f32 = 30.0; // depredador umbral óptimo de reservas
pub const UMBRAL_MINIMO_DEPREDADOR: f32 = 12.0; // depredador umbral mínimo de reservas
pub const UMBRAL_DEFICIENTE_DEPREDADOR: f32 = 3.0; // depredador umbral deficiente de reservas
pub const MAX_DIAS_SIN_RECUPERAR_DEPREDADOR: u32 = 7;
pub const DIAS_INMUNIDAD: u32 = 7; // número de días protegidos al inicio


pub const TIEMPO_ESPERA_COMIDA:f32 = 1.0; // depredador tiempo de espera entre comidas

pub const RADIO_PRESA: f32 = 8.0;
pub const RADIO_DEPREDADOR: f32 = 12.0;

pub const VEL_MAX_PRESA: f32 = 1.5;
pub const VEL_MAX_DEPREDADOR: f32 = 2.0;

// Probabilidades 
pub const PROB_MACHO_CONEJO: f32 = 0.55;
pub const PROB_MACHO_RATON: f32 = 0.50;
pub const PROB_MACHO_ARDILLA: f32 = 0.45;

pub const PROB_ENFERMAR_DIARIA_PRESA: f32 = 0.02; // probabilidad diaria de enfermar (presas)
pub const PROB_RECUPERACION_DIARIA_PRESA: f32 = 0.30; // probabilidad diaria de recuperarse (presas)

// Edad minima (en días) a partir de la cual la presa puede ser sacrificada / cazada
pub const EDAD_MINIMA_SACRIFICIO_CONEJO: u32 = 6; 
pub const EDAD_MINIMA_SACRIFICIO_RATON: u32 = 3;  
pub const EDAD_MINIMA_SACRIFICIO_ARDILLA: u32 = 5; 

pub const EDAD_MINIMA_REPRODUCCION_CONEJO: u32 = 10;
pub const EDAD_MINIMA_REPRODUCCION_RATON: u32 = 6;
pub const EDAD_MINIMA_REPRODUCCION_ARDILLA: u32 = 8;

// Poblaciones máximas 
pub const POBLACION_MAXIMA_CONEJO: usize = 30;
pub const POBLACION_MAXIMA_RATON: usize = 40;
pub const POBLACION_MAXIMA_ARDILLA: usize = 25;

// Probabilidades en el numero de crias 
pub const PROB_CRIAS_CONEJO: [f32; 7] = [0.02, 0.15, 0.40, 0.25, 0.10, 0.06, 0.02]; // 0 a 6 crías
pub const PROB_CRIAS_RATON: [f32; 5] = [0.10, 0.30, 0.40, 0.15, 0.05]; // 0 a 4 crías
pub const PROB_CRIAS_ARDILLA: [f32; 4] = [0.20, 0.50, 0.20, 0.10]; // 0 a 3 crías

pub const RADIO_APARICION_CRIA: f32 = 4.0; // distancia máxima desde la madre donde aparece la cría

pub const RUIDO_MOVIMIENTO: f32 = 0.05; // ruido añadido a la velocidad de las presas

trait Organismo {
    fn actualizar(&mut self);
    fn dibujar(&self);
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn r(&self) -> f32;
    fn especie(&self) -> Especie;
    fn esta_vivo(&self) -> bool;
    fn matar(&mut self);
}

// ==================== SEXO ====================
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sexo {
    Macho,
    Hembra,
}

impl Sexo {
    fn nombre(&self) -> &'static str {
        match self {
            Sexo::Macho => "Macho",
            Sexo::Hembra => "Hembra",
        }
    }
}

// ==================== ESTADO DE SALUD ====================
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EstadoSalud {
    Sano,
    Enfermo,
}

impl EstadoSalud {
    fn nombre(&self) -> &'static str {
        match self {
            EstadoSalud::Sano => "Sano",
            EstadoSalud::Enfermo => "Enfermo",
        }
    }
}



// ==================== ENUM ESPECIE ====================
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Especie {
    Conejo,
    Raton,
    Ardilla,
}

impl Especie {
    fn color(&self) -> Color {
        match self {
            Especie::Conejo => BLUE,
            Especie::Raton => GREEN,
            Especie::Ardilla => ORANGE,
        }
    }

    fn nombre(&self) -> &'static str {
        match self {
            Especie::Conejo => "Conejo",
            Especie::Raton => "Ratón",
            Especie::Ardilla => "Ardilla",
        }
    }

    fn num_crias(especie: Especie, rng: &mut impl Rng) -> usize {
        let probs: &[f32] = match especie {
            Especie::Conejo  => &PROB_CRIAS_CONEJO,
            Especie::Raton   => &PROB_CRIAS_RATON,
            Especie::Ardilla => &PROB_CRIAS_ARDILLA,
        };

        let r: f32 = rng.gen_range(0.0..1.0);
        let mut acumulado = 0.0;
        for (k, &p) in probs.iter().enumerate() {
            acumulado += p;
            if r < acumulado {
                return k;
            }
        }
        probs.len() - 1 // fallback
    }

    // Crea una nueva presa recién nacida
    fn crear_cria(x: f32, y: f32, especie: Especie, _rng: &mut impl Rng) -> Presa {
        let mut p = Presa::new(x, y, especie);
        p.edad = 0;
        p.peso = 0.0;
        p.cooldown = 1.0; // evita que se reproduzca inmediatamente
        p
    }


    // Parámetros de la curva de Gompertz para cada especie
    fn gompertz_params(&self) -> (f32, f32, f32) {
        match self {
            Especie::Conejo => (10.0, 0.2, 5.0),
            Especie::Raton => (4.0, 0.25, 4.0),
            Especie::Ardilla => (7.0, 0.18, 6.0),
        }
    }

    // Probabilidad de que una cría sea macho
    fn probabilidad_macho(&self) -> f32 {
        match self {
            Especie::Conejo => PROB_MACHO_CONEJO,
            Especie::Raton => PROB_MACHO_RATON,
            Especie::Ardilla => PROB_MACHO_ARDILLA,
        }
    }

    fn poblacion_maxima(&self) -> usize {
        match self {
            Especie::Conejo => POBLACION_MAXIMA_CONEJO,
            Especie::Raton => POBLACION_MAXIMA_RATON,
            Especie::Ardilla => POBLACION_MAXIMA_ARDILLA,
        }
    }

    // Edad mínima (en días) a partir de la cual la presa puede ser sacrificada / cazada
    fn edad_sacrificio(&self) -> u32 {
        match self {
            Especie::Conejo => EDAD_MINIMA_SACRIFICIO_CONEJO,  
            Especie::Raton => EDAD_MINIMA_SACRIFICIO_RATON,    
            Especie::Ardilla => EDAD_MINIMA_SACRIFICIO_ARDILLA,
        }
    }

    fn edad_reproduccion(&self) -> u32 {
        match self {
            Especie::Conejo => EDAD_MINIMA_REPRODUCCION_CONEJO,  
            Especie::Raton => EDAD_MINIMA_REPRODUCCION_RATON,    
            Especie::Ardilla => EDAD_MINIMA_REPRODUCCION_ARDILLA,
        }
    }
}

// ==================== PRESA ====================
#[derive(Clone, Debug)]
struct Presa {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
    viva: bool,
    especie: Especie,
    sexo: Sexo,
    cooldown: f32,
    edad: u32,
    peso: f32,
    modo_reproduccion: bool,
    salud: EstadoSalud,
    dias_enfermo: u32,
}

impl Presa {
    fn new(x: f32, y: f32, especie: Especie) -> Self {
        let mut rng = thread_rng();
        let sexo = if rng.gen_range(0.0..1.0) < especie.probabilidad_macho() {
            Sexo::Macho
        } else {
            Sexo::Hembra
        };

        Self {
            x,
            y,
            vx: rng.gen_range(-VEL_MAX_PRESA..VEL_MAX_PRESA),
            vy: rng.gen_range(-VEL_MAX_PRESA..VEL_MAX_PRESA),
            r: RADIO_PRESA,
            viva: true,
            especie,
            sexo,
            cooldown: 1.0,
            edad: 0,
            peso: 0.0,
            modo_reproduccion: false,
            salud: EstadoSalud::Sano,
            dias_enfermo: 0,
        }
    }

    fn peso_actual(&self) -> f32 {
        self.peso
    }

    fn mover_hacia(&mut self, tx: f32, ty: f32) {
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.0 {
            self.vx = dx / dist * VEL_MAX_PRESA;
            self.vy = dy / dist * VEL_MAX_PRESA;
        }
    }
}

impl Organismo for Presa {
    fn actualizar(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x < 0.0 || self.x > screen_width() {
            self.vx *= -1.0;
        }
        if self.y < 0.0 || self.y > screen_height() {
            self.vy *= -1.0;
        }

        if self.cooldown > 0.0 {
            self.cooldown -= get_frame_time();
        }
    }

    fn dibujar(&self) {
        draw_circle(self.x, self.y, self.r(), self.especie.color());
        //Dibujar borde rojo si está enfermo
        if self.salud == EstadoSalud::Enfermo {
            draw_circle_lines(self.x, self.y, self.r() + 2.0, 2.0, RED);
        }
        // Dibujar letra M o H para indicar el sexo
        let label = match self.sexo {
            Sexo::Macho => "M",
            Sexo::Hembra => "H",
        };
        draw_text(label, self.x - 5.0, self.y - 12.0, 16.0, BLACK);
    }

    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn r(&self) -> f32 { self.r }
    fn especie(&self) -> Especie { self.especie }
    fn esta_vivo(&self) -> bool { self.viva }
    fn matar(&mut self) { self.viva = false; }
}

// ==================== DEPREDADOR ====================
struct Depredador {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
    vivo: bool,
    reserva: f32,
    cooldown: f32,
    salud: EstadoSalud,
    dias_enfermo: u32,
}

impl Depredador {
    fn new(x: f32, y: f32) -> Self {
        let mut rng = thread_rng();
        Self {
            x,
            y,
            vx: rng.gen_range(-VEL_MAX_DEPREDADOR..VEL_MAX_DEPREDADOR),
            vy: rng.gen_range(-VEL_MAX_DEPREDADOR..VEL_MAX_DEPREDADOR),
            r: RADIO_DEPREDADOR,
            vivo: true,
            reserva: 0.0,
            cooldown: 0.0,
            salud: EstadoSalud::Sano,
            dias_enfermo: 0,
        }
    }

    fn mover_hacia(&mut self, tx: f32, ty: f32) {
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.0 {
            self.vx = dx / dist * VEL_MAX_DEPREDADOR;
            self.vy = dy / dist * VEL_MAX_DEPREDADOR;
        }
    }
}

impl Organismo for Depredador {
    fn actualizar(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.cooldown > 0.0 {
            self.cooldown -= get_frame_time();
            if self.cooldown < 0.0 {
                self.cooldown = 0.0;
            }
        }

        if self.x < 0.0 || self.x > screen_width() {
            self.vx *= -1.0;
        }
        if self.y < 0.0 || self.y > screen_height() {
            self.vy *= -1.0;
        }
    }

    fn dibujar(&self) {
        draw_circle(self.x, self.y, self.r(), RED);
        if self.salud == EstadoSalud::Enfermo {
            draw_circle_lines(self.x, self.y, self.r() + 2.0, 2.0, BLACK);
        }
        if self.cooldown > 0.0 {
            draw_text(
                &format!("{:.1}", self.cooldown),
                self.x - 10.0,
                self.y - 20.0,
                16.0,
                BLACK,
            );
        }
    }

    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn r(&self) -> f32 { self.r }
    fn especie(&self) -> Especie { Especie::Conejo }
    fn esta_vivo(&self) -> bool { self.vivo }
    fn matar(&mut self) { self.vivo = false; }
}

// ==================== FUNCIONES DE COLISION ====================
fn colision(a: &dyn Organismo, b: &dyn Organismo) -> bool {
    let dx = a.x() - b.x();
    let dy = a.y() - b.y();
    (dx * dx + dy * dy).sqrt() < a.r() + b.r()
}

// ==================== REPORTE DIARIO ====================
#[derive(Clone, Debug, serde::Serialize)]
struct DailyReport {
    dia: u32,
    conteo_conejos: usize,
    conteo_ratones: usize,
    conteo_ardillas: usize,
    conteo_total: usize,
    muertes_por_predacion: u32,
    muertes_por_enfermedad: u32,
    nuevos_infectados: u32,
    recuperaciones: u32,
    reproducciones: u32,
    depredadores_enfermos: usize,
    depredadores_vivos: usize,
}

fn guardar_reportes_csv(reportes: &Vec<DailyReport>, ruta: &str) -> csv::Result<()> {
    let mut wtr = csv::Writer::from_path(ruta)?;
    for rep in reportes {
        wtr.serialize(rep)?; // convierte struct -> fila CSV
    }
    wtr.flush()?;
    Ok(())
}


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

    let mut reportes: Vec<DailyReport> = Vec::new();

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
                p.edad += 1;
                    // Fórmula Gompertz para actualizar peso
                let (a, b, c) = p.especie.gompertz_params();
                p.peso = a * (-b * f32::exp(-c * p.edad as f32)).exp();
                if p.edad >= p.especie.edad_reproduccion(){
                    p.modo_reproduccion = true;
                }
            }


            // --- procesos diarios relacionados con enfermedades en presas ---
            for p in presas.iter_mut() {
                if !p.esta_vivo() { continue; }
                match p.salud {
                    EstadoSalud::Sano => {
                        // posibilidad diaria de enfermar
                        if rng.gen_range(0.0..1.0) < PROB_ENFERMAR_DIARIA_PRESA {
                            p.salud = EstadoSalud::Enfermo;
                            p.dias_enfermo = 0;
                            nuevos_infectados_diarios += 1;
                        }
                    }
                    EstadoSalud::Enfermo => {
                        p.dias_enfermo += 1;
                        // posibilidad de recuperación diaria
                        if rng.gen_range(0.0..1.0) < PROB_RECUPERACION_DIARIA_PRESA {
                            p.salud = EstadoSalud::Sano;
                            p.dias_enfermo = 0;
                            recuperaciones_diarias += 1;
                        } else if p.dias_enfermo >= MAX_DIAS_SIN_RECUPERAR_PRESA {
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
                if d.reserva >= CONSUMO_DIARIO_DEPREDADOR {
                    d.reserva -= CONSUMO_DIARIO_DEPREDADOR;
                } else {
                    // si no tiene suficiente, se queda en 0
                    d.reserva = (d.reserva - CONSUMO_DIARIO_DEPREDADOR).max(0.0);
                }
                    // 🔹 aplicar inmunidad en los primeros días
                if dias <= DIAS_INMUNIDAD {
                    // durante los días de inmunidad no se enferma
                    d.salud = EstadoSalud::Sano;
                    d.dias_enfermo = 0;
                    continue; // saltar al siguiente depredador
                }

                // evaluar estado según umbrales
                if d.reserva >= UMBRAL_OPTIMO_DEPREDADOR {
                    // óptimo: recupera si estaba enfermo
                    if d.salud == EstadoSalud::Enfermo {
                        d.salud = EstadoSalud::Sano;
                        d.dias_enfermo = 0;
                    } else {
                        d.dias_enfermo = 0;
                    }
                } else if d.reserva >= UMBRAL_MINIMO_DEPREDADOR {
                    // mínimo: se mantiene sano, pero no recupera enfermos (por regla dada)
                    d.dias_enfermo = 0;
                    d.salud = EstadoSalud::Sano;
                } else if d.reserva >= UMBRAL_DEFICIENTE_DEPREDADOR {
                    // entre deficiente y mínimo: riesgo; acumula días en estado precario
                    d.dias_enfermo += 1;
                    // no se declara enfermo inmediatamente, pero si se prolonga, pasa a enfermo
                    if d.dias_enfermo > 2 {
                        d.salud = EstadoSalud::Enfermo;
                    }
                    if d.dias_enfermo >= MAX_DIAS_SIN_RECUPERAR_DEPREDADOR {
                        d.matar();
                    }
                } else {
                    // por debajo de deficiente: se enferma y puede morir si se prolonga
                    d.salud = EstadoSalud::Enfermo;
                    d.dias_enfermo += 1;
                    if d.dias_enfermo >= MAX_DIAS_SIN_RECUPERAR_DEPREDADOR {
                        d.matar();
                    }
                }
            }

            // --- compilar y guardar reporte del día anterior (conteos actuales)
            let mut conteo = [0usize, 0usize, 0usize];
            let mut dep_enfermos = 0usize;
            for p in &presas {
                if p.esta_vivo() {
                    match p.especie {
                        Especie::Conejo => conteo[0] += 1,
                        Especie::Raton => conteo[1] += 1,
                        Especie::Ardilla => conteo[2] += 1,
                    }
                }
            }
            for d in &depredadores {
                if d.esta_vivo() && d.salud == EstadoSalud::Enfermo {
                    dep_enfermos += 1;
                }
            }

            reportes.push(DailyReport {
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
            if !presas[i].modo_reproduccion { continue; }
            if presas[i].cooldown > 0.0 { continue; }

            let mut pareja_index: Option<usize> = None;
            let mut dist_min = f32::MAX;

            for j in 0..presas.len() {
                if i == j || !presas[j].esta_vivo() { continue; }
                if presas[i].especie != presas[j].especie { continue; }
                if presas[i].sexo == presas[j].sexo { continue; }
                if !presas[j].modo_reproduccion || presas[j].cooldown > 0.0 { continue; }

                let dx = presas[j].x - presas[i].x;
                let dy = presas[j].y - presas[i].y;
                let dist = (dx*dx + dy*dy).sqrt();

                if dist < dist_min {
                    dist_min = dist;
                    pareja_index = Some(j);
                }
            }

            if let Some(j) = pareja_index {
                let pareja = &presas[j];

                // Vector hacia la pareja + dispersión aleatoria
                let mut dx = pareja.x - presas[i].x + rng.gen_range(-RADIO_PRESA..RADIO_PRESA);
                let mut dy = pareja.y - presas[i].y + rng.gen_range(-RADIO_PRESA..RADIO_PRESA);
                let dist = (dx*dx + dy*dy).sqrt();

                if dist > 0.0 {
                    presas[i].vx = dx / dist * VEL_MAX_PRESA;
                    presas[i].vy = dy / dist * VEL_MAX_PRESA;
                }
            } else {
                // Si no hay pareja, moverse aleatoriamente
                presas[i].vx += rng.gen_range(-RUIDO_MOVIMIENTO..RUIDO_MOVIMIENTO);
                presas[i].vy += rng.gen_range(-RUIDO_MOVIMIENTO..RUIDO_MOVIMIENTO);
                let vel = (presas[i].vx.powi(2) + presas[i].vy.powi(2)).sqrt();
                if vel > VEL_MAX_PRESA {
                    presas[i].vx = presas[i].vx / vel * VEL_MAX_PRESA;
                    presas[i].vy = presas[i].vy / vel * VEL_MAX_PRESA;
                }
            }
        }



        // Depredador busca presa más grande, pero sólo entre las presas que superen la edad de sacrificio
        for d in depredadores.iter_mut() {
            if let Some(obj) = presas.iter()
                .filter(|p| p.esta_vivo() && p.edad >= p.especie.edad_sacrificio())
                .min_by(|a, b| {
                    // Primero comparamos por peso (mayor peso primero)
                    let cmp_peso = b.peso_actual().partial_cmp(&a.peso_actual()).unwrap();
                    if cmp_peso == std::cmp::Ordering::Equal {
                        // Si el peso es igual, comparamos por distancia al depredador (más cercano primero)
                        let dist_a = (a.x() - d.x).powi(2) + (a.y() - d.y).powi(2);
                        let dist_b = (b.x() - d.x).powi(2) + (b.y() - d.y).powi(2);
                        dist_a.partial_cmp(&dist_b).unwrap()
                    } else {
                        cmp_peso
                    }
                })
            {
                d.mover_hacia(obj.x, obj.y);
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
                    && presas[i].cooldown <= 0.0
                    && presas[j].cooldown <= 0.0
                    && presas[i].edad >= presas[i].especie.edad_reproduccion()
                    && presas[j].edad >= presas[j].especie.edad_reproduccion()
                {
                    // asegurar pareja macho/hembra
                    if (presas[i].sexo == Sexo::Macho && presas[j].sexo == Sexo::Hembra)
                        || (presas[i].sexo == Sexo::Hembra && presas[j].sexo == Sexo::Macho)
                    {
                        let especie = presas[i].especie();
                        let count_actual = presas.iter().filter(|p| p.especie == especie).count();
                        let max_pobl = especie.poblacion_maxima();

                        // muestrear cuántas crías nacerán
                        let n_crias = Especie::num_crias(especie, &mut rng2);

                        // cuántas se pueden añadir sin pasar el límite
                        let espacio = if max_pobl > count_actual { max_pobl - count_actual } else { 0 };
                        let n_a_crear = n_crias.min(espacio);
                        if n_a_crear > 0 {
                            reproducciones_diarias += 1;
                        }

                        for _ in 0..n_a_crear {
                            let dx = rng2.gen_range(-RADIO_APARICION_CRIA..RADIO_APARICION_CRIA);
                            let dy = rng2.gen_range(-RADIO_APARICION_CRIA..RADIO_APARICION_CRIA);
                            nuevas_presas.push(Especie::crear_cria(
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
                    let count_actual = presas.iter().filter(|p| p.especie == *especie).count();
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
            presas[i].cooldown = 2.0;
            presas[j].cooldown = 2.0;
        }



        // Depredador come presa (sólo si la presa cumple edad de sacrificio y si el depredador no está en cooldown)
        for d in depredadores.iter_mut() {
            if d.cooldown <= 0.0 {
                for p in presas.iter_mut() {
                    if p.esta_vivo() && p.edad >= p.especie.edad_sacrificio() && colision(d, p) {
                        d.reserva += p.peso_actual();
                        p.matar();
                        muertes_por_predacion_diarias += 1;
                        d.cooldown = TIEMPO_ESPERA_COMIDA; // tiempo de espera entre comidas
                        break;
                    }
                }
            }
        }

        presas.retain(|p| p.esta_vivo());

        // Interfaz
        let mut conteo = [0, 0, 0];
        let mut suma_edades = [0u32, 0, 0];
        let mut suma_pesos = [0.0f32, 0.0, 0.0];

        for p in &presas {
            match p.especie {
                Especie::Conejo => { conteo[0] += 1; suma_edades[0] += p.edad; suma_pesos[0] += p.peso; }
                Especie::Raton => { conteo[1] += 1; suma_edades[1] += p.edad; suma_pesos[1] += p.peso; }
                Especie::Ardilla => { conteo[2] += 1; suma_edades[2] += p.edad; suma_pesos[2] += p.peso;}
            }
        }


        let promedio = |suma: u32, count: i32| -> f32 {
            if count > 0 { suma as f32 / count as f32 } else { 0.0 }
        };

        let promedio_peso = |suma: f32, count: i32| -> f32 {
            if count > 0 { suma / count as f32 } else { 0.0 }
        };

        draw_text(&format!("Día: {} | Esc:para finalizar y generar reporte", dias), 10.0, 20.0, 20.0, BLACK);
        draw_text(
            &format!("Conejos: {} (edad promedio: {:.1}, peso promedio: {:.1})", conteo[0], promedio(suma_edades[0], conteo[0]),promedio_peso(suma_pesos[0], conteo[0])),
            10.0, 50.0, 20.0, Especie::Conejo.color(),
        );
        draw_text(
            &format!("Ratones: {} (edad promedio: {:.1}, peso promedio: {:.1})", conteo[1], promedio(suma_edades[1], conteo[1]),promedio_peso(suma_pesos[1], conteo[1])),
            10.0, 70.0, 20.0, Especie::Raton.color(),
        );
        draw_text(
            &format!("Ardillas: {} (edad promedio: {:.1}, peso promedio: {:.1})", conteo[2], promedio(suma_edades[2], conteo[2]),promedio_peso(suma_pesos[2], conteo[2])),
            10.0, 90.0, 20.0, Especie::Ardilla.color(),
        );

        for (i, d) in depredadores.iter().enumerate() {
            draw_text(
                &format!("Depredador {} peso: {:.1} estado: {}", i + 1, d.reserva, d.salud.nombre()),
                10.0, 130.0 + i as f32 * 20.0, 20.0, RED,
            );
        }

        if is_key_pressed(KeyCode::Escape) {
            if let Err(e) = guardar_reportes_csv(&reportes, "reportes.csv") {
                eprintln!("Error guardando CSV: {}", e);
            } else {
                println!("Reportes guardados en reportes.csv");
            }
            break; 
        }

        next_frame().await;
    }
}