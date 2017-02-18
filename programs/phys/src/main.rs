extern crate siunits;

fn main() {
    let m1 = siunits::meters(5);
    let s1 = siunits::seconds(4);
    let v1 = siunits::velocity(5);
    let v2 = v1 - (m1 / s1);
    println!("{} and {}", v1, v2);
    println!("force of: {}",
             siunits::velocity(10) / siunits::seconds(1) * siunits::grams(50));
}
