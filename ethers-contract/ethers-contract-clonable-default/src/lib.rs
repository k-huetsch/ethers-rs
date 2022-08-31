use ethers_core::types::{Bytes, Uint8, H160, H256, H32, H512, H64, I256, U128, U256, U64};

pub trait ClonableDefault: Clone {
    fn clonable_default() -> Self;
}

impl<T: ClonableDefault> ClonableDefault for Vec<T> {
    fn clonable_default() -> Self {
        vec![]
    }
}

impl<T: ClonableDefault, const N: usize> ClonableDefault for [T; N] {
    fn clonable_default() -> Self {
        let item = T::clonable_default();
        [(); N].map(|_| item.clone())
    }
}

macro_rules! impl_by_default {
    ($ty: ident) => {
        impl ClonableDefault for $ty {
            fn clonable_default() -> Self {
                Default::default()
            }
        }
    };
}

macro_rules! impl_tuple {
    ( $( $name:ident )+ ) => {
        impl<$($name: ClonableDefault),+> ClonableDefault for ($($name,)+)
        {
            fn clonable_default() -> Self {
                ($($name::clonable_default(),)+)
            }
        }
    };
}

impl_by_default!(u8);
impl_by_default!(u16);
impl_by_default!(u32);
impl_by_default!(u64);
impl_by_default!(u128);

impl_by_default!(i8);
impl_by_default!(i16);
impl_by_default!(i32);
impl_by_default!(i64);
impl_by_default!(i128);
impl_by_default!(I256);

impl_by_default!(String);
impl_by_default!(bool);
impl_by_default!(Bytes);

impl_by_default!(H32);
impl_by_default!(H64);
impl_by_default!(H160);
impl_by_default!(H256);
impl_by_default!(H512);
impl_by_default!(U64);
impl_by_default!(U128);
impl_by_default!(U256);
impl_by_default!(Uint8);

impl_tuple! { A }
impl_tuple! { A B }
impl_tuple! { A B C }
impl_tuple! { A B C D }
impl_tuple! { A B C D E }
impl_tuple! { A B C D E F }
impl_tuple! { A B C D E F G }
impl_tuple! { A B C D E F G H }
impl_tuple! { A B C D E F G H I }
impl_tuple! { A B C D E F G H I J }
impl_tuple! { A B C D E F G H I J K }
impl_tuple! { A B C D E F G H I J K L }
