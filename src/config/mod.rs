pub mod database;
mod sert;
mod state;

pub struct AppState<'a> {
    pub app_name: &'a str,
}

pub trait AppStateConfig {
    fn bind_app_state(self) -> Self;
    #[cfg(test)]
    fn bind_app_state_for_tests(self) -> Self;
}

pub trait ServiceSertConfig {
    fn bind_server_ssl(self) -> Self;
    fn bind_simple_server(self) -> Self;
}
