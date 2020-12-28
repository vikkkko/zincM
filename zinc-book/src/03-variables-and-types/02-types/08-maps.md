# Maps

The `std::collections::MTreeMap` is a special type, which can only be used as a smart contract
storage field:

```rust,no_run,noplaypen
use std::collections::MTreeMap;

struct Data {
    a: u8,
    b: u8,
}

contract Test {
    data: MTreeMap<u8, Data>;

    pub fn example(mut self) {
        let (old1, existed1) = self.data.insert(42, Data { a: 16, b: 9 });
        let (value, exists1) = self.data.get(42);
        let exists2 = self.data.contains(42);
        let (old2, existed2) = self.data.remove(42);
    }
}
```

> The maps introduce a new concept of generic types, but this feature can
> only be used to specify the key and value types for the `MTreeMap` instance.

The full description of the `MTreeMap` methods is [here](../../appendix/E-standard-library.md#stdcollectionsmtreemapk-v).
