use macroquad::prelude::*;
use ::rand::Rng;

use crate::{
    PROB_CRIAS_CONEJO, PROB_CRIAS_RATON, PROB_CRIAS_ARDILLA,
    PROB_MACHO_CONEJO, PROB_MACHO_RATON, PROB_MACHO_ARDILLA,
    POBLACION_MAXIMA_CONEJO, POBLACION_MAXIMA_RATON, POBLACION_MAXIMA_ARDILLA,
    EDAD_MINIMA_SACRIFICIO_CONEJO, EDAD_MINIMA_SACRIFICIO_RATON, EDAD_MINIMA_SACRIFICIO_ARDILLA,
    EDAD_MINIMA_REPRODUCCION_CONEJO, EDAD_MINIMA_REPRODUCCION_RATON, EDAD_MINIMA_REPRODUCCION_ARDILLA,
    RADIO_APARICION_CRIA,
};

use super::presa::Presa;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sexo { Macho, Hembra }

impl Sexo {
    pub fn nombre(&self) -> &'static str {
        match self {
            Sexo::Macho => "Macho",
            Sexo::Hembra => "Hembra",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EstadoSalud { Sano, Enfermo }

impl EstadoSalud {
    pub fn nombre(&self) -> &'static str {
        match self {
            EstadoSalud::Sano => "Sano",
            EstadoSalud::Enfermo => "Enfermo",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Especie { Conejo, Raton, Ardilla }

impl Especie {
    pub fn color(&self) -> Color {
        match self {
            Especie::Conejo => BLUE,
            Especie::Raton => GREEN,
            Especie::Ardilla => ORANGE,
        }
    }

    pub fn nombre(&self) -> &'static str {
        match self {
            Especie::Conejo => "Conejo",
            Especie::Raton => "Ratón",
            Especie::Ardilla => "Ardilla",
        }
    }


    pub fn gompertz_params(&self) -> (f32, f32, f32) {
        match self {
            Especie::Conejo => (10.0, 0.2, 5.0),
            Especie::Raton => (4.0, 0.25, 4.0),
            Especie::Ardilla => (7.0, 0.18, 6.0),
        }
    }

    pub fn probabilidad_macho(&self) -> f32 {
        match self {
            Especie::Conejo => PROB_MACHO_CONEJO,
            Especie::Raton => PROB_MACHO_RATON,
            Especie::Ardilla => PROB_MACHO_ARDILLA,
        }
    }

    pub fn poblacion_maxima(&self) -> usize {
        match self {
            Especie::Conejo => POBLACION_MAXIMA_CONEJO,
            Especie::Raton => POBLACION_MAXIMA_RATON,
            Especie::Ardilla => POBLACION_MAXIMA_ARDILLA,
        }
    }

    pub fn edad_sacrificio(&self) -> u32 {
        match self {
            Especie::Conejo => EDAD_MINIMA_SACRIFICIO_CONEJO,
            Especie::Raton => EDAD_MINIMA_SACRIFICIO_RATON,
            Especie::Ardilla => EDAD_MINIMA_SACRIFICIO_ARDILLA,
        }
    }

    pub fn edad_reproduccion(&self) -> u32 {
        match self {
            Especie::Conejo => EDAD_MINIMA_REPRODUCCION_CONEJO,
            Especie::Raton => EDAD_MINIMA_REPRODUCCION_RATON,
            Especie::Ardilla => EDAD_MINIMA_REPRODUCCION_ARDILLA,
        }
    }
}
