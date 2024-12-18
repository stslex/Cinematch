use diesel::{
    r2d2::{self, ConnectionManager, PooledConnection, R2D2Connection},
    PgConnection,
};

mod database;
mod pool;
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
}

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub trait DataPool<C, E>
where
    C: R2D2Connection + 'static,
{
    fn safe_get(self) -> Result<PooledConnection<ConnectionManager<C>>, E>;
}

pub trait DataPoolConnection<P, R, E>
where
    P: R2D2Connection + 'static,
    R: std::marker::Send + 'static,
{
    async fn safely_run<A>(self, action: A) -> Result<R, E>
    where
        A: FnOnce(&mut P) -> Result<R, E> + Send + 'static;
}
