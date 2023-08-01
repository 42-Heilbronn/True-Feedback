use diesel::{ConnectionResult, ConnectionError};
use diesel_async::pooled_connection::PoolError;
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};
use diesel_async::AsyncPgConnection;
use futures_util::FutureExt;
use futures_util::future::BoxFuture;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<AsyncPgConnection>,
}

fn establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        // We first set up the way we want rustls to work.
        let rustls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_certs())
            .with_no_client_auth();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Database connection: {e}");
            }
        });
        AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

fn root_certs() -> rustls::RootCertStore {
    let mut roots = rustls::RootCertStore::empty();
    let certs = rustls_native_certs::load_native_certs().expect("Certs not loadable!");
    let certs: Vec<_> = certs.into_iter().map(|cert| cert.0).collect();
    roots.add_parsable_certificates(&certs);
    roots
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, PoolError> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_setup(database_url, establish_connection);
        let pool = Pool::builder().build(config).await?;
        Ok(Database { pool })
    }
}
