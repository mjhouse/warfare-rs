use serde::{
    Deserialize,
    Deserializer, 
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

const INITIAL: usize = 1;
const ORDERING: Ordering = Ordering::SeqCst;

pub enum IdType {
    Unit,
    Player,
    Message,
}

macro_rules! increment {
    ( $atomic: ident ) => {
        $atomic.fetch_add(1, ORDERING)
    }
}

macro_rules! load {
    ( $atomic: ident ) => {
        $atomic.load(ORDERING)
    }
}

macro_rules! store {
    ( $atomic: ident, $value: expr ) => {
        $atomic.store($value,ORDERING)
    }
}

macro_rules! identifier {
    ( $name: ident, $atomic: ident ) => {

        // backing global counter for id generation
        static $atomic: AtomicUsize = AtomicUsize::new(INITIAL);

        // wrapper id struct
        #[derive(Default, Serialize, Deserialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub struct $name(usize);

        impl $name {
            
            pub fn new() -> Self {
                $name(increment!($atomic))
            }

            pub fn reset() -> Self {
                store!($atomic,INITIAL);
                Self::new()
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

macro_rules! identifier_ex {
    ( $name: ident) => {

        // wrapper id struct
        #[derive(Default, Serialize, Deserialize, Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub struct $name(uuid::Uuid);

        impl $name {
            
            pub fn new() -> Self {
                $name(uuid::Uuid::new_v4())
            }

        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}

identifier_ex!(Id);
identifier_ex!(PlayerId);
identifier_ex!(MessageId);