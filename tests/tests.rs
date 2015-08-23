extern crate stack;

use stack::{ArrayVec, SmallVec, Vector, Addressable};

#[test]
fn addressable() {
    fn addressible<T: Vector<Item=u32> + Addressable<u32>>(_: T) { }

    addressible(SmallVec::<[u32; 4]>::new());
    addressible(ArrayVec::<[u32; 4]>::new());
}
