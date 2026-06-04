/// Sanitize a string for use in file/directory names.
/// Replaces characters that are invalid or problematic in filesystem paths with underscores.
pub fn sanitize(s: &str) -> String {
    s.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_")
}

#[cfg(test)]
mod tests {
    use super::sanitize;

    #[test]
    fn no_special_chars() {
        assert_eq!(sanitize("normal string"), "normal string");
    }

    #[test]
    fn slashes() {
        assert_eq!(sanitize("path/to/file"), "path_to_file");
        assert_eq!(sanitize("path\\to\\file"), "path_to_file");
    }

    #[test]
    fn colon() {
        assert_eq!(sanitize("title: subtitle"), "title_ subtitle");
    }

    #[test]
    fn all_special_chars() {
        let input = "/:*?\"<>|";
        let expected = "________";
        assert_eq!(sanitize(input), expected);
    }

    #[test]
    fn mixed_content() {
        assert_eq!(sanitize("Artist / Band: Vol. 1"), "Artist _ Band_ Vol. 1");
    }

    #[test]
    fn empty_string() {
        assert_eq!(sanitize(""), "");
    }

    #[test]
    fn unicode() {
        assert_eq!(sanitize("Artisté – Album™"), "Artisté – Album™");
    }

    #[test]
    fn leading_trailing_special_chars() {
        let result = sanitize("/path/");
        assert!(result.starts_with('_'));
        assert!(result.ends_with('_'));
    }

    #[test]
    fn consecutive_special_chars() {
        assert_eq!(sanitize("a//b"), "a__b");
    }

    #[test]
    fn only_special_chars() {
        assert_eq!(sanitize("///"), "___");
    }
}
