use objc_test_utils;

use block::Block;
use Id;

pub fn get_int_block_with(i: i32) -> Id<Block<(), i32>> {
    unsafe {
        let ptr = objc_test_utils::get_int_block_with(i);
        Id::from_retained_ptr(ptr as *mut _)
    }
}

pub fn get_add_block_with(i: i32) -> Id<Block<(i32,), i32>> {
    unsafe {
        let ptr = objc_test_utils::get_add_block_with(i);
        Id::from_retained_ptr(ptr as *mut _)
    }
}

pub fn invoke_int_block(block: &mut Block<(), i32>) -> i32 {
    let ptr = block as *mut _;
    unsafe {
        objc_test_utils::invoke_int_block(ptr as *mut _)
    }
}

pub fn invoke_add_block(block: &mut Block<(i32,), i32>, a: i32) -> i32 {
    let ptr = block as *mut _;
    unsafe {
        objc_test_utils::invoke_add_block(ptr as *mut _, a)
    }
}