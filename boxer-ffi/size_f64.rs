use boxer::CBox;
use boxer::size::{BoxerSizeF64 };

#[no_mangle]
pub fn boxer_size_f64_create() -> *mut BoxerSizeF64 {
    CBox::into_raw(BoxerSizeF64::default())
}

#[no_mangle]
pub fn boxer_size_f64_get_width(_size_ptr: *mut BoxerSizeF64) -> f64 {
    CBox::with_raw(_size_ptr, |size| size.width )
}

#[no_mangle]
pub fn boxer_size_f64_set_width(_size_ptr: *mut BoxerSizeF64, width: f64) {
    CBox::with_raw(_size_ptr, |size| size.width = width )
}

#[no_mangle]
pub fn boxer_size_f64_get_height(_size_ptr: *mut BoxerSizeF64) -> f64 {
    CBox::with_raw(_size_ptr, |size| size.height )
}

#[no_mangle]
pub fn boxer_size_f64_set_height(_size_ptr: *mut BoxerSizeF64, height: f64) {
    CBox::with_raw(_size_ptr, |size| size.height = height )
}

#[no_mangle]
pub fn boxer_size_f64_drop(_ptr: *mut BoxerSizeF64) {
    CBox::drop(_ptr)
}