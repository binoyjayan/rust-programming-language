
// Equality operators can be overloaded using derive which does an element-wise comparison
#[derive(Debug, PartialEq, Eq)]
struct Complex<T> {
    real: T,
    imag: T
}

impl<T> Complex<T> {
    fn new(real: T, imag:T) -> Complex<T> {
        Complex::<T> {real, imag}
    }
}

// impl std::ops::Add for Complex<i32> {
//     type Output = Complex<i32>;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Complex {real: self.real + rhs.real,
//                  imag: self.imag + rhs.imag
//         }
//     }
// }

// Generic Complex type - use where clause
impl<T> std::ops::Add for Complex<T> where T: std::ops::Add <Output = T> {
    type Output = Complex<T>;

    fn add(self, other: Self) -> Self::Output {
        Complex {real: self.real + other.real,
            imag: self.imag + other.imag
        }
    }
}

//operator +=
impl<T> std::ops::AddAssign for Complex<T> where T: std::ops::AddAssign {
    fn add_assign(&mut self, other: Self) {
        self.real += other.real;
        self.imag += other.imag;
    }
}

// Negation operator
impl<T> std::ops::Neg for Complex<T> where T: std::ops::Neg <Output = T> {
    type Output = Complex<T>;

    fn neg(mut self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}


/* Overloading comparison operators
 *
 * Partial equality
 * Full equality x = x
 * NAN - Not a number x/0 or inf/inf
 * NAN == NAN -> false
 */

// Not needed when using 'derive'
// impl<T> PartialEq for Complex<T> where T: PartialEq  {
//     fn eq(&self, other: &Self) -> bool {
//         self.real == other.real && self.imag == other.imag
//     }
// }
//
// // Eq reuses PartialEq trait so an implementation is not necessary
// impl<T: Eq> Eq for Complex<T> where T: Eq  {
// }

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn traits_operator_overloading() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);
    let mut c = Complex::new(5.0, 6.0);
    let mut d = Complex::new(7.0, 8.0);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    println!("d = {:?}", c);
    let mut s = a + b;
    println!("a+b = {:?}", s);
    s += c;
    println!("a+b+c = {:?}", s);
    s = -s;
    println!("-sum = {:?}", s);

    println!("s == s {}", s == s);
    println!("s == b {}", s == d);
}
