// quaternoin ops(operations) library
// f64 only

use std::ops::{Add, Sub, Mul, Neg, AddAssign, SubAssign, MulAssign};
use quaternion as quat;


#[derive(Copy, Clone)]
pub struct Quaternion {
    pub q: quat::Quaternion<f64>
}

// a + b
impl Add for Quaternion {
    type Output = Quaternion;
    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            q: quat::add(self.q, other.q)
        }
    }
}

// a - b
impl Sub for Quaternion {
    type Output = Quaternion;
    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            q: quat::sub(self.q, other.q)
        }
    }
}

// a * b
impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            q: quat::mul(self.q, other.q)
        }
    }
}

// -a
impl Neg for Quaternion {
    type Output = Quaternion;
    fn neg(self) -> Quaternion {
        Quaternion {
            q: quat::negate(self.q)
        }
    }
}

// a += b
impl AddAssign for Quaternion {
    fn add_assign(&mut self, other: Quaternion) {
        self.q = quat::add(self.q, other.q);
    }
}

// a -= b
impl SubAssign for Quaternion {
    fn sub_assign(&mut self, other: Quaternion) {
        self.q = quat::sub(self.q, other.q);
    }
}

// a *= b
impl MulAssign for Quaternion {
    fn mul_assign(&mut self, other: Quaternion) {
        self.q = quat::mul(self.q, other.q);
    }
}


impl Quaternion {
    pub fn new(q0: f64, q1: f64, q2: f64, q3: f64) -> Quaternion {
        Quaternion {
            q: (q0, [q1, q2, q3])
        }
    }

    /// get quaternion
    pub fn get(&self) -> (f64, [f64; 3]) {
        self.q
    }

    /// Return the real quaternion
    pub fn s(&self) -> Quaternion {
        Quaternion {
            q: ( (self.q).0, [0.0; 3] )
        }
    }

    /// Return the pure quaternion
    pub fn v(&self) -> Quaternion {
        Quaternion {
            q: ( 0.0, (self.q).1 )
        }
    }

    pub fn dot(&self, other: Quaternion) -> f64 {
        quat::dot(self.q, other.q)
    }

    pub fn norm(&self) -> f64 {
        quat::norm(self.q)
    }

    pub fn inv(&self) -> Quaternion {
        Quaternion {
            q: quat::inv(self.q)
        }
    }

    pub fn exp(&self) -> Quaternion {
        Quaternion {
            q: quat::exp(self.q)
        }
    }

    pub fn ln(&self) -> Quaternion {
        Quaternion {
            q: quat::ln(self.q)
        }
    }

    pub fn pow(&self, t: f64) -> Quaternion {
        Quaternion {
            q: quat::pow(self.q, t)
        }
    }

    pub fn scale(&self, s: f64) -> Quaternion {
        Quaternion {
            q: quat::scale(s, self.q)
        }
    }

    pub fn scale_add(&self, s: f64, other: Quaternion) -> Quaternion {
        Quaternion {
            q: quat::scale_add(s, self.q, other.q)
        }
    }
}