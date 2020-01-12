use quaternion_ops::*;
use quaternion as quat;

const A: quat::Quaternion<f64> = (1.0, [2.0, 3.0, 4.0]);
const B: quat::Quaternion<f64> = (3.0, [1.0, 4.0, 6.0]);


#[test]
fn add_test() {
    let a = Quaternion{ q: A };
    let b = Quaternion{ q: B };

    assert_eq!( (a+b).q, quat::add(A, B) );
}

#[test]
fn sub_test() {
    let a = Quaternion{ q: A };
    let b = Quaternion{ q: B };

    assert_eq!( (a-b).q, quat::sub(A, B) );
    assert_eq!( (b-a).q, quat::sub(B, A) );
}

#[test]
fn mul_test() {
    let a = Quaternion{ q: A };
    let b = Quaternion{ q: B };

    assert_eq!( (a*b).q, quat::mul(A, B) );
    assert_eq!( (b*a).q, quat::mul(B, A) );
}

#[test]
fn neg_test() {
    let a = Quaternion{ q: A };

    assert_eq!( (-a).q, quat::negate(A) );
}

#[test]
fn add_assign() {
    let mut a = Quaternion{ q: A };
    let b = Quaternion{ q: B };

    a += b;
    assert_eq!( a.q, quat::add(A, B) );
}

#[test]
fn sub_assign() {
    let mut a = Quaternion{ q: A };
    let b = Quaternion{ q: B };

    a -= b;
    assert_eq!( a.q, quat::sub(A, B) );
}

#[test]
fn mul_assign() {
    let mut a = Quaternion{ q: A };
    let b = Quaternion{ q: B };

    a *= b;
    assert_eq!( a.q, quat::mul(A, B) );
}