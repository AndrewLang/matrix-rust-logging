
mod post;

#[cfg(test)]
mod post_tests {

    #[test]
    fn create_post_test() {
        let post = post::new();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn is_valid_test() {
        let post = post::new();
        assert_eq!(true, post.is_valid());
    }
}