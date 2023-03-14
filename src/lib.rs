use std::io::Cursor;
use std::sync::{Arc, Mutex};

use arc_swap::ArcSwap;
use byteorder::{BigEndian, ReadBytesExt};
use hex::encode;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::{RngCore, SeedableRng};

thread_local! {
    static RNG : ArcSwap<Mutex<ChaCha20Rng>> = ArcSwap::new(Arc::new(Mutex::new(ChaCha20Rng::seed_from_u64(11233429492))));
}

pub fn set_seed(seed: [u8; 32]) {
    let mut seed = Cursor::new(seed.to_vec());
    let seed = seed.read_u64::<BigEndian>().unwrap();
    set_seed_u64(seed)
}

pub fn set_seed_u64(seed: u64) {
    RNG.with(|v| {
        v.swap(Arc::new(Mutex::new(ChaCha20Rng::seed_from_u64(seed))));
    })
}

/// Generates 16-byte UUID as a String
pub fn uuid16() -> String {
    let bytes = RNG.with(|rng| {
        let rng = rng.load();
        let mut rng = rng.lock().unwrap();

        let mut part_one: [u8; 4] = Default::default();
        let mut part_two: [u8; 2] = Default::default();
        let mut part_three: [u8; 2] = Default::default();
        let mut part_four: [u8; 2] = Default::default();
        let mut part_five: [u8; 6] = Default::default();

        rng.fill_bytes(&mut part_one);

        rng.fill_bytes(&mut part_two);
        rng.fill_bytes(&mut part_three);
        rng.fill_bytes(&mut part_four);
        rng.fill_bytes(&mut part_five);
        return (
            encode(part_one), //4
            encode(part_two), //2
            encode(part_three), //2
            encode(part_four), //2
            encode(part_five), //6
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
    assert_eq!("d6daf4ca-bee9-f9f9-4b53-f7f1d8e813e4", id)
}

// Check against known value
#[test]
fn test_set_seed_bytes() {
    set_seed([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

    let id: String = uuid16();
    assert_eq!("b2f7f581-d6de-a822-7e82-c00f8401696a", id)
}
