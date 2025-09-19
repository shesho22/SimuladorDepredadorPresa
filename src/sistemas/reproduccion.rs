use ::rand::Rng;
use ::rand::thread_rng;
use crate::entidades::{Presa, Especie};
use crate::utilidades::configuraciones::*;
use crate::sistemas::colision::colision;

pub fn reproduccion(presas: &mut Vec<Presa>, especies: &[Especie], reproducciones_diarias: &mut u32) {
    let mut nuevas_presas: Vec<Presa> = Vec::new();
    let mut rng2 = thread_rng();
    let mut parejas_repro: Vec<(usize, usize, Especie)> = Vec::new();

    for i in 0..presas.len() {
        for j in (i + 1)..presas.len() {
            if !presas[i].esta_vivo() || !presas[j].esta_vivo() { continue; }
            if presas[i].especie() != presas[j].especie() { continue; }
            if presas[i].cooldown() > 0.0 || presas[j].cooldown() > 0.0 { continue; }
            if presas[i].edad() < presas[i].especie().edad_reproduccion() ||
               presas[j].edad() < presas[j].especie().edad_reproduccion() { continue; }
            if (presas[i].sexo() == presas[j].sexo()) { continue; }
            if !colision(&presas[i], &presas[j]) { continue; }

            let especie = presas[i].especie();
            let count_actual = presas.iter().filter(|p| p.especie() == especie).count();
            let max_pobl = especie.poblacion_maxima();
            let n_crias = Presa::num_crias(especie, &mut rng2);
            let espacio = max_pobl.saturating_sub(count_actual);
            let n_a_crear = n_crias.min(espacio);

            for _ in 0..n_a_crear {
                let dx = rng2.gen_range(-RADIO_APARICION_CRIA..RADIO_APARICION_CRIA);
                let dy = rng2.gen_range(-RADIO_APARICION_CRIA..RADIO_APARICION_CRIA);
                nuevas_presas.push(Presa::crear_cria(
                    presas[i].x() + dx,
                    presas[i].y() + dy,
                    especie,
                    &mut rng2
                ));
            }

            if n_a_crear > 0 { *reproducciones_diarias += 1; }
            parejas_repro.push((i, j, especie));
        }
    }

    presas.extend(nuevas_presas);

    for (i, j, _) in parejas_repro {
        presas[i].set_cooldown(2.0);
        presas[j].set_cooldown(2.0);
    }
}