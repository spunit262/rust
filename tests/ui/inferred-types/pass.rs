//@ check-pass
#![feature(inferred_types)]

trait Trait {
    const T: Self;
}

#[derive(Eq, PartialEq)]
enum A {
    X,
    Y(),
    Z {},
}

impl A {
    const I: A = _::X;
}

impl Trait for A {
    const T: A = _::Z {};
}

fn a_in1(a: A) {
    match a {
        _::X => (),
        _::Y() => (),
        _::Z {} => (),
    }
}

fn a_in2(a: A) {
    match a {
        // FIXME: RFC says these should be rejected.
        _::I => (),
        _::T => (),
        _ => (),
    }
}

fn a_out(i: u8) -> A {
    match i {
        0 => _::X,
        1 => _::Z {},
        // FIXME: RFC says these should be rejected.
        2 => _::I,
        _ => _::T,
    }
}

fn main() {
}
