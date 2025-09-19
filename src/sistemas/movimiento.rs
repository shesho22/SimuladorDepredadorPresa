use ::rand::Rng;
use crate::entidades::organismo::Organismo;
use crate::entidades::{Presa, Depredador};
use crate::utilidades::configuraciones::*;


// Busca pareja si esta en modo_reproduccuin o se mueve aleatoriamente
pub fn movimiento_presas(presas: &mut Vec<Presa>, rng: &mut impl Rng) {
    for i in 0..presas.len() {
        let p = &presas[i];
        if !p.esta_vivo() || !p.modo_reproduccion() || p.cooldown() > 0.0 {
            continue;
        }

        // Buscar pareja más cercana
        let mut pareja_index: Option<usize> = None;
        let mut dist_min = f32::MAX;

        for j in 0..presas.len() {
            if i == j { continue; }
            let q = &presas[j];
            if !q.esta_vivo() || q.especie() != p.especie() || q.sexo() == p.sexo() {
                continue;
            }
            if !q.modo_reproduccion() || q.cooldown() > 0.0 { continue; }

            let dx = q.x() - p.x();
            let dy = q.y() - p.y();
            let dist = (dx*dx + dy*dy).sqrt();

            if dist < dist_min {
                dist_min = dist;
                pareja_index = Some(j);
            }
        }

        if let Some(j) = pareja_index {
            let pareja = &presas[j];
            let tx = pareja.x() + rng.gen_range(-RADIO_PRESA..RADIO_PRESA);
            let ty = pareja.y() + rng.gen_range(-RADIO_PRESA..RADIO_PRESA);
            presas[i].mover_hacia(tx, ty);
        } else {
            mover_aleatoriamente(&mut presas[i], rng);
        }
    }
}

pub fn mover_aleatoriamente(p: &mut Presa, rng: &mut impl Rng) {
    let mut vx = p.vx() + rng.gen_range(-RUIDO_MOVIMIENTO..RUIDO_MOVIMIENTO);
    let mut vy = p.vy() + rng.gen_range(-RUIDO_MOVIMIENTO..RUIDO_MOVIMIENTO);
    let vel = (vx.powi(2) + vy.powi(2)).sqrt();
    if vel > VEL_MAX_PRESA {
        vx = vx / vel * VEL_MAX_PRESA;
        vy = vy / vel * VEL_MAX_PRESA;
    }
    p.set_vx(vx);
    p.set_vy(vy);
}


// Busca presas que pasen la edad de sacrificio y la que sea mas pesada
pub fn depredadores_buscar_presas(depredadores: &mut Vec<Depredador>, presas: &[Presa]) {
    for d in depredadores.iter_mut() {
        if let Some(obj) = presas.iter()
            .filter(|p| p.esta_vivo() && p.edad() >= p.especie().edad_sacrificio())
            .min_by(|a, b| {
                let cmp_peso = b.peso_actual().partial_cmp(&a.peso_actual()).unwrap();
                if cmp_peso == std::cmp::Ordering::Equal {
                    let dist_a = (a.x() - d.x()).powi(2) + (a.y() - d.y()).powi(2);
                    let dist_b = (b.x() - d.x()).powi(2) + (b.y() - d.y()).powi(2);
                    dist_a.partial_cmp(&dist_b).unwrap()
                } else {
                    cmp_peso
                }
            })
        {
            d.mover_hacia(obj.x(), obj.y());
        }
    }
}