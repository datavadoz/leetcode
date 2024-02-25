use leetcode::leet_00001;

#[test]
fn test_two_sum_1_1() {
    let mut actual = leet_00001::Solution::two_sum_1(vec![2, 7, 11, 15], 9);
    let mut expected = vec![0, 1];
    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual.sort(), expected.sort());
}

#[test]
fn test_two_sum_1_2() {
    let actual = leet_00001::Solution::two_sum_1(vec![2, 7, 11, 15], 100);
    assert_eq!(actual.len(), 0);
}

#[test]
fn test_two_sum_2_1() {
    let mut actual = leet_00001::Solution::two_sum_2(vec![2, 7, 11, 15], 9);
    let mut expected = vec![0, 1];
    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual.sort(), expected.sort());
}

#[test]
fn test_two_sum_2_2() {
    let actual = leet_00001::Solution::two_sum_2(vec![2, 7, 11, 15], 100);
    assert_eq!(actual.len(), 0);
}
