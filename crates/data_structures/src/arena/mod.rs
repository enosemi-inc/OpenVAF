use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Range, RangeInclusive},
};

// TODO add tiny idx?

/// Backing storge used for `Idx<T>`
pub type RawIdx = u32;

/// The index of a value allocated in an arena that holds `T`s.
pub struct Idx<T> {
    raw: u32,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Idx<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Idx<T> {}

impl<T> PartialEq for Idx<T> {
    #[inline(always)]
    fn eq(&self, other: &Idx<T>) -> bool {
        self.raw == other.raw
    }
}
impl<T> Eq for Idx<T> {}

impl<T> Hash for Idx<T> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> PartialOrd for Idx<T> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl<T> Ord for Idx<T> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T> fmt::Debug for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut type_name = std::any::type_name::<T>();
        if let Some(idx) = type_name.rfind(':') {
            type_name = &type_name[idx + 1..];
        }
        write!(f, "Idx::<{}>({})", type_name, self.raw)
    }
}

impl<T> Idx<T> {
    /// Creates a new index from a [`RawIdx`].
    #[inline(always)]
    pub fn from_raw(raw: RawIdx) -> Self {
        Idx { raw, _ty: PhantomData }
    }

    /// Converts this index into the underlying [`RawIdx`].
    #[inline(always)]
    pub fn into_raw(self) -> RawIdx {
        self.raw
    }
}

impl<T: 'static> index_vec::Idx for Idx<T> {
    #[inline]
    fn from_usize(idx: usize) -> Self {
        Self::from_raw(idx.try_into().unwrap())
    }

    #[inline(always)]
    fn index(self) -> usize {
        self.into_raw() as usize
    }
}

/// A range of densely allocated arena values.
pub struct IdxRange<T> {
    range: Range<u32>,
    _p: PhantomData<T>,
}

impl<T> IdxRange<T> {
    /// Creates a new index range
    /// inclusive of the start value and exclusive of the end value.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let a = arena.alloc("a");
    /// let b = arena.alloc("b");
    /// let c = arena.alloc("c");
    /// let d = arena.alloc("d");
    ///
    /// let range = la_arena::IdxRange::new(b..d);
    /// assert_eq!(&arena[range], &["b", "c"]);
    /// ```
    pub fn new(range: Range<Idx<T>>) -> Self {
        Self { range: range.start.into_raw().into()..range.end.into_raw().into(), _p: PhantomData }
    }

    /// Creates a new index range
    /// inclusive of the start value and end value.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let foo = arena.alloc("foo");
    /// let bar = arena.alloc("bar");
    /// let baz = arena.alloc("baz");
    ///
    /// let range = la_arena::IdxRange::new_inclusive(foo..=baz);
    /// assert_eq!(&arena[range], &["foo", "bar", "baz"]);
    ///
    /// let range = la_arena::IdxRange::new_inclusive(foo..=foo);
    /// assert_eq!(&arena[range], &["foo"]);
    /// ```
    pub fn new_inclusive(range: RangeInclusive<Idx<T>>) -> Self {
        Self {
            range: u32::from(range.start().into_raw())..u32::from(range.end().into_raw()) + 1,
            _p: PhantomData,
        }
    }

    /// Merges two ranges so that the result covers both range (plus all idecies
    /// between these two ranges)
    pub fn cover(&self, other: &Self) -> Self {
        Self { range: self.range.start..other.range.end, _p: PhantomData }
    }

    /// Returns whether the index range is empty.
    ///
    /// ```
    /// let mut arena = la_arena::Arena::new();
    /// let one = arena.alloc(1);
    /// let two = arena.alloc(2);
    ///
    /// assert!(la_arena::IdxRange::new(one..one).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
}

impl<T> Iterator for IdxRange<T> {
    type Item = Idx<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|raw| Idx::from_raw(raw.into()))
    }
}

impl<T> DoubleEndedIterator for IdxRange<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(|raw| Idx::from_raw(raw.into()))
    }
}

impl<T> fmt::Debug for IdxRange<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&format!("IdxRange::<{}>", std::any::type_name::<T>()))
            .field(&self.range)
            .finish()
    }
}

impl<T> Clone for IdxRange<T> {
    fn clone(&self) -> Self {
        Self { range: self.range.clone(), _p: PhantomData }
    }
}

impl<T> PartialEq for IdxRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}

impl<T> Eq for IdxRange<T> {}

/// Index based Arena that automatically provides an Index Type for convenience
pub type Arena<T> = index_vec::IndexVec<Idx<T>, T>;
pub type ArenaMap<I,T> = index_vec::IndexVec<Idx<I>, T>;
