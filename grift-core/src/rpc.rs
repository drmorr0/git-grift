use std::fs;
use std::path::PathBuf;

use hyper_util::rt::TokioIo;
use tokio::net::{
    UnixListener,
    UnixStream,
};
use tokio_stream::wrappers::UnixListenerStream;
use tokio_util::sync::CancellationToken;
use tonic::transport::{
    Channel,
    Endpoint,
    Server,
};
use tower::service_fn;
use tracing::*;

use crate::prelude::*;

tonic::include_proto!("grift");

use grift_service_client::*;
use grift_service_server::*;

pub type GriftClient = GriftServiceClient<Channel>;

impl GriftClient {
    pub async fn build() -> eyre::Result<GriftClient> {
        // This URL gets thrown away immediately, but there's no option to not construct something
        // without a valid URL so I'm just gonna set it to example.com I guess
        let channel = Endpoint::try_from("http://example.com")?
            .connect_with_connector(service_fn(|_| async {
                let path = grift_socket()?;

                // Throw away the URI and connect to a UDS
                Ok::<_, eyre::Error>(TokioIo::new(UnixStream::connect(path).await?))
            }))
            .await?;

        Ok(GriftClient::new(channel))
    }
}

pub struct GriftServer<Svc: GriftService + Clone> {
    socket_path: PathBuf,
    svc: Svc,
}

impl<Svc: GriftService + Clone> GriftServer<Svc> {
    pub fn build(svc: Svc) -> eyre::Result<GriftServer<Svc>> {
        let socket_path = grift_socket()?;
        Ok(GriftServer { socket_path, svc })
    }

    pub async fn run(&self, cancel: CancellationToken) -> Empty {
        fs::create_dir_all(self.socket_path.parent().expect("could not find socket directory"))?;

        let listener = UnixListener::bind(self.socket_path.clone())?;
        let listener_stream = UnixListenerStream::new(listener);

        let srv = Server::builder()
            .add_service(GriftServiceServer::new(self.svc.clone()))
            .serve_with_incoming(listener_stream);

        tokio::select! {
            res = srv => res?,
            _ = cancel.cancelled() => (),
        }

        Ok(())
    }
}

impl<Svc: GriftService + Clone> Drop for GriftServer<Svc> {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.socket_path) {
            error!("unclean server shutdown, socket not removed: {e}")
        }
    }
}

fn grift_socket() -> eyre::Result<PathBuf> {
    Ok(XDG_DIRS.place_runtime_file("griftd.sock")?)
}
