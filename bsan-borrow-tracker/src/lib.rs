#![feature(rustc_private)]
#![warn(clippy::pedantic)]
#![allow(unused)]

extern crate rustc_abi;
extern crate rustc_apfloat;
extern crate rustc_ast;
extern crate rustc_attr;
extern crate rustc_const_eval;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

use core::ffi::c_void;
use std::num::NonZero;
use std::sync::OnceLock;

use log::debug;
use miri::borrow_tracker::{PlaceKind, ProtectorKind};
use miri::{AllocId, BorTag, Pointer, Provenance, ProvenanceExtra};
use rustc_abi::Size;
use rustc_middle::mir::RetagKind;

mod state;
use state::GlobalState;

static BSAN_GLOBAL: OnceLock<GlobalState> = OnceLock::new();

#[repr(C)]
#[derive(Debug)]
pub struct TrackedPointer {
    addr: *mut c_void,
    alloc_id: u64,
    // the borrow tag 0 indicates wildcard provenance.
    tag: u64,
}
impl TrackedPointer {
    fn to_miri_pointer(pointer: TrackedPointer) -> Pointer {
        let addr = Size::from_bytes(pointer.addr as u64);
        let alloc_id = pointer.alloc_id;
        let tag = pointer.tag;
        if addr == Size::ZERO {
            Pointer::null()
        } else {
            let provenance = if alloc_id == 0 {
                None
            } else {
                let extra = if tag == 0 {
                    Provenance::Wildcard
                } else {
                    let alloc_id = unsafe { AllocId(NonZero::new_unchecked(alloc_id)) };
                    let tag = BorTag::new(tag).unwrap();
                    unsafe { Provenance::Concrete { alloc_id, tag } }
                };
                Some(extra)
            };
            Pointer::new(provenance, addr)
        }
    }
}

#[no_mangle]
extern "C" fn __bsan_init() {
    BSAN_GLOBAL.get_or_init(GlobalState::default);
    debug!("Initialized global state");
}

#[no_mangle]
extern "C" fn __bsan_expose_tag(ptr: TrackedPointer) -> u64 {
    todo!();
}

#[no_mangle]
extern "C" fn __bsan_retag(ptr: TrackedPointer, place_kind: PlaceKind) -> u64 {
    todo!()
}

#[no_mangle]
extern "C" fn __bsan_retag_fn(
    ptr: TrackedPointer,
    protector_kind: ProtectorKind,
    place_kind: PlaceKind,
) -> u64 {
    todo!()
}

#[no_mangle]
extern "C" fn __bsan_read(ptr: TrackedPointer, access_size: u64) -> u64 {
    todo!()
}

#[no_mangle]
extern "C" fn __bsan_write(ptr: TrackedPointer, access_size: u64) -> u64 {
    todo!()
}

#[no_mangle]
extern "C" fn __bsan_dealloc(ptr: TrackedPointer) -> u64 {
    todo!()
}

#[no_mangle]
extern "C" fn __bsan_alloc(addr: *mut c_void) -> TrackedPointer {
    todo!()
}

#[no_mangle]
extern "C" fn __bsan_push_stack_frame() {
    todo!()
}

#[no_mangle]
extern "C" fn __bsan_pop_stack_frame() {
    todo!()
}