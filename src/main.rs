use ::rand::Rng;
use ::rand::thread_rng;
use macroquad::prelude::*;

// ==================== CONFIGURACIONES ====================
const PROB_INFECCION_DIARIA_PRESA: f32 = 0.02; // probabilidad diaria de enfermar (presas)
const PROB_RECUPERACION_DIARIA_PRESA: f32 = 0.30; // probabilidad diaria de recuperarse (presas)

const MAX_DIAS_SIN_RECUPERAR_PRESA: u32 = 5; // si pasa esto, la presa muere

const CONSUMO_DIARIO_DEPREDADOR: f32 = 2.5; // gasto diario de reservas del depredador
const UMBRAL_OPTIMO_DEPREDADOR: f32 = 30.0; // depredador umbral óptimo de reservas
const UMBRAL_MINIMO_DEPREDADOR: f32 = 12.0; // depredador umbral mínimo de reservas
const UMBRAL_DEFICIENTE_DEPREDADOR: f32 = 3.0; // depredador umbral deficiente de reservas
const MAX_DIAS_SIN_RECUPERAR_DEPREDADOR: u32 = 7;
const DIAS_INMUNIDAD: u32 = 7; // número de días protegidos al inicio


const TIEMPO_ESPERA_COMIDA:f32 = 1.0; // depredador tiempo de espera entre comidas


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

// ==================== TRAIT ORGANISMO ====================
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
            Especie::Conejo => 0.55,
            Especie::Raton => 0.50,
            Especie::Ardilla => 0.45,
        }
    }

    fn poblacion_maxima(&self) -> usize {
        match self {
            Especie::Conejo => 30,
            Especie::Raton => 40,
            Especie::Ardilla => 25,
        }
    }

    // Edad mínima (en días) a partir de la cual la presa puede ser sacrificada / cazada
    fn edad_sacrificio(&self) -> u32 {
        match self {
            Especie::Conejo => 6,   // por ejemplo, 6 días
            Especie::Raton => 3,    // 3 días
            Especie::Ardilla => 5,  // 5 días
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
            vx: rng.gen_range(-2.0..2.0),
            vy: rng.gen_range(-2.0..2.0),
            viva: true,
            especie,
            sexo,
            cooldown: 0.0,
            edad: 0,
            peso: 0.0,
            modo_reproduccion: true,
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
            self.vx = dx / dist * 1.5;
            self.vy = dy / dist * 1.5;
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
    fn r(&self) -> f32 { 8.0 }
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
            vx: rng.gen_range(-3.0..3.0),
            vy: rng.gen_range(-3.0..3.0),
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
            self.vx = dx / dist * 2.0;
            self.vy = dy / dist * 2.0;
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
    fn r(&self) -> f32 { 12.0 }
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

    let mut presas: Vec<Presa> = (0..20)
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
            }


            // --- procesos diarios relacionados con enfermedades en presas ---
            for p in presas.iter_mut() {
                if !p.esta_vivo() { continue; }
                match p.salud {
                    EstadoSalud::Sano => {
                        // posibilidad diaria de enfermar
                        if rng.gen_range(0.0..1.0) < PROB_INFECCION_DIARIA_PRESA {
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
                depredadores_enfermos: dep_enfermos,
                depredadores_vivos: depredadores.iter().filter(|d| d.esta_vivo()).count(),

            });

        }
        // ==================== Movimiento inteligente y modo cuando se alcanza máxima población ====================
        for especie in &especies {
            let count = presas.iter().filter(|p| p.especie == *especie).count();
            let max = especie.poblacion_maxima();
            for i in 0..presas.len() {
                if presas[i].especie == *especie {
                    // Si se alcanzó o excedió la población máxima, vuelven al comportamiento "rebotar en paredes"
                    presas[i].modo_reproduccion = count < max;
                    if !presas[i].modo_reproduccion {
                        // si modo_reproduccion == false, restaurar velocidades aleatorias si están a 0
                        if presas[i].vx.abs() < 0.01 && presas[i].vy.abs() < 0.01 {
                            presas[i].vx = rng.gen_range(-2.0..2.0);
                            presas[i].vy = rng.gen_range(-2.0..2.0);
                        }
                    }
                }
            }
        }

        // Presas buscan pareja sólo si están en modo_reproduccion
        let mut pareja_targets: Vec<Option<(f32, f32)>> = Vec::with_capacity(presas.len());
        for i in 0..presas.len() {
            if presas[i].modo_reproduccion && presas[i].cooldown <= 0.0 {
                let target = presas.iter()
                    .enumerate()
                    .filter(|(j, q)| *j != i
                        && q.especie == presas[i].especie
                        && q.sexo != presas[i].sexo
                        && q.cooldown <= 0.0
                        && q.esta_vivo())
                    .map(|(_, q)| ((q.x - presas[i].x).powi(2) + (q.y - presas[i].y).powi(2), q.x, q.y))
                    .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                    .map(|(_, x, y)| (x, y));
                pareja_targets.push(target);
            } else {
                pareja_targets.push(None);
            }
        }
        for (i, target) in pareja_targets.into_iter().enumerate() {
            if let Some((tx, ty)) = target {
                presas[i].mover_hacia(tx, ty);
            }
        }

        // Depredador busca presa más grande, pero sólo entre las presas que superen la edad de sacrificio
        for d in depredadores.iter_mut() {
            if let Some(obj) = presas.iter()
                .filter(|p| p.esta_vivo() && p.edad >= p.especie.edad_sacrificio())
                .max_by(|a, b| a.peso_actual().partial_cmp(&b.peso_actual()).unwrap())
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
        let mut nuevas_presas = vec![];
        for i in 0..presas.len() {
            for j in i + 1..presas.len() {
                if presas[i].esta_vivo() && presas[j].esta_vivo() {
                    if presas[i].especie() == presas[j].especie()
                        && colision(&presas[i], &presas[j])
                        && presas[i].cooldown <= 0.0
                        && presas[j].cooldown <= 0.0
                    {
                        if (presas[i].sexo == Sexo::Macho && presas[j].sexo == Sexo::Hembra)
                            || (presas[i].sexo == Sexo::Hembra && presas[j].sexo == Sexo::Macho)
                        {
                            let count = presas.iter().filter(|p| p.especie == presas[i].especie).count();
                            if count < presas[i].especie.poblacion_maxima() {
                                nuevas_presas.push(Presa::new(
                                    presas[i].x(),
                                    presas[i].y(),
                                    presas[i].especie(),
                                ));
                                recuperaciones_diarias += 1;
                            }
                            presas[i].cooldown = 2.0;
                            presas[j].cooldown = 2.0;
                        }
                    }
                }
            }
        }
        presas.extend(nuevas_presas);

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