# example_u32_sum

substrate入门第4课作业 

```rust
fn main() {
    let input = &[23, 12, 33];
    let result = sum(input);
    assert_eq!(result.is_some(), true);
    println!("the sum result is: {:?}", result);

    let input = &[23, 12, 33, u32::MAX];
    let result = sum(input);
    assert_eq!(result.is_none(), true);
    println!("the sum result is: {:?}", result);
}
```

