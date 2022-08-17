pub trait Area{
    fn area(&self) -> f64;
}

pub trait Perimetro{
    fn perimetro(&self) -> f64;
}

pub struct Circulo{
    pub radio:f64
}

pub struct Cuadrado{
    pub lado:f64
}

pub struct Rectangulo {
    pub base:f64,
    pub altura:f64
}

impl Area for Cuadrado {
    fn area(&self) -> f64 {
        self.lado * self.lado
    }
}
impl Perimetro for Cuadrado {
    fn perimetro(&self) -> f64 {
        self.lado * 4.0
    }
}

impl Area for Rectangulo {
    fn area(&self) -> f64 {
        self.base * self.altura
    }
}
impl Perimetro for Rectangulo {
    fn perimetro(&self) -> f64 {
        self.base * 2.0 + self.altura * 2.0
    }
}

impl Area for Circulo {
    fn area(&self) -> f64 {
        use::std::f64::consts::PI;
        PI * self.radio.powf(2.0)
    }
}
impl Perimetro for Circulo {
    fn perimetro(&self) -> f64 {
        use::std::f64::consts::PI;
        self.radio * 2.0 * PI
    }
}
