pub mod monocycle_permutation;

#[cfg(test)]
mod tests {
    #[test]
    fn monocycle_encryption_test_1() {
        let key = vec![3, 1, 2];
        let message = vec!['s', 'o', 'm', 'e', ' ', 't', 'e', 'x', 't'];
        let cipher = crate::monocycle_permutation::MonocyclePermutation::with_key(&key);
        let expected = vec!['m', 's', 'o', 't', 'e', ' ', 't', 'e', 'x'];
        let actual = cipher.encrypt(&message);
        assert_eq!(expected, actual);
    }

    #[test]
    fn monocycle_encryption_test_2() {
        let key = vec![3, 1, 2];
        let message = vec!['s', 'o', 'm', 'e', ' ', 't', 'e', 'x'];
        let cipher = crate::monocycle_permutation::MonocyclePermutation::with_key(&key);
        let expected = vec!['m', 's', 'o', 't', 'e', ' ', 'e', 'x'];
        let actual = cipher.encrypt(&message);
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn monocycle_decryption_test_1() {
        let key = vec![3, 1, 2];
        let expected = vec!['s', 'o', 'm', 'e', ' ', 't', 'e', 'x', 't'];
        let cipher = crate::monocycle_permutation::MonocyclePermutation::with_key(&key);
        let message = vec!['m', 's', 'o', 't', 'e', ' ', 't', 'e', 'x'];
        let actual = cipher.decrypt(&message);
        assert_eq!(expected, actual);
    }

    #[test]
    fn monocycle_decryption_test_2() {
        let key = vec![3, 1, 2];
        let expected = vec!['s', 'o', 'm', 'e', ' ', 't', 'e', 'x'];
        let cipher = crate::monocycle_permutation::MonocyclePermutation::with_key(&key);
        let message = vec!['m', 's', 'o', 't', 'e', ' ', 'e', 'x'];
        let actual = cipher.decrypt(&message);
        assert_eq!(expected, actual);
    }
}
