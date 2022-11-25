use bevy::prelude::*;

pub trait BundleBundleExt: Bundle + Sized {
    fn bundle<B: Bundle>(self, b: B) -> (Self, B) {
        (self, b)
    }
}

impl<B: Bundle> BundleBundleExt for B {}
