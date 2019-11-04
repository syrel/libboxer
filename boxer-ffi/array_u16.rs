use boxer::array::BoxerArrayU16;
use boxer::boxes::ValueBox;

#[no_mangle]
pub fn boxer_array_u16_create() -> *mut ValueBox<BoxerArrayU16> {
    BoxerArrayU16::boxer_array_create()
}

#[no_mangle]
pub fn boxer_array_u16_create_with(element: u16, amount: usize) -> *mut ValueBox<BoxerArrayU16> {
    BoxerArrayU16::boxer_array_create_with(element, amount)
}

#[no_mangle]
pub fn boxer_array_u16_get_length(_ptr: *mut ValueBox<BoxerArrayU16>) -> usize {
    BoxerArrayU16::boxer_array_get_length(_ptr)
}

#[no_mangle]
pub fn boxer_array_u16_get_capacity(_ptr: *mut ValueBox<BoxerArrayU16>) -> usize {
    BoxerArrayU16::boxer_array_get_capacity(_ptr)
}

#[no_mangle]
pub fn boxer_array_u16_get_data(_ptr: *mut ValueBox<BoxerArrayU16>) -> *mut u16 {
    BoxerArrayU16::boxer_array_get_data(_ptr)
}

#[no_mangle]
pub fn boxer_array_u16_drop(_ptr: *mut ValueBox<BoxerArrayU16>) {
    BoxerArrayU16::boxer_array_drop(_ptr);
}