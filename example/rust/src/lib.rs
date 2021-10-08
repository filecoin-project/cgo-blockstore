use ::cgobs::sys;

#[no_mangle]
pub extern "C" fn write_a_block(store: i32) -> i32 {
    let cid = "foobar";
    let block = "data";
    unsafe {
        return sys::cgobs_put(
            store,
            cid.as_ptr(),
            cid.len() as i32,
            block.as_ptr(),
            block.len() as i32,
        );
    }
}
