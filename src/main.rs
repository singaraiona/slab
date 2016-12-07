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

fn insert(env: &mut Environment, key: [u8; 16], value: AST) {
    let lvl = env.last_mut();
    let _ = lvl.insert(key, value);
}

#[derive(Copy, Clone)]
struct Tmp {
    i: usize,
    a: [u8; 16],
}

#[derive(Copy)]
struct Cap {
    i: usize,
    inner: [Tmp; 12048],
}

impl Clone for Cap {
    fn clone(&self) -> Cap {
        *self
    }
}

#[derive(Copy)]
struct Cap2 {
    i: usize,
    inner: [Cap; 12048],
}

impl Clone for Cap2 {
    fn clone(&self) -> Cap2 {
        *self
    }
}

impl Cap {
    pub fn new() -> Self {
        Cap {
            i: 23,
            inner: [Tmp {
                i: 12,
                a: [0u8; 16],
            }; 12048],
        }
    }
}

impl Cap2 {
    pub fn new() -> Self {
        Cap2 {
            i: 23,
            inner: [Cap::new(); 12048],
        }
    }
}

fn main() {
    let key1 = String::from("a");
    let key2 = String::from("b");
    let k1 = str2key(key1.clone());
    let k2 = str2key(key2.clone());

    // let mut e = Environment::new();
    // insert(&mut e, k1, AST::Number(1));
    // let _ = e.new_child();
    // insert(&mut e, k2, AST::Number(2));

    // println!("Get Key {}: {:?}", key1, e.get(k1).unwrap().0);
    // println!("Get Key {}: {:?}", key2, e.get(k2).unwrap().0);

    let c = Cap2::new();

}
