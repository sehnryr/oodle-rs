mod vec4d;

pub use vec4d::*;

#[cfg(feature = "simd")]
mod element {
    use std::simd::SimdElement;

    pub trait Element: SimdElement {}

    impl<T> Element for T where T: SimdElement {}
}

#[cfg(not(feature = "simd"))]
mod element {
    pub trait Element {}

    impl<T> Element for T {}
}

use element::Element;
