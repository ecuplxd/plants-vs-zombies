// https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust

use core::ffi::c_void;
use core::mem::transmute;
use core::ptr::null_mut;

pub struct ErasedFnPointer {
    struct_pointer: *mut c_void,
    fp: *const (),
}

impl Copy for ErasedFnPointer {}

impl Clone for ErasedFnPointer {
    fn clone(&self) -> Self {
        *self
    }
}

impl ErasedFnPointer {
    pub fn from_associated<S>(struct_pointer: &mut S, fp: fn(&mut S)) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: struct_pointer as *mut _ as *mut c_void,
            fp: fp as *const (),
        }
    }

    pub fn from_free(fp: fn()) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: null_mut(),
            fp: fp as *const (),
        }
    }

    pub fn call(&self) {
        if self.struct_pointer.is_null() {
            let fp = unsafe { transmute::<_, fn()>(self.fp) };
            fp()
        } else {
            let fp = unsafe { transmute::<_, fn(*mut c_void)>(self.fp) };
            fp(self.struct_pointer)
        }
    }
}
