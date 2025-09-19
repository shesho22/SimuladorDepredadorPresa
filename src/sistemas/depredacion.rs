use crate::entidades::{Presa, Depredador};
use crate::utilidades::configuraciones::*;
use crate::sistemas::colision::colision;

pub fn depredadores_comer(depredadores: &mut Vec<Depredador>, presas: &mut Vec<Presa>, muertes_pred: &mut u32) {
    for d in depredadores.iter_mut() {
        if d.cooldown() <= 0.0 {
            for p in presas.iter_mut() {
                if p.esta_vivo() && p.edad() >= p.especie().edad_sacrificio() && colision(d, p) {
                    d.set_reserva(d.reserva() + p.peso_actual());
                    p.matar();
                    *muertes_pred += 1;
                    d.set_cooldown(TIEMPO_ESPERA_COMIDA);
                    break;
                }
            }
        }
    }
}