use macroquad::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};


// Criterios faltantes 
// -----------------------------
// Presas
// -Curva de crecimiento de Gompertz
// -Probabilidades de nacimiento, enfermedad, número de crías, sexo
// -Acceso a comida ilimitada: la energía solo se incrementa cuando el depredador come.
// -----------------------------
// Depredadore
// -Reserva de comida: Actualmente solo se incrementa energía al comer, no hay “reserva” separada ni niveles mínimos/óptimos de alimentación.
// -Estrategia de caza completa: Solo se caza al azar o por colisión, no se implementa “solo presas con edad de sacrificio” ni se prioriza la más pesada.
// -----------------------------
// General
// -No hay verificación de equilibrio poblacional, ni control de enfermedad o muerte por falta de comida.
// -----------------------------
// Diseño
// - Traits para polimorfismo
// -----------------------------
// Visualización
// -No hay indicadores de energía o reservas de los depredadores.
// -No hay información sobre reproducción, muertes ni alertas de población limitada.
// -No se distinguen visualmente los depredadores de las presas aparte del color; no se marca la interacción (ataques o reproducción).



// -----------------------------
// Enum de especies
// -----------------------------
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Especie {
    Conejo,
    ZorroPequeño,
    Ratón,
    Lobo,
}

// -----------------------------
// Struct con atributos de especie
// -----------------------------
struct EspecieInfo {
    tamaño: f32,
    velocidad: f32,
    color: Color,
    energia: f32,
    es_presa: bool,
    limite: usize,
    edad_reproductiva: u32, // edad mínima en días
}

// -----------------------------
// Obtener atributos de cada especie
// -----------------------------
fn atributos_especie(especie: Especie) -> EspecieInfo {
    match especie {
        Especie::Conejo => EspecieInfo { tamaño: 25.0, velocidad: 2.0, color: RED, energia: 100.0, es_presa: true, limite: 20, edad_reproductiva: 5 },
        Especie::ZorroPequeño => EspecieInfo { tamaño: 20.0, velocidad: 3.0, color: ORANGE, energia: 100.0, es_presa: true, limite: 15, edad_reproductiva: 8 },
        Especie::Ratón => EspecieInfo { tamaño: 15.0, velocidad: 4.0, color: GREEN, energia: 80.0, es_presa: true, limite: 30, edad_reproductiva: 3 },
        Especie::Lobo => EspecieInfo { tamaño: 30.0, velocidad: 3.5, color: BLUE, energia: 200.0, es_presa: false, limite: 10, edad_reproductiva: 10 },
    }
}

// -----------------------------
// Struct para todos los organismos
// -----------------------------
struct Organismo {
    especie: Especie,
    macho: bool,
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    color: Color,
    energia: f32,
    tamaño: f32,
    edad: u32, // edad en días
    edad_reproductiva: u32,
    ultimo_reproduccion: Instant,
}

impl Organismo {
    fn mover(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.x <= 0.0 || self.x + self.tamaño >= screen_width() {
            self.vel_x *= -1.0;
        }
        if self.y <= 0.0 || self.y + self.tamaño >= screen_height() {
            self.vel_y *= -1.0;
        }
    }

    fn mostrar(&self) {
        let display_color = if self.macho {
            self.color
        } else {
            Color::new(
                (self.color.r + 0.3).min(1.0),
                (self.color.g + 0.3).min(1.0),
                (self.color.b + 0.3).min(1.0),
                1.0,
            )
        };
        draw_rectangle(self.x, self.y, self.tamaño, self.tamaño, display_color);
    }

    fn colisiona(&self, otra: &Organismo) -> bool {
        self.x < otra.x + otra.tamaño &&
        self.x + self.tamaño > otra.x &&
        self.y < otra.y + otra.tamaño &&
        self.y + self.tamaño > otra.y
    }
}

// -----------------------------
// Crear un organismo aleatorio según especie
// -----------------------------
fn crear_organismo(especie: Especie) -> Organismo {
    let info = atributos_especie(especie);
    Organismo {
        especie,
        macho: rand::gen_range(0.0, 1.0) < 0.5,
        x: rand::gen_range(0.0, screen_width() - info.tamaño),
        y: rand::gen_range(0.0, screen_height() - info.tamaño),
        vel_x: info.velocidad,
        vel_y: info.velocidad,
        color: info.color,
        energia: info.energia,
        tamaño: info.tamaño,
        edad: 0,
        edad_reproductiva: info.edad_reproductiva,
        ultimo_reproduccion: Instant::now() - Duration::from_secs(10),
    }
}

