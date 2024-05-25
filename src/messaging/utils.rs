pub fn len_to_u8_arr(num: usize) -> [u8; 2] {
    let len_1 = (num & 255).try_into().unwrap();
    let len_2 = ((num >> 8) & 255).try_into().unwrap();
    return [len_1, len_2];
}

pub fn u8_arr_to_len(arr: [u8; 2]) -> usize {
    let len_1: usize = arr[0].into();
    let len_2: usize = arr[1].into();
    return (len_2 >> 8) + len_1;
}