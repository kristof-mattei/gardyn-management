use color_eyre::Report;
use color_eyre::eyre::Context;
use url::Url;

/// Gets an environment vaiable and tries to convert it to a Url
///
/// # Errors
/// When the variable could not be converted to a Url
pub fn get_env_as_url(key: &str) -> Result<Url, Report> {
    let value = std::env::var(key)?;

    Url::parse(&value).wrap_err_with(|| format!("Couldn't convert {:?} to URL", value))
}
