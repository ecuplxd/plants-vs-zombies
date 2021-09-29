// https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust

use core::ffi::c_void;
use core::mem::transmute;
use core::ptr::null_mut;
use std::marker::PhantomData;

pub struct ErasedFnPointer<T, Ret = ()> {
    struct_pointer: *mut c_void,
    fp: *const (),
    phantom_sp: PhantomData<()>,
    phantom_fp: PhantomData<fn(T) -> Ret>,
}

impl<T, Ret> Copy for ErasedFnPointer<T, Ret> {}
impl<T, Ret> Clone for ErasedFnPointer<T, Ret> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, Ret> ErasedFnPointer<T, Ret> {
    pub fn from_associated<S>(
        struct_pointer: &mut S,
        fp: fn(&mut S, T) -> Ret,
    ) -> ErasedFnPointer<T, Ret> {
        ErasedFnPointer {
            struct_pointer: struct_pointer as *mut _ as *mut c_void,
            fp: fp as *const (),
            phantom_sp: PhantomData,
            phantom_fp: PhantomData,
        }
    }

    pub fn from_free(fp: fn(T) -> Ret) -> ErasedFnPointer<T, Ret> {
        ErasedFnPointer {
            struct_pointer: null_mut(),
            fp: fp as *const (),
            phantom_sp: PhantomData,
            phantom_fp: PhantomData,
        }
    }

    pub fn call(&self, param: T) -> Ret {
        if self.struct_pointer.is_null() {
            let fp = unsafe { transmute::<_, fn(T) -> Ret>(self.fp) };
            fp(param)
        } else {
            let fp = unsafe { transmute::<_, fn(*mut c_void, T) -> Ret>(self.fp) };
            fp(self.struct_pointer, param)
        }
    }
}
