pub mod env;

use env::{AST, Level, Environment};

fn str2key(s: String) -> [u8; 16] {
    let mut key = [0u8; 16];
    let sb = s.into_bytes();
    for i in 0..sb.len() {
        key[i] = sb[i];
    }
    key
}

fn main() {
    let key1 = 1;
    let key2 = 2;

    let mut e = Environment::new();
    let _ = e.last_mut().insert(key1, AST::Number(1));
    let _ = e.new_child();
    let _ = e.last_mut().insert(key2, AST::Number(2));

    println!("Get Key {}: {:?}", key1, e.get(key1).unwrap().0);
    println!("Get Key {}: {:?}", key2, e.get(key2).unwrap().0);

}
