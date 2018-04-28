#[cfg(feature = "xyz")]
fn xyz() {
    println!("xyz enabled");
}


#[cfg(not(feature = "xyz"))]
fn xyz() {
    println!("xyz disabled");
}


fn main() {
    xyz()
}
