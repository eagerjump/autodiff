#![allow(unused_variables)]
fn main() {
    use std::ops::{Add, Sub, Mul, Div};
    
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
        fn exp(self) -> Dual {
            Dual {
                a: self.a.exp(),
                b: self.b * self.a.exp(),
            }
        }

        fn sin(self) -> Dual {
            Dual {
                a: self.a.sin(),
                b: self.b * self.a.cos(),
            }
        }

        fn cos(self) -> Dual {
            Dual {
                a: self.a.cos(),
                b: self.b * self.a.sin(),
            }
        }

        fn tan(self) -> Dual {
            Dual {
                a: self.a.tan(),
                b: self.b / (1 - self.a.tan() * self.a.tan()),
            }
        }
    }
}
