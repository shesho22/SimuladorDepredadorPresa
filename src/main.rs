use ::rand::Rng;
use ::rand::thread_rng;
use macroquad::prelude::*;

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
            Especie::Conejo => 0.55,   // 55% probabilidad macho
            Especie::Raton => 0.50,    // 50/50
            Especie::Ardilla => 0.45,  // 45% probabilidad macho
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
        }
    }

    fn peso_actual(&self) -> f32 {
        let (peso_max, tasa, despl) = self.especie.gompertz_params();
        let edad_f = self.edad as f32;
        peso_max * (-f32::exp(-tasa * (edad_f - despl))).exp()
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

        // opcional: mostrar sexo encima de la presa
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
    peso: f32,
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
            peso: 0.0,
        }
    }
}

impl Organismo for Depredador {
    fn actualizar(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.x < 0.0 || self.x > screen_width() {
            self.vx *= -1.0;
        }
        if self.y < 0.0 || self.y > screen_height() {
            self.vy *= -1.0;
        }
    }

    fn dibujar(&self) {
        draw_circle(self.x, self.y, self.r(), RED);
    }

    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn r(&self) -> f32 {
        12.0
    }
    fn especie(&self) -> Especie {
        Especie::Conejo // placeholder sin uso real
    }
    fn esta_vivo(&self) -> bool {
        self.vivo
    }
    fn matar(&mut self) {
        self.vivo = false;
    }
}

// ==================== FUNCIONES DE COLISION ====================
fn colision(a: &dyn Organismo, b: &dyn Organismo) -> bool {
    let dx = a.x() - b.x();
    let dy = a.y() - b.y();
    (dx * dx + dy * dy).sqrt() < a.r() + b.r()
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
        .map(|_| Depredador::new(rng.gen_range(0.0..screen_width()), rng.gen_range(0.0..screen_height())))
        .collect();

    let mut dias: u32 = 0;
    let mut tiempo_acumulado: f32 = 0.0;

    loop {
        clear_background(LIGHTGRAY);

        // Actualizar tiempo -> contar días
        tiempo_acumulado += get_frame_time();
        if tiempo_acumulado >= 1.0 {
            dias += 1;
            tiempo_acumulado = 0.0;
            for p in presas.iter_mut() {
                p.edad += 1;
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

        // Reproducción entre presas (con cooldown y sexo)
        let mut nuevas_presas = vec![];
        for i in 0..presas.len() {
            for j in i + 1..presas.len() {
                if presas[i].esta_vivo() && presas[j].esta_vivo() {
                    if presas[i].especie() == presas[j].especie()
                        && colision(&presas[i], &presas[j])
                        && presas[i].cooldown <= 0.0
                        && presas[j].cooldown <= 0.0
                    {
                        // Debe ser macho + hembra
                        if (presas[i].sexo == Sexo::Macho && presas[j].sexo == Sexo::Hembra)
                            || (presas[i].sexo == Sexo::Hembra && presas[j].sexo == Sexo::Macho)
                        {
                            nuevas_presas.push(Presa::new(presas[i].x(), presas[i].y(), presas[i].especie()));
                            presas[i].cooldown = 2.0;
                            presas[j].cooldown = 2.0;
                        }
                    }
                }
            }
        }
        presas.extend(nuevas_presas);

        // Depredador come presa (aumenta peso con la curva de Gompertz)
        for d in depredadores.iter_mut() {
            for p in presas.iter_mut() {
                if p.esta_vivo() && colision(d, p) {
                    d.peso += p.peso_actual();
                    p.matar();
                }
            }
        }

        // Eliminar presas muertas
        presas.retain(|p| p.esta_vivo());

        // ====== Interfaz con conteo y edad promedio ======
        let mut conteo = [0, 0, 0];
        let mut suma_edades = [0u32, 0, 0];

        for p in &presas {
            match p.especie {
                Especie::Conejo => {
                    conteo[0] += 1;
                    suma_edades[0] += p.edad;
                }
                Especie::Raton => {
                    conteo[1] += 1;
                    suma_edades[1] += p.edad;
                }
                Especie::Ardilla => {
                    conteo[2] += 1;
                    suma_edades[2] += p.edad;
                }
            }
        }

        let promedio = |suma: u32, count: i32| -> f32 {
            if count > 0 {
                suma as f32 / count as f32
            } else {
                0.0
            }
        };

        draw_text(&format!("Día: {}", dias), 10.0, 20.0, 20.0, BLACK);
        draw_text(
            &format!("Conejos: {} (edad promedio: {:.1})", conteo[0], promedio(suma_edades[0], conteo[0])),
            10.0,
            50.0,
            20.0,
            Especie::Conejo.color(),
        );
        draw_text(
            &format!("Ratones: {} (edad promedio: {:.1})", conteo[1], promedio(suma_edades[1], conteo[1])),
            10.0,
            70.0,
            20.0,
            Especie::Raton.color(),
        );
        draw_text(
            &format!("Ardillas: {} (edad promedio: {:.1})", conteo[2], promedio(suma_edades[2], conteo[2])),
            10.0,
            90.0,
            20.0,
            Especie::Ardilla.color(),
        );

        for (i, d) in depredadores.iter().enumerate() {
            draw_text(&format!("Depredador {} peso: {:.1}", i + 1, d.peso), 10.0, 130.0 + i as f32 * 20.0, 20.0, RED);
        }

        next_frame().await;
    }
}
