use std::fmt;

const MAX_LVLS: u16 = !0 as u16;
const HEAP_SIZE: u16 = !0 as u16;

pub type Key = u16;

#[derive(Debug, Clone, Copy)]
pub enum ByteCode {
    Number(i64),
    Nil,
}

#[derive(Debug)]
pub enum Error {
    Capacity,
    InvalidOperation,
}

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub key: Key,
    pub value: ByteCode,
}

pub struct Heap {
    pub heap_tail: u16,
    pub lvl_count: u16,
    pub lvl_bounds: [u16; MAX_LVLS as usize],
    pub lvls: Vec<Entry>,
}

impl Heap {
    pub fn new() -> Self {
        let mut v = Vec::with_capacity(HEAP_SIZE as usize);
        for _ in 0..HEAP_SIZE {
            v.push(Entry {
                key: 0,
                value: ByteCode::Nil,
            });
        }
        Heap {
            heap_tail: 0,
            lvl_count: 1,
            lvl_bounds: [0u16; MAX_LVLS as usize],
            lvls: v,
        }
    }

    fn last_lvl_size(&self) -> u16 {
        if self.lvl_count == 1 {
            return self.lvl_bounds[0];
        }
        self.lvl_bounds[self.lvl_count as usize - 1] - self.lvl_bounds[self.lvl_count as usize - 2]
    }

    pub fn push_level(&mut self) -> Result<(), Error> {
        if self.heap_tail == HEAP_SIZE {
            return Err(Error::Capacity);
        }
        self.lvl_bounds[self.lvl_count as usize] = self.heap_tail;
        self.lvl_count += 1;
        Ok(())
    }

    pub fn pop_level(&mut self) -> Result<(), Error> {
        if self.heap_tail == 0 {
            return Err(Error::InvalidOperation);
        }

        self.lvl_bounds[self.lvl_count as usize] = 0;
        self.heap_tail -= self.last_lvl_size();
        self.lvl_count -= 1;
        Ok(())
    }

    pub fn insert(&mut self, key: Key, value: ByteCode) -> Result<(), Error> {
        if self.heap_tail == HEAP_SIZE - 1 {
            return Err(Error::Capacity);
        }
        self.lvls[self.heap_tail as usize] = Entry {
            key: key,
            value: value,
        };
        self.lvl_bounds[self.lvl_count as usize - 1] += 1;
        self.heap_tail += 1;
        Ok(())
    }

    pub fn get(&self, key: Key) -> Option<&ByteCode> {
        let tail = self.heap_tail;
        for i in (0..tail).rev() {
            println!("KEY: {}", self.lvls[i as usize].key);
            if self.lvls[i as usize].key == key {
                return Some(&self.lvls[i as usize].value);
            }
        }
        None
    }
}
