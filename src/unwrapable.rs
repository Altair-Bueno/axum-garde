use axum::extract::*;

pub trait Unwrapable<T> {
    fn extract(self) -> T;
}

macro_rules! gen_impl {
    ($name:ident) => {
        impl<T> Unwrapable<T> for $name<T> {
            fn extract(self) -> T {
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
