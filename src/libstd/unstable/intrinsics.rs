// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*! rustc compiler intrinsics.

The corresponding definitions are in librustc/middle/trans/foreign.rs.

# Atomics

The atomic intrinsics provide common atomic operations on machine
words, with multiple possible memory orderings. They obey the same
semantics as C++11. See the LLVM documentation on [[atomics]].

[atomics]: http://llvm.org/docs/Atomics.html

A quick refresher on memory ordering:

* Acquire - a barrier for aquiring a lock. Subsequent reads and writes
  take place after the barrier.
* Release - a barrier for releasing a lock. Preceding reads and writes
  take place before the barrier.
* Sequentially consistent - sequentially consistent operations are
  guaranteed to happen in order. This is the standard mode for working
  with atomic types and is equivalent to Java's `volatile`.

*/

// This is needed to prevent duplicate lang item definitions.
#[cfg(test)]
pub use realstd::unstable::intrinsics::{TyDesc, Opaque, TyVisitor};

#[cfg(not(stage0))]
pub type GlueFn = extern "Rust" fn(*i8);

#[cfg(stage0)]
pub type GlueFn = extern "Rust" fn(**TyDesc, *i8);

// NB: this has to be kept in sync with the Rust ABI.
#[lang="ty_desc"]
#[cfg(not(test))]
pub struct TyDesc {
    size: uint,
    align: uint,
    take_glue: GlueFn,
    drop_glue: GlueFn,
    free_glue: GlueFn,
    visit_glue: GlueFn,
}

#[lang="opaque"]
#[cfg(not(test))]
pub enum Opaque { }

#[lang="ty_visitor"]
#[cfg(not(test))]
pub trait TyVisitor {
    fn visit_bot(&self) -> bool;
    fn visit_nil(&self) -> bool;
    fn visit_bool(&self) -> bool;

    fn visit_int(&self) -> bool;
    fn visit_i8(&self) -> bool;
    fn visit_i16(&self) -> bool;
    fn visit_i32(&self) -> bool;
    fn visit_i64(&self) -> bool;

    fn visit_uint(&self) -> bool;
    fn visit_u8(&self) -> bool;
    fn visit_u16(&self) -> bool;
    fn visit_u32(&self) -> bool;
    fn visit_u64(&self) -> bool;

    fn visit_float(&self) -> bool;
    fn visit_f32(&self) -> bool;
    fn visit_f64(&self) -> bool;

    fn visit_char(&self) -> bool;
    fn visit_str(&self) -> bool;

    fn visit_estr_box(&self) -> bool;
    fn visit_estr_uniq(&self) -> bool;
    fn visit_estr_slice(&self) -> bool;
    fn visit_estr_fixed(&self, n: uint, sz: uint, align: uint) -> bool;

