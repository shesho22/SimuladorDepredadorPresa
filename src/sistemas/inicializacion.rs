use ::rand::Rng;
use crate::entidades::{Presa, Depredador,Especie};
use macroquad::prelude::*;


pub fn inicializar_presas(especies: &[Especie], cantidad: usize, rng: &mut impl Rng) -> Vec<Presa> {
    (0..cantidad).map(|_| {
        let especie = especies[rng.gen_range(0..especies.len())];
        Presa::new(
            rng.gen_range(0.0..screen_width()),
            rng.gen_range(0.0..screen_height()),
            especie,
        )
    }).collect()
}

pub fn inicializar_depredadores(cantidad: usize, rng: &mut impl Rng) -> Vec<Depredador> {
    (0..cantidad).map(|_| Depredador::new(
        rng.gen_range(0.0..screen_width()),
        rng.gen_range(0.0..screen_height())
    )).collect()
}