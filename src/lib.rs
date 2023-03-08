use std::io::Cursor;
use std::sync::{Arc, Mutex};

use arc_swap::ArcSwap;
use byteorder::{BigEndian, ReadBytesExt};
use hex::encode;
use nanorand::{Rng, WyRand};

thread_local! {
    static RNG : ArcSwap<Mutex<WyRand>> = ArcSwap::new(Arc::new(Mutex::new(WyRand::new_seed(11233429492))));
}

pub fn set_seed(seed: [u8; 32]) {
    let mut seed = Cursor::new(seed.to_vec());
    let seed = seed.read_u64::<BigEndian>().unwrap();
    set_seed_u64(seed)
}

pub fn set_seed_u64(seed: u64) {
    RNG.with(|v| {
        v.swap(Arc::new(Mutex::new(WyRand::new_seed(seed))));
    })
}

/// Generates 16-byte UUID as a String
pub fn uuid16() -> String {
    let bytes = RNG.with(|rng| {
        let rng = rng.load();
        let mut rng = rng.lock().unwrap();
        return (
            encode([
                rng.generate::<u8>(),
                rng.generate::<u8>(),
                rng.generate::<u8>(),
                rng.generate::<u8>(),
            ]), //4
            encode([rng.generate::<u8>(), rng.generate::<u8>()]), //2
            encode([rng.generate::<u8>(), rng.generate::<u8>()]), //2
            encode([rng.generate::<u8>(), rng.generate::<u8>()]), //2
            encode([
                rng.generate::<u8>(),
                rng.generate::<u8>(),
                rng.generate::<u8>(),
                rng.generate::<u8>(),
                rng.generate::<u8>(),
                rng.generate::<u8>(),
            ]), //6
        );
    });

    format!(
        "{}-{}-{}-{}-{}",
        bytes.0, bytes.1, bytes.2, bytes.3, bytes.4
    )
}

#[test]
fn test_uuid16() {
    let id: String = uuid16();
    println!("{}", id);
    // 16 * 2 bytes + 4 dashes
    assert_eq!(id.len(), 36);
}

// Check against known value
#[test]
fn test_set_seed_u64() {
    set_seed_u64(123);
    let id: String = uuid16();
    assert_eq!("1859d40f-2c26-22ce-da23-33be037553d0", id)
}

// Check against known value
#[test]
fn test_set_seed_bytes() {
    set_seed([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

    let id: String = uuid16();
    assert_eq!("8e6da460-de1f-29a2-570a-7e8e3ac51a99", id)
}