    fn visit_box(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_uniq(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_ptr(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_rptr(&self, mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_vec(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_unboxed_vec(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_box(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_uniq(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_slice(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_fixed(&self, n: uint, sz: uint, align: uint,
                        mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_enter_rec(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_rec_field(&self, i: uint, name: &str,
                       mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_rec(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_class(&self, n_fields: uint,
                         sz: uint, align: uint) -> bool;
    fn visit_class_field(&self, i: uint, name: &str,
                         mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_class(&self, n_fields: uint,
                         sz: uint, align: uint) -> bool;

    fn visit_enter_tup(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_tup_field(&self, i: uint, inner: *TyDesc) -> bool;
    fn visit_leave_tup(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_enum(&self, n_variants: uint,
                        get_disr: extern unsafe fn(ptr: *Opaque) -> int,
                        sz: uint, align: uint) -> bool;
    fn visit_enter_enum_variant(&self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_enum_variant_field(&self, i: uint, offset: uint, inner: *TyDesc) -> bool;
    fn visit_leave_enum_variant(&self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_leave_enum(&self, n_variants: uint,
                        get_disr: extern unsafe fn(ptr: *Opaque) -> int,
                        sz: uint, align: uint) -> bool;

    fn visit_enter_fn(&self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;
    fn visit_fn_input(&self, i: uint, mode: uint, inner: *TyDesc) -> bool;
    fn visit_fn_output(&self, retstyle: uint, inner: *TyDesc) -> bool;
    fn visit_leave_fn(&self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;

    fn visit_trait(&self) -> bool;
    fn visit_var(&self) -> bool;
    fn visit_var_integral(&self) -> bool;
    fn visit_param(&self, i: uint) -> bool;
    fn visit_self(&self) -> bool;
    fn visit_type(&self) -> bool;
    fn visit_opaque_box(&self) -> bool;
    fn visit_constr(&self, inner: *TyDesc) -> bool;
    fn visit_closure_ptr(&self, ck: uint) -> bool;
}

#[abi = "rust-intrinsic"]
pub extern "rust-intrinsic" {

    /// Atomic compare and exchange, sequentially consistent.
    pub fn atomic_cxchg(dst: &mut int, old: int, src: int) -> int;
    /// Atomic compare and exchange, acquire ordering.
    pub fn atomic_cxchg_acq(dst: &mut int, old: int, src: int) -> int;
    /// Atomic compare and exchange, release ordering.
    pub fn atomic_cxchg_rel(dst: &mut int, old: int, src: int) -> int;

    pub fn atomic_cxchg_acqrel(dst: &mut int, old: int, src: int) -> int;
    pub fn atomic_cxchg_relaxed(dst: &mut int, old: int, src: int) -> int;


    /// Atomic load, sequentially consistent.
    pub fn atomic_load(src: &int) -> int;
    /// Atomic load, acquire ordering.
    pub fn atomic_load_acq(src: &int) -> int;

    pub fn atomic_load_relaxed(src: &int) -> int;

    /// Atomic store, sequentially consistent.
    pub fn atomic_store(dst: &mut int, val: int);
    /// Atomic store, release ordering.
    pub fn atomic_store_rel(dst: &mut int, val: int);

    pub fn atomic_store_relaxed(dst: &mut int, val: int);

    /// Atomic exchange, sequentially consistent.
    pub fn atomic_xchg(dst: &mut int, src: int) -> int;
    /// Atomic exchange, acquire ordering.
    pub fn atomic_xchg_acq(dst: &mut int, src: int) -> int;
    /// Atomic exchange, release ordering.
    pub fn atomic_xchg_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_xchg_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_xchg_relaxed(dst: &mut int, src: int) -> int;

    /// Atomic addition, sequentially consistent.
    pub fn atomic_xadd(dst: &mut int, src: int) -> int;
    /// Atomic addition, acquire ordering.
    pub fn atomic_xadd_acq(dst: &mut int, src: int) -> int;
    /// Atomic addition, release ordering.
    pub fn atomic_xadd_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_xadd_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_xadd_relaxed(dst: &mut int, src: int) -> int;

    /// Atomic subtraction, sequentially consistent.
    pub fn atomic_xsub(dst: &mut int, src: int) -> int;
    /// Atomic subtraction, acquire ordering.
    pub fn atomic_xsub_acq(dst: &mut int, src: int) -> int;
    /// Atomic subtraction, release ordering.
    pub fn atomic_xsub_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_xsub_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_xsub_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_and(dst: &mut int, src: int) -> int;
    pub fn atomic_and_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_and_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_and_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_and_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_nand(dst: &mut int, src: int) -> int;
    pub fn atomic_nand_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_nand_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_nand_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_nand_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_or(dst: &mut int, src: int) -> int;
    pub fn atomic_or_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_or_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_or_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_or_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_xor(dst: &mut int, src: int) -> int;
    pub fn atomic_xor_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_xor_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_xor_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_xor_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_max(dst: &mut int, src: int) -> int;
    pub fn atomic_max_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_max_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_max_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_max_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_min(dst: &mut int, src: int) -> int;
    pub fn atomic_min_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_min_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_min_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_min_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_umin(dst: &mut int, src: int) -> int;
    pub fn atomic_umin_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_umin_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_umin_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_umin_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_umax(dst: &mut int, src: int) -> int;
    pub fn atomic_umax_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_umax_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_umax_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_umax_relaxed(dst: &mut int, src: int) -> int;

    /// The size of a type in bytes.
    ///
    /// This is the exact number of bytes in memory taken up by a
    /// value of the given type. In other words, a memset of this size
    /// would *exactly* overwrite a value. When laid out in vectors
    /// and structures there may be additional padding between
    /// elements.
    pub fn size_of<T>() -> uint;

    /// Move a value to a memory location containing a value.
    ///
    /// Drop glue is run on the destination, which must contain a
    /// valid Rust value.
    pub fn move_val<T>(dst: &mut T, src: T);

    /// Move a value to an uninitialized memory location.
    ///
    /// Drop glue is not run on the destination.
    pub fn move_val_init<T>(dst: &mut T, src: T);

    pub fn min_align_of<T>() -> uint;
    pub fn pref_align_of<T>() -> uint;

    /// Get a static pointer to a type descriptor.
    #[cfg(not(stage0))]
    pub fn get_tydesc<T>() -> *TyDesc;
    #[cfg(stage0)]
    pub fn get_tydesc<T>() -> *();

    /// Create a value initialized to zero.
    ///
    /// `init` is unsafe because it returns a zeroed-out datum,
    /// which is unsafe unless T is POD. We don't have a POD
    /// kind yet. (See #4074).
    pub unsafe fn init<T>() -> T;

    /// Create an uninitialized value.
    pub unsafe fn uninit<T>() -> T;

    /// Move a value out of scope without running drop glue.
    ///
    /// `forget` is unsafe because the caller is responsible for
    /// ensuring the argument is deallocated already.
    pub unsafe fn forget<T>(_: T) -> ();
    pub fn transmute<T,U>(e: T) -> U;

    /// Returns `true` if a type requires drop glue.
    pub fn needs_drop<T>() -> bool;

    /// Returns `true` if a type is managed (will be allocated on the local heap)
    #[cfg(not(stage0))]
    pub fn contains_managed<T>() -> bool;

    #[cfg(not(stage0))]
    pub fn visit_tydesc(td: *TyDesc, tv: @TyVisitor);

    pub fn frame_address(f: &once fn(*u8));

    /// Get the address of the `__morestack` stack growth function.
    pub fn morestack_addr() -> *();

    /// Equivalent to the `llvm.memcpy.p0i8.0i8.i32` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memcpy32<T>(dst: *mut T, src: *T, count: u32);
    /// Equivalent to the `llvm.memcpy.p0i8.0i8.i64` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memcpy64<T>(dst: *mut T, src: *T, count: u64);

    /// Equivalent to the `llvm.memmove.p0i8.0i8.i32` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memmove32<T>(dst: *mut T, src: *T, count: u32);
    /// Equivalent to the `llvm.memmove.p0i8.0i8.i64` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memmove64<T>(dst: *mut T, src: *T, count: u64);

    /// Equivalent to the `llvm.memset.p0i8.i32` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memset32<T>(dst: *mut T, val: u8, count: u32);
    /// Equivalent to the `llvm.memset.p0i8.i64` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memset64<T>(dst: *mut T, val: u8, count: u64);

    pub fn sqrtf32(x: f32) -> f32;
    pub fn sqrtf64(x: f64) -> f64;

    pub fn powif32(a: f32, x: i32) -> f32;
    pub fn powif64(a: f64, x: i32) -> f64;

    // the following kill the stack canary without
    // `fixed_stack_segment`. This possibly only affects the f64
    // variants, but it's hard to be sure since it seems to only
    // occur with fairly specific arguments.
    #[fixed_stack_segment]
    pub fn sinf32(x: f32) -> f32;
    #[fixed_stack_segment]
    pub fn sinf64(x: f64) -> f64;

    #[fixed_stack_segment]
    pub fn cosf32(x: f32) -> f32;
    #[fixed_stack_segment]
    pub fn cosf64(x: f64) -> f64;

    #[fixed_stack_segment]
    pub fn powf32(a: f32, x: f32) -> f32;
    #[fixed_stack_segment]
    pub fn powf64(a: f64, x: f64) -> f64;

    #[fixed_stack_segment]
    pub fn expf32(x: f32) -> f32;
    #[fixed_stack_segment]
    pub fn expf64(x: f64) -> f64;

    pub fn exp2f32(x: f32) -> f32;
    pub fn exp2f64(x: f64) -> f64;

    pub fn logf32(x: f32) -> f32;
    pub fn logf64(x: f64) -> f64;

    pub fn log10f32(x: f32) -> f32;
    pub fn log10f64(x: f64) -> f64;

    pub fn log2f32(x: f32) -> f32;
    pub fn log2f64(x: f64) -> f64;

    pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    pub fn fmaf64(a: f64, b: f64, c: f64) -> f64;

    pub fn fabsf32(x: f32) -> f32;
    pub fn fabsf64(x: f64) -> f64;

    pub fn floorf32(x: f32) -> f32;
    pub fn floorf64(x: f64) -> f64;

    pub fn ceilf32(x: f32) -> f32;
    pub fn ceilf64(x: f64) -> f64;

    pub fn truncf32(x: f32) -> f32;
    pub fn truncf64(x: f64) -> f64;

    pub fn ctpop8(x: i8) -> i8;
    pub fn ctpop16(x: i16) -> i16;
    pub fn ctpop32(x: i32) -> i32;
    pub fn ctpop64(x: i64) -> i64;

    pub fn ctlz8(x: i8) -> i8;
    pub fn ctlz16(x: i16) -> i16;
    pub fn ctlz32(x: i32) -> i32;
    pub fn ctlz64(x: i64) -> i64;

    pub fn cttz8(x: i8) -> i8;
    pub fn cttz16(x: i16) -> i16;
    pub fn cttz32(x: i32) -> i32;
    pub fn cttz64(x: i64) -> i64;

    pub fn bswap16(x: i16) -> i16;
    pub fn bswap32(x: i32) -> i32;
    pub fn bswap64(x: i64) -> i64;
}

#[cfg(target_endian = "little")] pub fn to_le16(x: i16) -> i16 { x }
#[cfg(target_endian = "big")]    pub fn to_le16(x: i16) -> i16 { unsafe { bswap16(x) } }
#[cfg(target_endian = "little")] pub fn to_le32(x: i32) -> i32 { x }
#[cfg(target_endian = "big")]    pub fn to_le32(x: i32) -> i32 { unsafe { bswap32(x) } }
#[cfg(target_endian = "little")] pub fn to_le64(x: i64) -> i64 { x }
#[cfg(target_endian = "big")]    pub fn to_le64(x: i64) -> i64 { unsafe { bswap64(x) } }

#[cfg(target_endian = "little")] pub fn to_be16(x: i16) -> i16 { unsafe { bswap16(x) } }
#[cfg(target_endian = "big")]    pub fn to_be16(x: i16) -> i16 { x }
#[cfg(target_endian = "little")] pub fn to_be32(x: i32) -> i32 { unsafe { bswap32(x) } }
#[cfg(target_endian = "big")]    pub fn to_be32(x: i32) -> i32 { x }
#[cfg(target_endian = "little")] pub fn to_be64(x: i64) -> i64 { unsafe { bswap64(x) } }
#[cfg(target_endian = "big")]    pub fn to_be64(x: i64) -> i64 { x }
