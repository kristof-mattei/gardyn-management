use color_eyre::eyre;
use url::Url;

#[allow(dead_code)]
/// Adds a segment to a Url
/// # Errors
/// When the Url given is relative
pub fn add_segments(mut base_url: Url, segments: &[&str]) -> Result<Url, eyre::Report> {
    {
        let mut s = base_url
            .path_segments_mut()
            .map_err(|()| eyre::Report::msg("Url is not a base"))?;
        s.extend(segments);
    }

    Ok(base_url)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use url::Url;

    use crate::utils::url::add_segments;

    #[test]
    fn test_add_single_segment() {
        let url = Url::from_str("https://example.com").unwrap();

        let new_url = add_segments(url, &["foobar"]);

        assert!(new_url.is_ok());
        assert_eq!(new_url.unwrap().as_str(), "https://example.com/foobar");
    }

    #[test]
    fn test_multple_segments() {
        let url = Url::from_str("https://example.com").unwrap();

        let new_url = add_segments(url, &["foo", "bar"]);

        assert!(new_url.is_ok());
        assert_eq!(new_url.unwrap().as_str(), "https://example.com/foo/bar");
    }
}
