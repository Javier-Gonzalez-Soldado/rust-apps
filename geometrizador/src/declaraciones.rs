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

pub struct Triangulo {
    pub lado:f64
}

pub struct Poligono {
    pub nlados:u64,
    pub lado:f64
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
impl Area for Triangulo {
    fn area(&self) -> f64 {
        let mut altura; let c1 = self.lado/2.0;
        altura = self.lado.powf(2.0) - c1.powf(2.0); 
        altura = altura.powf(0.5);
        self.lado * altura /2.0
    }
}
impl Perimetro for Triangulo{
    fn perimetro(&self) -> f64 {
        self.lado * 3.0
    }
}

impl Area for Poligono {
    fn area(&self) -> f64 {
        let  apot; let c1 = self.lado/2.0;let nl = self.nlados as f64; let mut aux;
        aux = (nl + 2.0 * nl.powf(0.5))/nl;
        aux = aux.powf(0.5);
        apot = c1 * aux;
        (self.lado * self.nlados as f64) * 0.5 * apot 
    }
}

impl Perimetro for Poligono {
    fn perimetro(&self) -> f64 {
        self.lado * self.nlados as f64
    }
}