use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

pub mod point;
pub mod size;

pub struct CBox {}

impl CBox {
    pub fn into_raw<T> (object: T) -> *mut T {
        Box::into_raw(Box::new(object))
    }

    pub fn from_raw<T>(pointer: *mut T) -> Box<T> {
        assert_eq!(pointer.is_null(), false, "CBox::from_raw(): Pointer must not be null!");
        unsafe { Box::from_raw(pointer) }
    }

    pub fn drop<T>(pointer: *mut T) {
        if pointer.is_null() {
            return;
        }
        CBox::from_raw(pointer);
    }

    pub fn to_string(_ptr_chars: *const c_char) -> String {
        let title_string = unsafe {
            CStr::from_ptr(_ptr_chars).to_string_lossy().into_owned()
        };
        title_string
    }

    pub fn to_chars<T: Into<Vec<u8>>>(_data: T) -> *mut c_char {
        let c_to_print = CString::new(_data).expect("CString::new failed");
        c_to_print.into_raw()
    }

    pub fn free_chars(_ptr_contents: *mut c_char) {
        unsafe { CString::from_raw(_ptr_contents) };
    }

    pub fn with_raw<F, R, T>(pointer: *mut T, block: F) -> R where F : FnOnce(&mut Box<T>) -> R {
        assert_eq!(pointer.is_null(), false, "CBox::with_raw(): Pointer must not be null!");

        let mut boxed_object: Box<T> = Self::from_raw(pointer);
        let result: R = block(&mut boxed_object);
        Self::into_raw(boxed_object);
        result
    }

    pub fn with_two_raw<F, R, T1, T2>(pointer_1: *mut T1, pointer_2: *mut T2, block: F) -> R where F : FnOnce(&mut Box<T1>, &mut Box<T2>) -> R {
        assert_eq!(pointer_1.is_null(), false, "CBox::with_raw(): Pointer #1 must not be null!");
        assert_eq!(pointer_2.is_null(), false, "CBox::with_raw(): Pointer #2 must not be null!");

        let mut boxed_object_1: Box<T1> = Self::from_raw(pointer_1);
        let mut boxed_object_2: Box<T2> = Self::from_raw(pointer_2);
        let result: R = block(&mut boxed_object_1, &mut boxed_object_2);
        Self::into_raw(boxed_object_1);
        Self::into_raw(boxed_object_2);
        result
    }

    pub fn with_three_raw<F, R, T1, T2, T3>(pointer_1: *mut T1, pointer_2: *mut T2, pointer_3: *mut T3, block: F)
                                            -> R where F : FnOnce(&mut Box<T1>, &mut Box<T2>, &mut Box<T3>) -> R {
        assert_eq!(pointer_1.is_null(), false, "CBox::with_raw(): Pointer #1 must not be null!");
        assert_eq!(pointer_2.is_null(), false, "CBox::with_raw(): Pointer #2 must not be null!");
        assert_eq!(pointer_3.is_null(), false, "CBox::with_raw(): Pointer #3 must not be null!");

        let mut boxed_object_1: Box<T1> = Self::from_raw(pointer_1);
        let mut boxed_object_2: Box<T2> = Self::from_raw(pointer_2);
        let mut boxed_object_3: Box<T3> = Self::from_raw(pointer_3);
        let result: R = block(&mut boxed_object_1, &mut boxed_object_2, &mut boxed_object_3);
        Self::into_raw(boxed_object_1);
        Self::into_raw(boxed_object_2);
        Self::into_raw(boxed_object_3);
        result
    }

    pub fn with_consumable_raw<F, T>(_ptr_consumable: *mut T, block: F) -> *mut T where F : FnOnce(T) -> T {
        let consumable = *CBox::from_raw(_ptr_consumable);
        let modified_consumable = block(consumable);
        CBox::into_raw(modified_consumable)
    }
}