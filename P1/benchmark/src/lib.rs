pub fn mat_num_mul(mat: Vec<u8>, num: u8) -> Vec<u8> {
    mat.iter().map(|elem| elem.overflowing_mul(num).0).collect()
}

pub fn mat_mul(left: Vec<u8>, right: Vec<u8>) -> Vec<u8> {
    if left.len() != 9 || right.len() != 9 {
        panic!("Expecting 2 3x3 matrices")
    }
    let mut result = vec![0u8; 9];
    result[0] = left[0]
        .overflowing_mul(right[0]).0
        .overflowing_add(left[1].overflowing_mul(right[3]).0).0
        .overflowing_add(left[2].overflowing_mul(right[6]).0).0;
    result[1] = left[0]
        .overflowing_mul(right[1]).0
        .overflowing_add(left[1].overflowing_mul(right[4]).0).0
        .overflowing_add(left[2].overflowing_mul(right[7]).0).0;
    result[2] = left[0]
        .overflowing_mul(right[2]).0
        .overflowing_add(left[1].overflowing_mul(right[5]).0).0
        .overflowing_add(left[2].overflowing_mul(right[8]).0).0;

    result[3] = left[3]
        .overflowing_mul(right[0]).0
        .overflowing_add(left[4].overflowing_mul(right[3]).0).0
        .overflowing_add(left[5].overflowing_mul(right[6]).0).0;
    result[4] = left[3]
        .overflowing_mul(right[1]).0
        .overflowing_add(left[4].overflowing_mul(right[4]).0).0
        .overflowing_add(left[5].overflowing_mul(right[7]).0).0;
    result[5] = left[3]
        .overflowing_mul(right[2]).0
        .overflowing_add(left[4].overflowing_mul(right[5]).0).0
        .overflowing_add(left[5].overflowing_mul(right[8]).0).0;

    result[6] = left[6]
        .overflowing_mul(right[0]).0
        .overflowing_add(left[7].overflowing_mul(right[3]).0).0
        .overflowing_add(left[8].overflowing_mul(right[6]).0).0;
    result[7] = left[6]
        .overflowing_mul(right[1]).0
        .overflowing_add(left[7].overflowing_mul(right[4]).0).0
        .overflowing_add(left[8].overflowing_mul(right[7]).0).0;
    result[8] = left[6]
        .overflowing_mul(right[2]).0
        .overflowing_add(left[7].overflowing_mul(right[5]).0).0
        .overflowing_add(left[8].overflowing_mul(right[8]).0).0;

    result
}

pub fn det(mat: Vec<u8>) -> u8 {
    if mat.len() != 9 {
        panic!("Expecting 3x3 matrix")
    }

    mat[0]
        .overflowing_mul(
            mat[4]
                .overflowing_mul(mat[8])
                .0
                .overflowing_sub(mat[5].overflowing_mul(mat[7]).0)
                .0,
        )
        .0
        .overflowing_sub(
            mat[1]
                .overflowing_mul(
                    mat[3]
                        .overflowing_mul(mat[8])
                        .0
                        .overflowing_sub(mat[5].overflowing_mul(mat[6]).0)
                        .0,
                )
                .0,
        )
        .0
        .overflowing_sub(
            mat[2]
                .overflowing_mul(
                    mat[3]
                        .overflowing_mul(mat[7])
                        .0
                        .overflowing_sub(mat[4].overflowing_mul(mat[6]).0)
                        .0,
                )
                .0,
        )
        .0
}

pub fn bm(left: Vec<u8>, right: Vec<u8>, mul: u8) -> u8 {
    det(mat_mul(mat_num_mul(left, mul), right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bm_value() {
        let left = vec![20, 49, 24, 03, 08, 23, 60, 56, 17];
        let right = vec![51, 11, 01, 52, 39, 42, 07, 16, 14];
        let multiplier = 51;
        let result = bm(left, right, multiplier);
        assert_eq!(result, 65);
    }
}
