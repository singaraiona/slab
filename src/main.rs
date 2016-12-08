pub mod env;
pub mod heap;

use env::{AST, Level, Environment};
use heap::{Heap, ByteCode};

fn str2key(s: String) -> [u8; 16] {
    let mut key = [0u8; 16];
    let sb = s.into_bytes();
    for i in 0..sb.len() {
        key[i] = sb[i];
    }
    key
}

fn test_env() {
    let key1 = 1;
    let key2 = 2;

    let mut e = Environment::new();
    let _ = e.last_mut().insert(key1, AST::Number(1));
    let _ = e.new_child();
    let _ = e.last_mut().insert(key2, AST::Number(2));

    println!("Get Key {}: {:?}", key1, e.get(key1).unwrap().0);
    println!("Get Key {}: {:?}", key2, e.get(key2).unwrap().0);

}

fn test_heap() {
    let mut h = Heap::new();
    let _ = h.insert(1, ByteCode::Number(1));
    let _ = h.insert(2, ByteCode::Number(2));
    let _ = h.push_level();
    let _ = h.insert(3, ByteCode::Number(3));

    println!("Get Key {}: {:?}", 1, h.get(1).unwrap());
    println!("Get Key {}: {:?}", 2, h.get(2).unwrap());
    println!("Get Key {}: {:?}", 3, h.get(3).unwrap());

    let _ = h.pop_level();
    println!("Get Key {}: {:?}", 3, h.get(3).unwrap());
}

fn main() {
    test_heap()
}
