/// Trait for unwrapping extractor's payloads
///
/// Types that extract data from request should implement this trait, as it
/// unlocks extractor composition with this library
pub trait IntoInner {
    /// Wrapped payload type
    type Inner;
    /// Consume the extractor and unwrap the payload
    fn into_inner(self) -> Self::Inner;
}

macro_rules! impl_into_inner_simple {
    (
        $name:ty,
        [$($type_var:ident),* $(,)?]
    ) => {
        impl<$($type_var),*> IntoInner for $name {
            type Inner = T;
            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }
    };
}

// Axum
#[cfg(feature = "json")]
impl_into_inner_simple!(axum::extract::Json<T>, [T]);
impl_into_inner_simple!(axum::extract::Extension<T>, [T]);
#[cfg(feature = "form")]
impl_into_inner_simple!(axum::extract::Form<T>, [T]);
impl_into_inner_simple!(axum::extract::Path<T>, [T]);
#[cfg(feature = "query")]
impl_into_inner_simple!(axum::extract::Query<T>, [T]);
impl_into_inner_simple!(axum::extract::State<T>, [T]);

// Axum extra
#[cfg(feature = "extra-protobuf")]
impl_into_inner_simple!(axum_extra::protobuf::Protobuf<T>, [T]);
#[cfg(feature = "extra-query")]
impl_into_inner_simple!(axum_extra::extract::Query<T>, [T]);
