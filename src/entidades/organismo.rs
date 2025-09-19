use crate::entidades::Especie;

pub trait Organismo {
    fn actualizar(&mut self);
    fn dibujar(&self);
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn r(&self) -> f32;
    fn especie(&self) -> Especie;
    fn esta_vivo(&self) -> bool;
    fn matar(&mut self);
}
