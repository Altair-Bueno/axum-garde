use axum::extract::*;

/// Trait for unwrapping extractor's payloads
///
/// Types that extract data from request should implement this trait, as it
/// unlocks extractor composition with third party crates
pub trait IntoInner {
    /// Wrapped payload type
    type Inner;
    /// Consume the extractor and unwrap the payload
    fn into_inner(self) -> Self::Inner;
}

macro_rules! gen_impl {
    ($name:ident) => {
        impl<T> IntoInner for $name<T> {
            type Inner = T;
            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }
    };
}

#[cfg(feature = "json")]
gen_impl!(Json);
#[cfg(feature = "query")]
gen_impl!(Query);
#[cfg(feature = "form")]
gen_impl!(Form);
gen_impl!(Path);
