// #
//
// env.rs
// Copyright (C) 2016 Lynx ltd. <anton@algotradinghub.com>
// Created by Anton Kundenko.
//

use std::fmt;

const KEY_SIZE: usize = 16;
const DICT_CAPACITY: usize = 16;
const ENV_CAPACITY: usize = 16;

pub type Key = [u8; KEY_SIZE];

#[derive(Debug, Clone, Copy)]
pub enum AST {
    Number(i64),
    Nil,
}

#[derive(Debug)]
pub enum Error {
    Capacity,
}

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub key: Key,
    pub value: AST,
}

#[derive(Debug, Clone, Copy)]
pub struct Level {
    size: usize,
    entries: [Entry; DICT_CAPACITY],
}

impl Level {
    pub fn new() -> Self {
        Level {
            size: 1,
            entries: [Entry {
                key: [0u8; KEY_SIZE],
                value: AST::Nil,
            }; DICT_CAPACITY],
        }
    }

    pub fn insert(&mut self, key: Key, value: AST) -> Result<(), Error> {
        let idx = self.size;
        match idx {
            DICT_CAPACITY => Err(Error::Capacity),
            _ => {
                self.entries[idx] = Entry {
                    key: key,
                    value: value,
                };
                self.size += 1;
                Ok(())
            }
        }
    }

    pub fn get(&self, key: Key) -> Option<&Entry> {
        for i in 0..self.size {
            if self.entries[i].key == key {
                return Some(&self.entries[i]);
            };
        }
        None
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for e in self.entries.iter() {
            let _ = write!(f, "Entry: {:?}\n", e);
        }
        write!(f, "Size: {:?}\n", self.size)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Environment {
    pub size: usize,
    pub levels: [Level; ENV_CAPACITY],
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            size: 1,
            levels: [Level::new(); ENV_CAPACITY],
        }
    }

    pub fn new_child(&mut self) -> Result<&mut Level, Error> {
        let id = &mut self.size;
        match *id {
            ENV_CAPACITY => Err(Error::Capacity),
            _ => {
                *id += 1;
                Ok(&mut self.levels[*id])
            }
        }
    }

    pub fn last_mut(&mut self) -> &mut Level {
        &mut self.levels[self.size - 1]
    }

    pub fn get(&self, key: Key) -> Option<(&AST, &Level)> {
        for i in (0..self.size).rev() {
            if let Some(e) = self.levels[i].get(key) {
                return Some((&e.value, &self.levels[i]));
            }
        }
        None
    }
}
