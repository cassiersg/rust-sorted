use std::ops;
use std::marker::PhantomData;

use super::{
    Sorted,
    SortOrder,
};


#[derive(Debug)]
pub struct SortedSlice<'a,T,O>
where
    T:'a
{
    inner: &'a[T],
    ordering: PhantomData<*const O>
}

impl<'a,T,O> Sorted for SortedSlice<'a,T,O> {
    type Ordering = O;
}

impl<'a,T,O> SortedSlice<'a,T,O>
where
    T: Ord,
    O: SortOrder<T>
{
    pub fn by_sorting(slice: &'a mut [T], _: O) -> Self {
        O::sort(slice);
        Self {inner: slice, ordering: PhantomData} 
    }
}

impl<'a,T,O> SortedSlice<'a,T,O> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

impl<'a,T,O> PartialEq<&'a [T]> for SortedSlice<'a,T,O>
where T: Ord
{
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice() == *other
    }

    fn ne(&self, other: &&[T]) -> bool {
        self.as_slice() != *other
    }
}

impl<'a,T,O> ops::Index<usize> for SortedSlice<'a,T,O>
{
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

/*
impl<'a,T,S> ops::Deref for SortedSlice<'a,T,S> 
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.inner
    }
}
*/

impl<'a,T,O,U> From<&'a U> for SortedSlice<'a,T,O>
where
    U: 'a + ops::Deref<Target=[T]> + Sorted<Ordering=O>,
{
    fn from(x: &'a U) -> Self {
        SortedSlice {
            inner: x.deref(),
            ordering: PhantomData
        }
    }
}