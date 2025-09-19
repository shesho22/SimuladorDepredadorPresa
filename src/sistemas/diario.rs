use ::rand::Rng;
use crate::entidades::organismo::Organismo;
use crate::entidades::{Presa, Depredador, EstadoSalud};
use crate::utilidades::configuraciones::*;

pub fn resetear_contadores_diarios(muertes_pred: &mut u32, muertes_enf: &mut u32,
                               nuevos_infectados: &mut u32, recuperaciones: &mut u32) {
    *muertes_pred = 0;
    *muertes_enf = 0;
    *nuevos_infectados = 0;
    *recuperaciones = 0;
}

pub fn actualizar_presas_diarias(presas: &mut Vec<Presa>) {
    for p in presas.iter_mut() {
        p.set_edad(p.edad() + 1);
        let (a, b, c) = p.especie().gompertz_params();
        p.set_peso(a * (-b * f32::exp(-c * p.edad() as f32)).exp());
        if p.edad() >= p.especie().edad_reproduccion() {
            p.set_modo_reproduccion(true);
        }
    }
}


pub fn procesar_enfermedad_presas(presas: &mut Vec<Presa>, rng: &mut impl Rng,
                               nuevos_infectados: &mut u32, recuperaciones: &mut u32,
                               muertes: &mut u32) {
    for p in presas.iter_mut() {
        if !p.esta_vivo() { continue; }
        match p.salud() {
            EstadoSalud::Sano => {
                if rng.gen_range(0.0..1.0) < PROB_ENFERMAR_DIARIA_PRESA {
                    p.set_salud(EstadoSalud::Enfermo);
                    p.reset_dias_enfermo();
                    *nuevos_infectados += 1;
                }
            }
            EstadoSalud::Enfermo => {
                p.incrementar_dias_enfermo();
                if rng.gen_range(0.0..1.0) < PROB_RECUPERACION_DIARIA_PRESA {
                    p.set_salud(EstadoSalud::Sano);
                    p.reset_dias_enfermo();
                    *recuperaciones += 1;
                } else if p.dias_enfermo() >= MAX_DIAS_SIN_RECUPERAR_PRESA {
                    p.matar();
                    *muertes += 1;
                }
            }
        }
    }
}

pub fn procesar_dietas_depredadores(depredadores: &mut Vec<Depredador>, dias: u32) {
    for d in depredadores.iter_mut() {
        if !d.esta_vivo() { continue; }

        // Consumo diario
        if d.reserva() >= CONSUMO_DIARIO_DEPREDADOR {
            d.set_reserva(d.reserva() - CONSUMO_DIARIO_DEPREDADOR);
        } else {
            d.set_reserva((d.reserva() - CONSUMO_DIARIO_DEPREDADOR).max(0.0));
        }

        if dias <= DIAS_INMUNIDAD {
            d.set_salud(EstadoSalud::Sano);
            d.reset_dias_enfermo();
            continue;
        }

        if d.reserva() >= UMBRAL_OPTIMO_DEPREDADOR {
            if d.salud() == EstadoSalud::Enfermo {
                d.set_salud(EstadoSalud::Sano);
                d.reset_dias_enfermo();
            } else {
                d.reset_dias_enfermo();
            }
        } else if d.reserva() >= UMBRAL_MINIMO_DEPREDADOR {
            d.reset_dias_enfermo();
            d.set_salud(EstadoSalud::Sano);
        } else if d.reserva() >= UMBRAL_DEFICIENTE_DEPREDADOR {
            d.incrementar_dias_enfermo();
            if d.dias_enfermo() > 2 {
                d.set_salud(EstadoSalud::Enfermo);
            }
            if d.dias_enfermo() >= MAX_DIAS_SIN_RECUPERAR_DEPREDADOR {
                d.matar();
            }
        } else {
            d.set_salud(EstadoSalud::Enfermo);
            d.incrementar_dias_enfermo();
            if d.dias_enfermo() >= MAX_DIAS_SIN_RECUPERAR_DEPREDADOR {
                d.matar();
            }
        }
    }
}