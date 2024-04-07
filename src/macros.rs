#[macro_export]
macro_rules! hala_bitflags_wrapped {
  ($ name : ident , $ flag_type : ty) => {
    impl Default for $name {
      fn default() -> Self {
        Self(0)
      }
    }
    impl $name {
      #[inline]
      pub const fn empty() -> Self {
        Self(0)
      }
      #[inline]
      pub const fn from_raw(x: $flag_type) -> Self {
        Self(x)
      }
      #[inline]
      pub const fn as_raw(self) -> $flag_type {
        self.0
      }
      #[inline]
      pub const fn is_empty(self) -> bool {
        self.0 == Self::empty().0
      }
      #[inline]
      pub const fn intersects(self, other: Self) -> bool {
        !Self(self.0 & other.0).is_empty()
      }
      #[doc = r" Returns whether `other` is a subset of `self`"]
      #[inline]
      pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
      }
    }
    impl ::std::ops::BitOr for $name {
      type Output = Self;
      #[inline]
      fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
      }
    }
    impl ::std::ops::BitOrAssign for $name {
      #[inline]
      fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
      }
    }
    impl ::std::ops::BitAnd for $name {
      type Output = Self;
      #[inline]
      fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
      }
    }
    impl ::std::ops::BitAndAssign for $name {
      #[inline]
      fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
      }
    }
    impl ::std::ops::BitXor for $name {
      type Output = Self;
      #[inline]
      fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
      }
    }
    impl ::std::ops::BitXorAssign for $name {
      #[inline]
      fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
      }
    }
    impl ::std::ops::Not for $name {
      type Output = Self;
      #[inline]
      fn not(self) -> Self {
        Self(!self.0)
      }
    }
  };
}
