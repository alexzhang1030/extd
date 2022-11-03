# extd

Fetch and extract HTML's title and description by given link.

## Usage

in `Cargo.toml`:

```toml
[dependencies]
extd = "0.1.4"
```

### Example

```rust
use extd::extract_td;

fn main() {

    let res = extract_td("https://www.rustlang.org/");
    match res {
        Ok(res) => println!("{:?}", res),
        _ => println!("otherwise"),
    }
}
```

return value:

```rust
pub struct ExtractResult {
    // title
    pub title: String,
    // description
    pub desc: String,
}
```
