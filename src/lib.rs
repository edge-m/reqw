//! This crate is intended for documentation purposes only.
//! Please copy this snippet into your project instead of using it as a dependency.

pub enum Error {
    Http(reqwest::Response),
    Transport(reqwest::Error),
}

/// This function is re-estimates response of reqwest
///
/// - Returns `Ok(reqwest::Response)` if status code is 200-299
/// - Returns `Err(Error::Http)` if the status code is not 2xx
/// - Returns `Err(Error::Transport)` if there was a transport error
///
/// # Examples
///
/// ```rust
/// async fn run() {
///     let result = reqw::est(reqwest::get("https://example.com").await);
///     assert!(result.is_ok());
/// }
/// ```
pub fn est(result: Result<reqwest::Response, reqwest::Error>) -> Result<reqwest::Response, Error> {
    match result {
        Ok(r) if r.status().is_success() => Ok(r),
        Ok(r) => Err(Error::Http(r)),
        Err(e) => Err(Error::Transport(e)),
    }
}
