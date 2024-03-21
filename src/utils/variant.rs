use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
};

pub trait Debug: std::fmt::Debug {}

impl Debug for Empty {}
impl Debug for bool {}
impl Debug for u8 {}
impl Debug for i8 {}
impl Debug for u16 {}
impl Debug for i16 {}
impl Debug for u32 {}
impl Debug for i32 {}
impl Debug for u64 {}
impl Debug for i64 {}
impl Debug for String {}
impl<T: std::fmt::Debug> Debug for Vec<T> {}
impl<T: std::fmt::Debug> Debug for Option<T> {}
impl<K: std::fmt::Debug, V: std::fmt::Debug> Debug for HashMap<K, V> {}

pub trait TVariant: Debug + Any + Send {
    fn into_variant(self) -> Variant;
    fn value(&self) -> Box<dyn TVariant>;
}

pub trait TVariantClone: TVariant {
    fn into_variant(self) -> Variant;
    fn value(&self) -> Box<dyn TVariantClone>;
}

pub trait TVariantCopy: TVariant {
    fn into_variant(self) -> Variant;
    fn value(&self) -> Box<dyn TVariantCopy>;
}

impl TVariant for bool {
    fn into_variant(self) -> Variant {
        Variant::new::<bool>(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for u8 {
    fn into_variant(self) -> Variant {
        Variant::new::<u8>(self)
    }

    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for i8 {
    fn into_variant(self) -> Variant {
        Variant::new::<i8>(self)
    }

    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for u16 {
    fn into_variant(self) -> Variant {
        Variant::new::<u16>(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for i16 {
    fn into_variant(self) -> Variant {
        Variant::new::<i16>(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for u32 {
    fn into_variant(self) -> Variant {
        Variant::new::<u32>(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for i32 {
    fn into_variant(self) -> Variant {
        Variant::new::<i32>(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for u64 {
    fn into_variant(self) -> Variant {
        Variant::new::<u64>(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for i64 {
    fn into_variant(self) -> Variant {
        Variant::new::<i64>(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}
impl TVariant for String {
    fn into_variant(self) -> Variant {
        Variant::new::<String>(self.clone())
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(self.clone())
    }
}
impl<T: TVariant + 'static> TVariant for Option<T>
where
    T: Clone,
{
    fn into_variant(self) -> Variant {
        Variant::new(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(self.clone())
    }
}
impl<T: TVariant + 'static> TVariant for Vec<T>
where
    T: Clone,
{
    fn into_variant(self) -> Variant {
        Variant::new(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(self.clone())
    }
}
impl<K: TVariant + 'static, V: TVariant + Send + 'static> TVariant for HashMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn into_variant(self) -> Variant {
        Variant::new(self)
    }
    fn value(&self) -> Box<dyn TVariant> {
        Box::new(self.clone())
    }
}
impl TVariant for Empty {
    fn into_variant(self) -> Variant {
        Variant::new::<Empty>(self)
    }

    fn value(&self) -> Box<dyn TVariant> {
        Box::new(*self)
    }
}

macro_rules! impl_debug {
    ( $( $V:ident )+ ) => {
        impl<$($V: Debug),+> Debug for ($($V,)+)
        {
        }
    };
}

impl_debug! {A}
impl_debug! {A B}
impl_debug! {A B C}
impl_debug! {A B C D}
impl_debug! {A B C D E}
impl_debug! {A B C D E F}
impl_debug! {A B C D E F G}
impl_debug! {A B C D E F G H}
impl_debug! {A B C D E F G H I}
impl_debug! {A B C D E F G H I J}
impl_debug! {A B C D E F G H I J K}
impl_debug! {A B C D E F G H I J K L}

macro_rules! impl_variant {
    ( $( $val:ident )+ ) => {
    #[allow(non_snake_case, unused_variables)]
    // removing this causes 80 warnings...
        impl<$($val: TVariant),+> TVariant for ( $($val,)+ )
        {
            fn into_variant(self) -> Variant {
                let ($($val,)+) = self;
                Variant::new::<( $($val,)+ )>( ( $($val,)+ ) )
            }

            fn value(&self) -> Box<dyn TVariant> {
                let ($($val,)+) = self;
                // TODO: check if this causes any value to brick?
                // theoretically copy or clone has to be used due to &self
                Box::new(0)
            }
        }
    };
}

impl_variant! { A }
impl_variant! { A B }
impl_variant! { A B C }
impl_variant! { A B C D }
impl_variant! { A B C D E }
impl_variant! { A B C D E F }
impl_variant! { A B C D E F G }
impl_variant! { A B C D E F G H }
impl_variant! { A B C D E F G H I }
impl_variant! { A B C D E F G H I J }
impl_variant! { A B C D E F G H I J K }
impl_variant! { A B C D E F G H I J K L }

#[allow(non_snake_case, unused_attributes)]
macro_rules! impl_variant_clone {
    ( $( $val:ident )+ ) => {
    #[allow(non_snake_case)]
    // removing this causes 80 warnings...
        impl<$($val: TVariantClone + Clone),+> TVariantClone for ( $($val,)+ )
        {
            fn into_variant(self) -> Variant {
                let ($($val,)+) = self;
                Variant::new::<( $($val,)+ )>( ( $($val,)+ ) )
            }

            fn value(&self) -> Box<dyn TVariantClone> {
                let ($($val,)+) = self;
                Box::new( ( $($val.clone(),)+ ) )
            }
        }
    };
}

impl_variant_clone! { A }
impl_variant_clone! { A B }
impl_variant_clone! { A B C }
impl_variant_clone! { A B C D }
impl_variant_clone! { A B C D E }
impl_variant_clone! { A B C D E F }
impl_variant_clone! { A B C D E F G }
impl_variant_clone! { A B C D E F G H }
impl_variant_clone! { A B C D E F G H I }
impl_variant_clone! { A B C D E F G H I J }
impl_variant_clone! { A B C D E F G H I J K }
impl_variant_clone! { A B C D E F G H I J K L }

macro_rules! impl_variant_copy {
    ( $( $val:ident )+ ) => {
    #[allow(non_snake_case)]
    // removing this causes 80 warnings...
        impl<$($val: TVariantCopy + Copy),+> TVariantCopy for ( $($val,)+ )
        {
            fn into_variant(self) -> Variant {
                let ($($val,)+) = self;
                Variant::new::<( $($val,)+ )>( ( $($val,)+ ) )
            }

            fn value(&self) -> Box<dyn TVariantCopy> {
                let ($($val,)+) = self;
                Box::new( ( $(*$val,)+ ) )
            }
        }
    };
}

impl_variant_copy! { A }
impl_variant_copy! { A B }
impl_variant_copy! { A B C }
impl_variant_copy! { A B C D }
impl_variant_copy! { A B C D E }
impl_variant_copy! { A B C D E F }
impl_variant_copy! { A B C D E F G }
impl_variant_copy! { A B C D E F G H }
impl_variant_copy! { A B C D E F G H I }
impl_variant_copy! { A B C D E F G H I J }
impl_variant_copy! { A B C D E F G H I J K }
impl_variant_copy! { A B C D E F G H I J K L }

#[derive(Debug)]
pub struct Variant {
    value: Box<dyn TVariant>,
    kind: TypeId,
}

impl Debug for Variant {}

impl TVariant for Variant {
    fn into_variant(self) -> Variant {
        self.clone()
    }

    fn value(&self) -> Box<dyn TVariant> {
        Box::new(self.clone())
    }
}

impl Clone for Variant {
    fn clone(&self) -> Self {
        Self {
            value: self.value.value(),
            kind: self.kind,
        }
    }
}

impl Variant {
    pub fn empty() -> Variant {
        Self::new::<Empty>(Empty {})
    }

    pub fn new<T: TVariant + 'static>(value: T) -> Self {
        let id = value.type_id();
        Variant {
            value: Box::new(value),
            kind: id,
        }
    }

    pub fn to_value<T: Copy + 'static>(&self) -> Result<T, ConversionError> {
        if self.kind != TypeId::of::<T>() {
            return Err(ConversionError("Conversion Failed"));
        }
        unsafe { Ok(*self.to_value_unchecked::<T>()) }
    }

    pub fn to_value_cloned<T: Clone + 'static>(&self) -> Result<&T, ConversionError> {
        if self.kind != TypeId::of::<T>() {
            return Err(ConversionError("Conversion Failed"));
        }
        unsafe { Ok(self.to_value_unchecked::<T>()) }
    }

    unsafe fn to_value_unchecked<T>(&self) -> &T {
        &*(self.value.deref() as *const dyn Any as *mut T)
    }
}

#[derive(Debug)]
pub struct ConversionError(pub &'static str);

#[derive(Clone, Copy, Debug)]
pub struct Empty {}

#[test]
fn test_i32() {
    let mock = 5.into_variant();
    assert_eq!(mock.kind, TypeId::of::<i32>());
    assert_eq!(mock.to_value::<i32>().unwrap(), 5);
}

#[test]
fn test_option() {
    let mock = Some(10).into_variant();
    assert_eq!(mock.kind, TypeId::of::<Option<i32>>());
    assert_eq!(mock.to_value::<Option<i32>>().unwrap(), Some(10));
}

#[test]
fn test_vec() {
    let mock = vec![3, 2, 4, 5, 10].into_variant();
    assert_eq!(mock.kind, TypeId::of::<Vec<i32>>());
    assert_eq!(
        mock.to_value_cloned::<Vec<i32>>().unwrap().clone(),
        vec![3, 2, 4, 5, 10]
    );
}

#[test]
fn test_hashmap() {
    let mut map = HashMap::new();
    map.insert("Something".to_string(), 20);
    let mock = map.into_variant();

    let mut testmap = HashMap::new();
    testmap.insert("Something".to_string(), 20);

    assert_eq!(mock.kind, TypeId::of::<HashMap<String, i32>>());
    assert_eq!(
        mock.to_value_cloned::<HashMap<String, i32>>()
            .unwrap()
            .clone(),
        testmap
    );
}

#[test]
fn test_conversion_fail() {
    let mock = "hello".to_string().into_variant();
    assert_eq!(mock.kind, TypeId::of::<String>());
    assert!(mock.to_value_cloned::<i32>().is_err());
}

#[test]
fn test_variant_clone() {
    let mut map = HashMap::new();
    map.insert("Something".to_string(), 20);
    let mock = map.into_variant();
    let new_mock = mock.clone();

    let mut testmap = HashMap::new();
    testmap.insert("Something".to_string(), 20);

    assert_eq!(
        new_mock
            .to_value_cloned::<HashMap<String, i32>>()
            .unwrap()
            .clone(),
        testmap
    );
}

#[test]
fn test_conversion_tuple() {
    let mock = (25, "pingpang".to_string()).into_variant();
    dbg!(&mock);
    assert_eq!(mock.kind, TypeId::of::<(i32, String)>());
    let converted_value = mock.to_value_cloned::<(i32, String)>();
    assert!(converted_value.is_ok());
    let comparison_value = (25, "pingpang".to_string());
    let converted_value = converted_value.unwrap();
    assert_eq!(converted_value.0, comparison_value.0);
    assert_eq!(converted_value.1, comparison_value.1);
}

#[test]
fn test_conversion_tuple_same_type() {
    let mock = (25, 25).into_variant();
    dbg!(&mock);
    assert_eq!(mock.kind, TypeId::of::<(i32, i32)>());
    let converted_value = mock.to_value::<(i32, i32)>();
    assert!(converted_value.is_ok());
    let comparison_value = (25, 25);
    let converted_value = converted_value.unwrap();
    assert_eq!(converted_value.0, comparison_value.0);
    assert_eq!(converted_value.1, comparison_value.1);
}
