#[cfg(FALSE)]
fn f() {
    _::A; //~ ERROR paths starting with
    _::A(); //~ ERROR paths starting with
    _::A {}; //~ ERROR paths starting with
    match () {
        _::A => (), //~ ERROR paths starting with
        _::A() => (), //~ ERROR paths starting with
        _::A {} => (), //~ ERROR paths starting with
    }
}

fn main() {
    // not this feature
    _();  //~ ERROR in expressions
}
