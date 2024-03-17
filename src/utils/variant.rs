use std::{any::Any, collections::HashMap, ops::Deref};

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
    fn into_mock_variant(self) -> Variant;
}

impl TVariant for bool {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<bool>(self, "bool")
    }
}
impl TVariant for u8 {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<u8>(self, "u8")
    }
}
impl TVariant for i8 {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<i8>(self, "i8")
    }
}
impl TVariant for u16 {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<u16>(self, "u16")
    }
}
impl TVariant for i16 {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<i16>(self, "i16")
    }
}
impl TVariant for u32 {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<u32>(self, "u32")
    }
}
impl TVariant for i32 {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<i32>(self, "i32")
    }
}
impl TVariant for u64 {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<u64>(self, "u64")
    }
}
impl TVariant for i64 {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<i64>(self, "i64")
    }
}
impl TVariant for String {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<String>(self.clone(), "String")
    }
}
impl<T: IntrospectType + Debug + Send + 'static> TVariant for Option<T>
where
    T: IntrospectType + Clone,
{
    fn into_mock_variant(self) -> Variant {
        Variant::new(self, "Option ".to_string() + &T::get_type())
    }
}
impl<T: IntrospectType + Debug + Send + 'static> TVariant for Vec<T>
where
    T: IntrospectType + Clone,
{
    fn into_mock_variant(self) -> Variant {
        Variant::new(self, "Vec ".to_string() + &T::get_type())
    }
}
impl<K: IntrospectType + Debug + Send + 'static, V: IntrospectType + Debug + Send + 'static>
    TVariant for HashMap<K, V>
where
    K: IntrospectType + Clone,
    V: IntrospectType + Clone,
{
    fn into_mock_variant(self) -> Variant {
        Variant::new(
            self,
            "HashMap ".to_string() + &K::get_type() + " " + &V::get_type(),
        )
    }
}

#[derive(Debug)]
pub struct Variant {
    value: Box<dyn TVariant>,
    kind: String,
}

impl Variant {
    pub fn empty() -> Self {
        Self::new::<Empty>(Empty {}, "None")
    }

    pub fn new<T: TVariant + 'static>(value: T, kind: impl Into<String>) -> Self {
        Variant {
            value: Box::new(value),
            kind: kind.into(),
        }
    }

    pub fn to_value<T: Copy>(&self, conversion_type: &'static str) -> Result<T, ConversionError> {
        if self.kind != conversion_type {
            return Err(ConversionError("Conversion Failed"));
        }
        unsafe { Ok(*self.to_value_unchecked::<T>()) }
    }

    pub fn to_value_cloned<T: Clone>(
        &self,
        conversion_type: &'static str,
    ) -> Result<&T, ConversionError> {
        if self.kind != conversion_type {
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

impl IntrospectType for Empty {
    fn get_type() -> String {
        "None".into()
    }
}

impl TVariant for Empty {
    fn into_mock_variant(self) -> Variant {
        Variant::new::<Empty>(self, "None")
    }
}

pub trait IntrospectType {
    fn get_type() -> String;
}

impl IntrospectType for bool {
    fn get_type() -> String {
        "bool".into()
    }
}

impl IntrospectType for u8 {
    fn get_type() -> String {
        "u8".into()
    }
}

impl IntrospectType for i8 {
    fn get_type() -> String {
        "i8".into()
    }
}

impl IntrospectType for u16 {
    fn get_type() -> String {
        "u16".into()
    }
}

impl IntrospectType for i16 {
    fn get_type() -> String {
        "i16".into()
    }
}

impl IntrospectType for u32 {
    fn get_type() -> String {
        "u32".into()
    }
}

impl IntrospectType for i32 {
    fn get_type() -> String {
        "i32".into()
    }
}

impl IntrospectType for u64 {
    fn get_type() -> String {
        "u64".into()
    }
}

impl IntrospectType for i64 {
    fn get_type() -> String {
        "i64".into()
    }
}

impl IntrospectType for String {
    fn get_type() -> String {
        "String".into()
    }
}

impl<T: IntrospectType> IntrospectType for Option<T> {
    fn get_type() -> String {
        "Option".to_string() + " " + &T::get_type()
    }
}

impl<T: IntrospectType> IntrospectType for Vec<T> {
    fn get_type() -> String {
        "Vec".to_string() + " " + &T::get_type()
    }
}

impl<K: IntrospectType, V: IntrospectType> IntrospectType for HashMap<K, V> {
    fn get_type() -> String {
        "HashMap".to_string() + " " + &K::get_type() + " " + &V::get_type()
    }
}

#[test]
fn test_i32() {
    let mock = 5.into_mock_variant();
    assert_eq!(mock.kind, "i32".to_string());
    assert_eq!(mock.to_value::<i32>("i32").unwrap(), 5);
}

#[test]
fn test_option() {
    let mock = Some(10).into_mock_variant();
    assert_eq!(mock.kind, "Option i32".to_string());
    assert_eq!(
        mock.to_value::<Option<i32>>("Option i32").unwrap(),
        Some(10)
    );
}

#[test]
fn test_vec() {
    let mock = vec![3, 2, 4, 5, 10].into_mock_variant();
    assert_eq!(mock.kind, "Vec i32".to_string());
    assert_eq!(
        mock.to_value_cloned::<Vec<i32>>("Vec i32").unwrap().clone(),
        vec![3, 2, 4, 5, 10]
    );
}

#[test]
fn test_hashmap() {
    let mut map = HashMap::new();
    map.insert("Something".to_string(), 20);
    let mock = map.into_mock_variant();

    let mut testmap = HashMap::new();
    testmap.insert("Something".to_string(), 20);

    assert_eq!(mock.kind, "HashMap String i32".to_string());
    assert_eq!(
        mock.to_value_cloned::<HashMap<String, i32>>("HashMap String i32")
            .unwrap()
            .clone(),
        testmap
    );
}

#[test]
fn test_conversion_fail() {
    let mock = "hello".to_string().into_mock_variant();
    assert_eq!(mock.kind, "String".to_string());
    assert!(mock.to_value_cloned::<i32>("Not String").is_err());
}
