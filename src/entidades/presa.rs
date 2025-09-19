use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;

use crate::entidades::{Organismo, Especie, EstadoSalud, Sexo};
use crate::{RADIO_PRESA, VEL_MAX_PRESA,PROB_CRIAS_CONEJO,PROB_CRIAS_ARDILLA,PROB_CRIAS_RATON
};

// ==================== PRESA ====================
#[derive(Clone, Debug)]
pub struct Presa {
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
    pub fn new(x: f32, y: f32, especie: Especie) -> Self {
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

    pub fn peso_actual(&self) -> f32 {
        self.peso
    }

    pub fn mover_hacia(&mut self, tx: f32, ty: f32) {
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.0 {
            self.vx = dx / dist * VEL_MAX_PRESA;
            self.vy = dy / dist * VEL_MAX_PRESA;
        }
    }
    // Getters
    pub fn edad(&self) -> u32 { self.edad }
    pub fn peso(&self) -> f32 { self.peso }
    pub fn sexo(&self) -> Sexo { self.sexo }
    pub fn salud(&self) -> EstadoSalud { self.salud }
    pub fn especie(&self) -> Especie { self.especie }
    pub fn modo_reproduccion(&self) -> bool { self.modo_reproduccion }
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn vx(&self) -> f32 { self.vx }
    pub fn vy(&self) -> f32 { self.vy }
    pub fn dias_enfermo(&self) -> u32 {self.dias_enfermo}
    pub fn cooldown(&self) -> f32 {self.cooldown}

    // Setters
    pub fn set_edad(&mut self, edad: u32) { self.edad = edad; }
    pub fn set_peso(&mut self, peso: f32) { self.peso = peso; }
    pub fn set_modo_reproduccion(&mut self, modo: bool) { self.modo_reproduccion = modo; }
    pub fn set_vx(&mut self, vx: f32) { self.vx = vx; }
    pub fn set_vy(&mut self, vy: f32) { self.vy = vy; }
    pub fn set_salud(&mut self, estado: EstadoSalud) {self.salud = estado;}
    pub fn set_dias_enfermo(&mut self, d: u32){self.dias_enfermo = d;}
    pub fn set_cooldown(&mut self, valor: f32) {self.cooldown = valor;}


    // Método para matar la presa
    pub fn matar(&mut self) { self.viva = false; }
    pub fn esta_vivo(&self) -> bool { self.viva } 
    pub fn incrementar_dias_enfermo(&mut self) {self.dias_enfermo += 1;}
    pub fn reset_dias_enfermo(&mut self) {self.dias_enfermo = 0;}

    pub fn num_crias(especie: Especie, rng: &mut impl Rng) -> usize {
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
        probs.len() - 1
    }

    pub fn crear_cria(x: f32, y: f32, especie: Especie, rng: &mut impl Rng) -> Presa {
        let mut p = Presa::new(x, y, especie);
        p.edad = 0;
        p.peso = 0.0;
        p.cooldown = 1.0;
        p
    }
}


impl Organismo for Presa {
    // Movimiento, cooldown y rebote en pantalla 
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

