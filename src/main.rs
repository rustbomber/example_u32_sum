fn sum(input: &[u32]) -> Option<u32> {
    let mut count = 0_u64;
    for item in input {
        count += *item as u64;
    }

    if count > u32::MAX as u64 {
        return None;
    }

    Some(count as u32)


    // 方式二：
    // let mut count = 0_u32;
    // let mut is_overflow =false;
    // for item in input {
    //     let a=count.checked_add(*item);
    //     if let Some(x) = a {
    //         count= x;
    //     }else{
    //         is_overflow= true;
    //         break;
    //     }
    // }

    // if is_overflow{
    //     None
    // }else{
    //     Some(count)
    // }
}

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
