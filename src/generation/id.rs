use serde::{
    Deserialize, 
    Serialize,
};
use std::sync::atomic::{
    AtomicUsize, 
    Ordering,
};
use std::{
    ops,
    fmt,
};

macro_rules! increment {
    ( $atomic: ident ) => {
        $atomic.fetch_add(1, Ordering::SeqCst)
    }
}

macro_rules! identifier {
    ( $name: ident, $atomic: ident ) => {

        // backing global counter for id generation
        static $atomic: AtomicUsize = AtomicUsize::new(1);

        // wrapper id struct
        #[derive(Default, Serialize, Deserialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub struct $name(usize);

        impl $name {
            
            pub fn new() -> Self {
                $name(increment!($atomic))
            }

            pub fn inner(self) -> usize {
                self.0
            }

        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ops::Deref for $name {
            type Target = usize;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
}

identifier!(Id,ID);
identifier!(PlayerId,PLAYER);