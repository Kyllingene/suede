use suede::*;

use std::marker::PhantomData;

pub trait Hash {
    fn hash(&self, state: &mut u64);
}

impl Hash for u64 {
    fn hash(&self, state: &mut u64) {
        *state <<= 7;
        *state ^= *self;
    }
}

impl Hash for i64 {
    fn hash(&self, state: &mut u64) {
        *state <<= 7;
        *state ^= *self as u64;
    }
}

impl Hash for bool {
    fn hash(&self, state: &mut u64) {
        *state <<= 1;
        *state ^= *self as u64;
    }
}

impl Hash for str {
    fn hash(&self, state: &mut u64) {
        (self.len() as u64).hash(state);
        for ch in self.chars() {
            *state <<= 3;
            *state ^= ch as u64;
        }
    }
}

impl Hash for String {
    fn hash(&self, state: &mut u64) {
        (self.len() as u64).hash(state);
        for ch in self.chars() {
            *state <<= 3;
            *state ^= ch as u64;
        }
    }
}

impl Hash for () {
    fn hash(&self, _: &mut u64) {}
}

pub struct Hasher<M: ?Sized> {
    state: u64,
    _marker: PhantomData<fn() -> M>,
}

impl<M: ?Sized> Default for Hasher<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: ?Sized> Hasher<M> {
    pub fn new() -> Self {
        Self { state: 0, _marker: PhantomData }
    }
}

impl<M: ?Sized> Collector for Hasher<M> {
    type Output = u64;
    type Error = Never;
    type Meta = M;

    fn finish(self) -> Result<u64, Never> { Ok(self.state) }
}

impl<M: Hash + ?Sized, N: Hash + ?Sized> Provide<N> for Hasher<M> {
    type Adapter = Hasher<N>;

    fn provide(&self) -> Self::Adapter {
        Hasher::new()
    }

    fn restore(&mut self, adapter: Self::Adapter, meta: &M) -> Result<(), Never> {
        meta.hash(&mut self.state);
        adapter.state.hash(&mut self.state);
        Ok(())
    }
}

impl<T: Hash + ?Sized, M: Hash + ?Sized> Collect<T> for Hasher<M> {
    fn collect(&mut self, data: &T, meta: &M) -> Result<(), Never> {
        meta.hash(&mut self.state);
        data.hash(&mut self.state);
        Ok(())
    }
}
