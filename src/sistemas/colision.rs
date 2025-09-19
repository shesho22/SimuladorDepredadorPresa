use crate::entidades::Organismo;

pub fn colision(a: &dyn Organismo, b: &dyn Organismo) -> bool {
    let dx = a.x() - b.x();
    let dy = a.y() - b.y();
    (dx * dx + dy * dy).sqrt() < a.r() + b.r()
}