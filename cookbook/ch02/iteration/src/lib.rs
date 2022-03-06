#[cfg(test)]
mod tests {
    #[test]
    fn getting_the_iterator() {
        let v = vec![10, 10, 10];
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), None);

        for i in v {
            assert_eq!(i, 10);
        }
    }

    fn count_files(path: &String) -> usize {
        path.len()
    }

    #[test]
    fn data_transformations() {
        let v = vec![10, 10, 10];
        let hexed = v.iter().map(|i| format!("{:x}", i));
        assert_eq!(
            hexed.collect::<Vec<String>>(),
            vec!["a".to_string(), "a".to_string(), "a".to_string()]
        );
        // (((0 + 10) + 10) + 10)
        assert_eq!(v.iter().fold(0, |p, c| p + c), 30);

        let dirs = vec![
            "/home/alice".to_string(),
            "/home/bob".to_string(),
            "/home/carl".to_string(),
            "/home/debra".to_string(),
        ];
        let file_counter = dirs.iter().map(count_files);
        let dir_file_counts: Vec<(&String, usize)> = dirs.iter().zip(file_counter).collect();
        assert_eq!(
            dir_file_counts,
            vec![
                (&"/home/alice".to_string(), 11),
                (&"/home/bob".to_string(), 9),
                (&"/home/carl".to_string(), 10),
                (&"/home/debra".to_string(), 11)
            ]
        );
    }

    #[test]
    fn data_filtering() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert!(data.iter().filter(|&n| n % 2 == 0).all(|&n| n % 2 == 0));
        assert_eq!(data.iter().find(|&&n| n == 5), Some(&5));
        assert_eq!(data.iter().find(|&&n| n == 0), None);
        assert_eq!(data.iter().position(|&n| n == 5), Some(4));
        assert_eq!(data.iter().skip(1).next(), Some(&2));
        
        let mut data_iter = data.iter().take(2);
        assert_eq!(data_iter.next(), Some(&1));
        assert_eq!(data_iter.next(), Some(&2));
        assert_eq!(data_iter.next(), None);

        // validation: true, train: false
        let (validation, train): (Vec<i32>, Vec<i32>) = data
            .iter()
            .partition(|&_| (rand::random::<f32>() % 1.0) > 0.8);

        assert!(train.len() > validation.len());
    }
}
