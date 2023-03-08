
fn set_seed() {
    dl_uuid::set_seed_u64(123);
    println!("{}", dl_uuid::uuid16())
}


fn set_seed_ic_random() {
    // swap for some randomness from the beacon
    dl_uuid::set_seed([
        1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    println!("{}", dl_uuid::uuid16())
}

fn main() {
    set_seed();
    set_seed_ic_random()
}