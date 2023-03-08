# IC UUID Generator library

> Modified UUID lib to work with the IC. [Original here](https://github.com/BitFields/rs-uuid)
## Examples

```rust
use dl_uuid::{set_seed_u64, uuid16};

set_seed_u64(123);
let id_16byte = uuid16();

// 1859d40f-2c26-22ce-da23-33be037553d0
println!("{}", id_16byte);
```
