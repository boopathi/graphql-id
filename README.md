# graphql-id

[![Build Status](https://travis-ci.com/boopathi/graphql-id.svg?branch=master)](https://travis-ci.com/boopathi/graphql-id) [![Latest version](https://img.shields.io/crates/v/graphql-id.svg)](https://crates.io/crates/graphql-id) [![Documentation](https://docs.rs/graphql-id/badge.svg)](https://docs.rs/graphql-id)

Generate ID for a GraphQL Query.

## Usage

### lib

```rust
extern crate graphql_id;

use graphql_id::{generate_operation_id, generate_default_operation_id};

fn main() {
  let query = "
    query petSearch($name: String!) {
      search(name: $name) {
        ... on Node {
          id
        }
        category {
          ...walk
        }
      }
    }
    fragment walk on Walk {
      speed
    }
  ";

  println!("{}", generate_operation_id(&query, &"petSearch"));
}
```

### bin

```sh
graphql-id path/to/query.graphql operationName
```

# License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
