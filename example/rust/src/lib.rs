use cgobs::Blockstore;
use cid::multihash::{Code, MultihashDigest};
use cid::Cid;

#[no_mangle]
pub extern "C" fn write_blocks(store: i32, count: i32) -> i32 {
    let bs = unsafe { Blockstore::new(store) };
    let block = b"thing";
    let key = Cid::new_v1(0x55, Code::Sha2_256.digest(block));
    for b in 0..count {
        if let Err(_) = bs.put(&key, block) {
            return 1;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn read_blocks(store: i32, count: i32) -> i32 {
    let bs = unsafe { Blockstore::new(store) };
    let block = b"thing";
    let key = Cid::new_v1(0x55, Code::Sha2_256.digest(block));
    for b in 0..count {
        let _ = bs.get(&key);
    }
    0
}
