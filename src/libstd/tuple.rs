// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations on tuples

#[allow(missing_doc)];

use kinds::Copy;
use vec;
use vec::ImmutableVector;
use iterator::IteratorUtil;

pub use self::inner::*;

/// Method extensions to pairs where both types satisfy the `Copy` bound
pub trait CopyableTuple<T, U> {
    /// Return the first element of self
    fn first(&self) -> T;
    /// Return the second element of self
    fn second(&self) -> U;
    /// Return the results of swapping the two elements of self
    fn swap(&self) -> (U, T);
}

impl<T:Copy,U:Copy> CopyableTuple<T, U> for (T, U) {
    /// Return the first element of self
    #[inline]
    fn first(&self) -> T {
        match *self {
            (ref t, _) => copy *t,
        }
    }

    /// Return the second element of self
    #[inline]
    fn second(&self) -> U {
        match *self {
            (_, ref u) => copy *u,
        }
    }

    /// Return the results of swapping the two elements of self
    #[inline]
    fn swap(&self) -> (U, T) {
        match copy *self {
            (t, u) => (u, t),
        }
    }
}

/// Method extensions for pairs where the types don't necessarily satisfy the
/// `Copy` bound
pub trait ImmutableTuple<T, U> {
    /// Return a reference to the first element of self
    fn first_ref<'a>(&'a self) -> &'a T;
    /// Return a reference to the second element of self
    fn second_ref<'a>(&'a self) -> &'a U;
}

impl<T, U> ImmutableTuple<T, U> for (T, U) {
    #[inline]
    fn first_ref<'a>(&'a self) -> &'a T {
        match *self {
            (ref t, _) => t,
        }
    }
    #[inline]
    fn second_ref<'a>(&'a self) -> &'a U {
        match *self {
            (_, ref u) => u,
        }
    }
}

pub trait ExtendedTupleOps<A,B> {
    fn zip(&self) -> ~[(A, B)];
    fn map<C>(&self, f: &fn(a: &A, b: &B) -> C) -> ~[C];
}

impl<'self,A:Copy,B:Copy> ExtendedTupleOps<A,B> for (&'self [A], &'self [B]) {
    #[inline]
    fn zip(&self) -> ~[(A, B)] {
        match *self {
            (ref a, ref b) => {
                vec::zip_slice(*a, *b)
            }
        }
    }

    #[inline]
    fn map<C>(&self, f: &fn(a: &A, b: &B) -> C) -> ~[C] {
        match *self {
            (ref a, ref b) => {
                a.iter().zip(b.iter()).transform(|(aa, bb)| f(aa, bb)).collect()
            }
        }
    }
}

impl<A:Copy,B:Copy> ExtendedTupleOps<A,B> for (~[A], ~[B]) {
    #[inline]
    fn zip(&self) -> ~[(A, B)] {
        match *self {
            (ref a, ref b) => {
                vec::zip_slice(*a, *b)
            }
        }
    }

    #[inline]
    fn map<C>(&self, f: &fn(a: &A, b: &B) -> C) -> ~[C] {
        match *self {
            (ref a, ref b) => {
                a.iter().zip(b.iter()).transform(|(aa, bb)| f(aa, bb)).collect()
            }
        }
    }
}

// macro for implementing n-ary tuple functions and operations

