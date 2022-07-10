# example_u32_sum

substrate入门第4课作业 

实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None。

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

![](https://github.com/rustbomber/example_u32_sum/blob/3fe1bb10844034994a780a6d7d174d74094c9c69/screen.png)