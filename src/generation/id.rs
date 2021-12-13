use std::sync::atomic::{AtomicUsize,Ordering};

pub type Id = usize;

static ID: AtomicUsize = AtomicUsize::new(0);

pub fn get() -> usize {
    ID.fetch_add(1, Ordering::SeqCst)
}

pub fn set(i: usize) {
    ID.store(i, Ordering::SeqCst);
}