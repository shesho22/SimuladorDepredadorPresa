use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;

use crate::entidades::{Organismo, Especie, EstadoSalud};
use crate::{RADIO_DEPREDADOR, VEL_MAX_DEPREDADOR,
};

// ==================== DEPREDADOR ====================
#[derive(Clone, Debug)]
pub struct Depredador {
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
    pub fn new(x: f32, y: f32) -> Self {
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

    pub fn mover_hacia(&mut self, tx: f32, ty: f32) {
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > 0.0 {
            self.vx = dx / dist * VEL_MAX_DEPREDADOR;
            self.vy = dy / dist * VEL_MAX_DEPREDADOR;
        }
    }
    // Getters
    pub fn reserva(&self) -> f32 { self.reserva } 
    pub fn salud(&self) -> EstadoSalud { self.salud }
    pub fn cooldown(&self) -> f32 { self.cooldown }
    pub fn dias_enfermo(&self) -> u32 {self.dias_enfermo}
    // Setters
    pub fn set_reserva(&mut self, r: f32) { self.reserva = r;}
    pub fn set_salud(&mut self, s: EstadoSalud) { self.salud = s;}
    pub fn set_cooldown(&mut self, c: f32) { self.cooldown = c; }
    pub fn set_dias_enfermo(&mut self, d: u32){self.dias_enfermo = d;}
    // Otros
    pub fn incrementar_dias_enfermo(&mut self) {self.dias_enfermo += 1;}
    pub fn reset_dias_enfermo(&mut self) {self.dias_enfermo = 0;}


}

impl Organismo for Depredador {
    // Movimiento, cooldown y rebote en pantalla 
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