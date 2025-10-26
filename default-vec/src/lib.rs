#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Vec<T>"))]
pub struct DefaultVec<T>(Vec<T>);

impl<T> Default for DefaultVec<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(vec![T::default()])
    }
}

#[allow(clippy::into_iter_without_iter)]
impl<'a, T> IntoIterator for &'a DefaultVec<T> {
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
    type Item = <&'a Vec<T> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T> AsMut<[T]> for DefaultVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<T> AsRef<[T]> for DefaultVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> From<T> for DefaultVec<T> {
    fn from(value: T) -> Self {
        Self(vec![value])
    }
}

impl<T, const N: usize> From<[T; N]> for DefaultVec<T> {
    fn from(value: [T; N]) -> Self {
        Self(value.into())
    }
}