macro_rules! tuple_impls {
    ($(
        ($cloneable_trait:ident, $immutable_trait:ident) {
            $(($get_fn:ident, $get_ref_fn:ident) -> $T:ident {
                $get_pattern:pat => $ret:expr
            })+
        }
    )+) => {
        pub mod inner {
            use clone::Clone;
            #[cfg(not(test))] use cmp::*;
            #[cfg(not(test))] use num::Zero;

            $(
                pub trait $cloneable_trait<$($T),+> {
                    $(fn $get_fn(&self) -> $T;)+
                }

                impl<$($T:Clone),+> $cloneable_trait<$($T),+> for ($($T),+) {
                    $(
                        #[inline]
                        fn $get_fn(&self) -> $T {
                            self.$get_ref_fn().clone()
                        }
                    )+
                }

                pub trait $immutable_trait<$($T),+> {
                    $(fn $get_ref_fn<'a>(&'a self) -> &'a $T;)+
                }

                impl<$($T),+> $immutable_trait<$($T),+> for ($($T),+) {
                    $(
                        #[inline]
                        fn $get_ref_fn<'a>(&'a self) -> &'a $T {
                            match *self { $get_pattern => $ret }
                        }
                    )+
                }

                impl<$($T:Clone),+> Clone for ($($T),+) {
                    fn clone(&self) -> ($($T),+) {
                        ($(self.$get_ref_fn().clone()),+)
                    }
                }

                #[cfg(not(test))]
                impl<$($T:Eq),+> Eq for ($($T),+) {
                    #[inline]
                    fn eq(&self, other: &($($T),+)) -> bool {
                        $(*self.$get_ref_fn() == *other.$get_ref_fn())&&+
                    }
                    #[inline]
                    fn ne(&self, other: &($($T),+)) -> bool {
                        !(*self == *other)
                    }
                }

                #[cfg(not(test))]
                impl<$($T:TotalEq),+> TotalEq for ($($T),+) {
                    #[inline]
                    fn equals(&self, other: &($($T),+)) -> bool {
                        $(self.$get_ref_fn().equals(other.$get_ref_fn()))&&+
                    }
                }

                #[cfg(not(test))]
                impl<$($T:Ord),+> Ord for ($($T),+) {
                    #[inline]
                    fn lt(&self, other: &($($T),+)) -> bool {
                        lexical_lt!($(self.$get_ref_fn(), other.$get_ref_fn()),+)
                    }
                    #[inline]
                    fn le(&self, other: &($($T),+)) -> bool { !(*other).lt(&(*self)) }
                    #[inline]
                    fn ge(&self, other: &($($T),+)) -> bool { !(*self).lt(other) }
                    #[inline]
                    fn gt(&self, other: &($($T),+)) -> bool { (*other).lt(&(*self)) }
                }

                #[cfg(not(test))]
                impl<$($T:TotalOrd),+> TotalOrd for ($($T),+) {
                    #[inline]
                    fn cmp(&self, other: &($($T),+)) -> Ordering {
                        lexical_cmp!($(self.$get_ref_fn(), other.$get_ref_fn()),+)
                    }
                }

                #[cfg(not(test))]
                impl<$($T:Zero),+> Zero for ($($T),+) {
                    #[inline]
                    fn zero() -> ($($T),+) {
                        ($(Zero::zero::<$T>()),+)
                    }
                    #[inline]
                    fn is_zero(&self) -> bool {
                        $(self.$get_ref_fn().is_zero())&&+
                    }
                }
            )+
        }
    }
}

// Constructs an expression that performs a lexical less-than
// ordering.  The values are interleaved, so the macro invocation for
// `(a1, a2, a3) < (b1, b2, b3)` would be `lexical_lt!(a1, b1, a2, b2,
// a3, b3)` (and similarly for `lexical_cmp`)
macro_rules! lexical_lt {
    ($a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
        if *$a < *$b { true }
        else if !(*$b < *$a) { lexical_lt!($($rest_a, $rest_b),+) }
        else { false }
    };
    ($a:expr, $b:expr) => { *$a < *$b };
}

macro_rules! lexical_cmp {
    ($a:expr, $b:expr, $($rest_a:expr, $rest_b:expr),+) => {
        match ($a).cmp($b) {
            Equal => lexical_cmp!($($rest_a, $rest_b),+),
            ordering   => ordering
        }
    };
    ($a:expr, $b:expr) => { ($a).cmp($b) };
}


