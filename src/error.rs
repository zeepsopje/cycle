#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Timer is not running")]
    TimerNotRunning,
}