// -----------------------------
// Contar cantidad y edad promedio por especie
// -----------------------------
fn conteo_y_edad_promedio(organismos: &Vec<Organismo>) -> HashMap<Especie, (usize, f32)> {
    let mut conteo: HashMap<Especie, (usize, u32)> = HashMap::new();
    for org in organismos {
        let entry = conteo.entry(org.especie).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += org.edad;
    }

    let mut resultado: HashMap<Especie, (usize, f32)> = HashMap::new();
    for (especie, (cantidad, suma_edad)) in conteo {
        let promedio = if cantidad > 0 { suma_edad as f32 / cantidad as f32 } else { 0.0 };
        resultado.insert(especie, (cantidad, promedio));
    }
    resultado
}

// -----------------------------
// Función principal
// -----------------------------
#[macroquad::main("Ecosistema con Edad, Reproducción, Alimentación y Días")]
async fn main() {
    let mut organismos: Vec<Organismo> = Vec::new();

    let inicial_por_especie: HashMap<Especie, usize> = [
        (Especie::Conejo, 5),
        (Especie::ZorroPequeño, 3),
        (Especie::Ratón, 7),
        (Especie::Lobo, 2),
    ].iter().cloned().collect();

    for (especie, &cantidad) in &inicial_por_especie {
        for _ in 0..cantidad {
            organismos.push(crear_organismo(*especie));
        }
    }

    let mut tiempo_dias: f32 = 0.0;
    let mut dias_totales: u32 = 0;

    loop {
        clear_background(WHITE);

        // Incrementar edad simulando que 1 segundo = 1 día
        tiempo_dias += get_frame_time();
        if tiempo_dias >= 1.0 {
            for org in organismos.iter_mut() {
                org.edad += 1;
            }
            dias_totales += 1;
            tiempo_dias = 0.0;
        }

        // Mover y mostrar organismos
        for org in organismos.iter_mut() {
            org.mover();
            org.mostrar();
        }

        // -----------------------------
        // Reproducción y alimentación
        // -----------------------------
        let mut nuevos: Vec<Organismo> = Vec::new();
        let mut eliminados: Vec<usize> = Vec::new();
        let conteo = conteo_y_edad_promedio(&organismos);

        for i in 0..organismos.len() {
            for j in (i+1)..organismos.len() {
                let (o1, o2) = {
                    let (left, right) = organismos.split_at_mut(j);
                    (&mut left[i], &mut right[0])
                };

                if o1.colisiona(o2) {
                    // Alimentación
                    if !atributos_especie(o1.especie).es_presa && atributos_especie(o2.especie).es_presa {
                        o1.energia += o2.energia * 0.8;
                        eliminados.push(j);
                        continue;
                    } else if !atributos_especie(o2.especie).es_presa && atributos_especie(o1.especie).es_presa {
                        o2.energia += o1.energia * 0.8;
                        eliminados.push(i);
                        continue;
                    }

                    // Reproducción con límite de población y edad mínima
                    let limite = atributos_especie(o1.especie).limite;
                    let cantidad_actual = conteo.get(&o1.especie).map(|(c, _)| *c).unwrap_or(0);
                    if o1.especie == o2.especie &&
                       o1.macho != o2.macho &&
                       o1.edad >= o1.edad_reproductiva &&
                       o2.edad >= o2.edad_reproductiva &&
                       o1.ultimo_reproduccion.elapsed().as_secs() > 5 &&
                       o2.ultimo_reproduccion.elapsed().as_secs() > 5 &&
                       cantidad_actual < limite
                    {
                        nuevos.push(crear_organismo(o1.especie));
                        o1.ultimo_reproduccion = Instant::now();
                        o2.ultimo_reproduccion = Instant::now();
                    }
                }
            }
        }

        // Eliminar presas comidas
        eliminados.sort_unstable();
        eliminados.dedup();
        for &index in eliminados.iter().rev() {
            organismos.remove(index);
        }

        // Añadir nuevos organismos
        organismos.extend(nuevos);

        // -----------------------------
        // Mostrar interfaz con contador de días
        // -----------------------------
        draw_text(&format!("Días: {}", dias_totales), 10.0, 40.0, 25.0, BLACK);

        let conteo = conteo_y_edad_promedio(&organismos);
        let mut y = 70.0;
        for (especie, &cantidad) in &inicial_por_especie {
            let (cant, edad_prom) = conteo.get(especie).unwrap_or(&(0, 0.0));
            let nombre = match especie {
                Especie::Conejo => "Conejo",
                Especie::ZorroPequeño => "ZorroPequeño",
                Especie::Ratón => "Ratón",
                Especie::Lobo => "Lobo",
            };
            draw_text(&format!("{}: {} | Edad promedio: {:.1}", nombre, cant, edad_prom), 10.0, y, 20.0, BLACK);
            y += 25.0;
        }

        next_frame().await;
    }
}
