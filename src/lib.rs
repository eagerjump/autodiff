use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Dual {
    a: f32,
    b: f32,
}

impl Add for Dual {
    type Output = Dual;
    fn add(self, other: Dual) -> Dual {
        Dual {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl Sub for Dual {
    type Output = Dual;
    fn sub(self, other: Dual) -> Dual {
        Dual {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

impl Mul for Dual {
    type Output = Dual;
    fn mul(self, other: Dual) -> Dual {
        Dual {
            a: self.a * other.a,
            b: self.a * other.b + other.a * self.b,
        }
    }
}

impl Div for Dual {
    type Output = Dual;
    fn div(self, other: Dual) -> Dual {
        Dual {
            a: self.a / other.a,
            b: (other.a * self.b - self.a * other.b) / (other.a * other.a)
        }
    }
}

impl Dual {
    #[allow(dead_code)]
    fn exp(self) -> Dual {
        Dual {
            a: self.a.exp(),
            b: self.b * self.a.exp(),
        }
    }
    
    #[allow(dead_code)]
    fn sin(self) -> Dual {
        Dual {
            a: self.a.sin(),
            b: self.b * self.a.cos(),
        }
    }

    #[allow(dead_code)]
    fn cos(self) -> Dual {
        Dual {
            a: self.a.cos(),
            b: self.b * self.a.sin(),
        }
    }

    #[allow(dead_code)]
    fn tan(self) -> Dual {
        Dual {
            a: self.a.tan(),
            b: self.b / (self.a.cos() * self.a.cos()),
        }
    }
}



#[cfg(test)]
mod tests {   
    use Dual;
    #[test]
    fn test_polynomial() {
        let x = Dual { a: -4.0, b: 1.0 };

        fn func(x: Dual) -> Dual {
            let a1 = Dual { a: 5.0, b: 0.0 };
            let a2 = Dual { a: 3.0, b: 0.0 };
            let a3 = Dual { a: 1.0, b: 0.0 };
            a1 * x * x + a2 * x + a3
        }

        let y = func(x);
        assert_eq!(y.b, -37.0)
    }

    #[test]
    fn test_exp() {
        let x = Dual { a: 0.0, b: 1.0 };

        fn func(x: Dual) -> Dual { 
            let a1 = Dual { a: 10.0, b: 0.0 };
            a1 * x.exp()
        }

        let y = func(x);
        assert_eq!(y.b, 10.0);
    }
}
