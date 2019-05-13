use std::env;
use telegram_bot_fork::Api;

/// create_telegram_client returns an API object that functions
/// as the telegram client. If there's an error it will panic
/// and stop the program's executuion.
///
/// # Arguments
/// * None
///
/// # Examples
/// ```
/// use clients::create_telegram_client;
/// let telegram_client = create_telegram_client
/// ```
pub fn create_telegram_client() -> Api {
    let telegram_token = env::var("TELEGRAM_API_KEY").expect("TELEGRAM_API_KEY not set");
    match Api::new(telegram_token) {
        Ok(api) => api,
        Err(err) => panic!(
            "Unable to connect to telegram, reason: {error}",
            error = err
        ),
    }
}
