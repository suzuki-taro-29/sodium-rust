use crate::Cell;
use crate::Stream;

/// Operational primitives that must be used with care because they
/// break non-detectability of `Cell` steps/updates.
pub struct Operational {}

impl Operational {
    pub fn updates<A>(ca: &Cell<A>) -> Stream<A>
    where
        A: Clone + Send + 'static,
    {
        Stream {
            impl_: ca.impl_.updates(),
        }
    }

    pub fn value<A>(ca: &Cell<A>) -> Stream<A>
    where
        A: Clone + Send + 'static,
    {
        Stream {
            impl_: ca.impl_.value(),
        }
    }

    pub fn defer<A>(sa: &Stream<A>) -> Stream<A>
    where
        A: Clone + Send + 'static,
    {
        Stream {
            impl_: sa.impl_.defer(),
        }
    }
}