tuple_impls! {
    (CloneableTuple2, ImmutableTuple2) {
        (n0, n0_ref) -> A { (ref a,_) => a }
        (n1, n1_ref) -> B { (_,ref b) => b }
    }

    (CloneableTuple3, ImmutableTuple3) {
        (n0, n0_ref) -> A { (ref a,_,_) => a }
        (n1, n1_ref) -> B { (_,ref b,_) => b }
        (n2, n2_ref) -> C { (_,_,ref c) => c }
    }

    (CloneableTuple4, ImmutableTuple4) {
        (n0, n0_ref) -> A { (ref a,_,_,_) => a }
        (n1, n1_ref) -> B { (_,ref b,_,_) => b }
        (n2, n2_ref) -> C { (_,_,ref c,_) => c }
        (n3, n3_ref) -> D { (_,_,_,ref d) => d }
    }

    (CloneableTuple5, ImmutableTuple5) {
        (n0, n0_ref) -> A { (ref a,_,_,_,_) => a }
        (n1, n1_ref) -> B { (_,ref b,_,_,_) => b }
        (n2, n2_ref) -> C { (_,_,ref c,_,_) => c }
        (n3, n3_ref) -> D { (_,_,_,ref d,_) => d }
        (n4, n4_ref) -> E { (_,_,_,_,ref e) => e }
    }

    (CloneableTuple6, ImmutableTuple6) {
        (n0, n0_ref) -> A { (ref a,_,_,_,_,_) => a }
        (n1, n1_ref) -> B { (_,ref b,_,_,_,_) => b }
        (n2, n2_ref) -> C { (_,_,ref c,_,_,_) => c }
        (n3, n3_ref) -> D { (_,_,_,ref d,_,_) => d }
        (n4, n4_ref) -> E { (_,_,_,_,ref e,_) => e }
        (n5, n5_ref) -> F { (_,_,_,_,_,ref f) => f }
    }

    (CloneableTuple7, ImmutableTuple7) {
        (n0, n0_ref) -> A { (ref a,_,_,_,_,_,_) => a }
        (n1, n1_ref) -> B { (_,ref b,_,_,_,_,_) => b }
        (n2, n2_ref) -> C { (_,_,ref c,_,_,_,_) => c }
        (n3, n3_ref) -> D { (_,_,_,ref d,_,_,_) => d }
        (n4, n4_ref) -> E { (_,_,_,_,ref e,_,_) => e }
        (n5, n5_ref) -> F { (_,_,_,_,_,ref f,_) => f }
        (n6, n6_ref) -> G { (_,_,_,_,_,_,ref g) => g }
    }

    (CloneableTuple8, ImmutableTuple8) {
        (n0, n0_ref) -> A { (ref a,_,_,_,_,_,_,_) => a }
        (n1, n1_ref) -> B { (_,ref b,_,_,_,_,_,_) => b }
        (n2, n2_ref) -> C { (_,_,ref c,_,_,_,_,_) => c }
        (n3, n3_ref) -> D { (_,_,_,ref d,_,_,_,_) => d }
        (n4, n4_ref) -> E { (_,_,_,_,ref e,_,_,_) => e }
        (n5, n5_ref) -> F { (_,_,_,_,_,ref f,_,_) => f }
        (n6, n6_ref) -> G { (_,_,_,_,_,_,ref g,_) => g }
        (n7, n7_ref) -> H { (_,_,_,_,_,_,_,ref h) => h }
    }

    (CloneableTuple9, ImmutableTuple9) {
        (n0, n0_ref) -> A { (ref a,_,_,_,_,_,_,_,_) => a }
        (n1, n1_ref) -> B { (_,ref b,_,_,_,_,_,_,_) => b }
        (n2, n2_ref) -> C { (_,_,ref c,_,_,_,_,_,_) => c }
        (n3, n3_ref) -> D { (_,_,_,ref d,_,_,_,_,_) => d }
        (n4, n4_ref) -> E { (_,_,_,_,ref e,_,_,_,_) => e }
        (n5, n5_ref) -> F { (_,_,_,_,_,ref f,_,_,_) => f }
        (n6, n6_ref) -> G { (_,_,_,_,_,_,ref g,_,_) => g }
        (n7, n7_ref) -> H { (_,_,_,_,_,_,_,ref h,_) => h }
        (n8, n8_ref) -> I { (_,_,_,_,_,_,_,_,ref i) => i }
    }

    (CloneableTuple10, ImmutableTuple10) {
        (n0, n0_ref) -> A { (ref a,_,_,_,_,_,_,_,_,_) => a }
        (n1, n1_ref) -> B { (_,ref b,_,_,_,_,_,_,_,_) => b }
        (n2, n2_ref) -> C { (_,_,ref c,_,_,_,_,_,_,_) => c }
        (n3, n3_ref) -> D { (_,_,_,ref d,_,_,_,_,_,_) => d }
        (n4, n4_ref) -> E { (_,_,_,_,ref e,_,_,_,_,_) => e }
        (n5, n5_ref) -> F { (_,_,_,_,_,ref f,_,_,_,_) => f }
        (n6, n6_ref) -> G { (_,_,_,_,_,_,ref g,_,_,_) => g }
        (n7, n7_ref) -> H { (_,_,_,_,_,_,_,ref h,_,_) => h }
        (n8, n8_ref) -> I { (_,_,_,_,_,_,_,_,ref i,_) => i }
        (n9, n9_ref) -> J { (_,_,_,_,_,_,_,_,_,ref j) => j }
    }

    (CloneableTuple11, ImmutableTuple11) {
        (n0,  n0_ref)  -> A { (ref a,_,_,_,_,_,_,_,_,_,_) => a }
        (n1,  n1_ref)  -> B { (_,ref b,_,_,_,_,_,_,_,_,_) => b }
        (n2,  n2_ref)  -> C { (_,_,ref c,_,_,_,_,_,_,_,_) => c }
        (n3,  n3_ref)  -> D { (_,_,_,ref d,_,_,_,_,_,_,_) => d }
        (n4,  n4_ref)  -> E { (_,_,_,_,ref e,_,_,_,_,_,_) => e }
        (n5,  n5_ref)  -> F { (_,_,_,_,_,ref f,_,_,_,_,_) => f }
        (n6,  n6_ref)  -> G { (_,_,_,_,_,_,ref g,_,_,_,_) => g }
        (n7,  n7_ref)  -> H { (_,_,_,_,_,_,_,ref h,_,_,_) => h }
        (n8,  n8_ref)  -> I { (_,_,_,_,_,_,_,_,ref i,_,_) => i }
        (n9,  n9_ref)  -> J { (_,_,_,_,_,_,_,_,_,ref j,_) => j }
        (n10, n10_ref) -> K { (_,_,_,_,_,_,_,_,_,_,ref k) => k }
    }

    (CloneableTuple12, ImmutableTuple12) {
        (n0,  n0_ref)  -> A { (ref a,_,_,_,_,_,_,_,_,_,_,_) => a }
        (n1,  n1_ref)  -> B { (_,ref b,_,_,_,_,_,_,_,_,_,_) => b }
        (n2,  n2_ref)  -> C { (_,_,ref c,_,_,_,_,_,_,_,_,_) => c }
        (n3,  n3_ref)  -> D { (_,_,_,ref d,_,_,_,_,_,_,_,_) => d }
        (n4,  n4_ref)  -> E { (_,_,_,_,ref e,_,_,_,_,_,_,_) => e }
        (n5,  n5_ref)  -> F { (_,_,_,_,_,ref f,_,_,_,_,_,_) => f }
        (n6,  n6_ref)  -> G { (_,_,_,_,_,_,ref g,_,_,_,_,_) => g }
        (n7,  n7_ref)  -> H { (_,_,_,_,_,_,_,ref h,_,_,_,_) => h }
        (n8,  n8_ref)  -> I { (_,_,_,_,_,_,_,_,ref i,_,_,_) => i }
        (n9,  n9_ref)  -> J { (_,_,_,_,_,_,_,_,_,ref j,_,_) => j }
        (n10, n10_ref) -> K { (_,_,_,_,_,_,_,_,_,_,ref k,_) => k }
        (n11, n11_ref) -> L { (_,_,_,_,_,_,_,_,_,_,_,ref l) => l }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clone::Clone;
    use cmp::*;

    #[test]
    fn test_tuple_ref() {
        let x = (~"foo", ~"bar");
        assert_eq!(x.first_ref(), &~"foo");
        assert_eq!(x.second_ref(), &~"bar");
    }

    #[test]
    #[allow(non_implicitly_copyable_typarams)]
    fn test_tuple() {
        assert_eq!((948, 4039.48).first(), 948);
        assert_eq!((34.5, ~"foo").second(), ~"foo");
        assert_eq!(('a', 2).swap(), (2, 'a'));
    }

    #[test]
    fn test_clone() {
        let a = (1, ~"2");
        let b = a.clone();
        assert_eq!(a.first(), b.first());
        assert_eq!(a.second(), b.second());
    }

    #[test]
    fn test_n_tuple() {
        let t = (0u8, 1u16, 2u32, 3u64, 4u, 5i8, 6i16, 7i32, 8i64, 9i, 10f32, 11f64);
        assert_eq!(t.n0(), 0u8);
        assert_eq!(t.n1(), 1u16);
        assert_eq!(t.n2(), 2u32);
        assert_eq!(t.n3(), 3u64);
        assert_eq!(t.n4(), 4u);
        assert_eq!(t.n5(), 5i8);
        assert_eq!(t.n6(), 6i16);
        assert_eq!(t.n7(), 7i32);
        assert_eq!(t.n8(), 8i64);
        assert_eq!(t.n9(), 9i);
        assert_eq!(t.n10(), 10f32);
        assert_eq!(t.n11(), 11f64);

        assert_eq!(t.n0_ref(), &0u8);
        assert_eq!(t.n1_ref(), &1u16);
        assert_eq!(t.n2_ref(), &2u32);
        assert_eq!(t.n3_ref(), &3u64);
        assert_eq!(t.n4_ref(), &4u);
        assert_eq!(t.n5_ref(), &5i8);
        assert_eq!(t.n6_ref(), &6i16);
        assert_eq!(t.n7_ref(), &7i32);
        assert_eq!(t.n8_ref(), &8i64);
        assert_eq!(t.n9_ref(), &9i);
        assert_eq!(t.n10_ref(), &10f32);
        assert_eq!(t.n11_ref(), &11f64);
    }

    #[test]
    fn test_tuple_cmp() {
        let (small, big) = ((1u, 2u, 3u), (3u, 2u, 1u));

        // Eq
        assert_eq!(small, small);
        assert_eq!(big, big);
        assert!(small != big);
        assert!(big != small);

        // Ord
        assert!(small < big);
        assert!(!(small < small));
        assert!(!(big < small));
        assert!(!(big < big));

        assert!(small <= small);
        assert!(big <= big);

        assert!(big > small);
        assert!(small >= small);
        assert!(big >= small);
        assert!(big >= big);

        // TotalEq
        assert!(small.equals(&small));
        assert!(big.equals(&big));
        assert!(!small.equals(&big));
        assert!(!big.equals(&small));

        // TotalOrd
        assert_eq!(small.cmp(&small), Equal);
        assert_eq!(big.cmp(&big), Equal);
        assert_eq!(small.cmp(&big), Less);
        assert_eq!(big.cmp(&small), Greater);
    }
}
