use std::future::Future;
use std::path::{Path, PathBuf};

use tokio::fs;
use tokio::net::TcpListener;

use crate::common::{info, Result};
use crate::config::{filepath, Config};
use crate::core;
use crate::server::tcp::Server;
use crate::KvsdError;

/// Initializer initialize kvsd.
/// mainly to do the following.
///   * create directory structure
///   * build core kvsd from config
///   * listen tcp if needed, then run tcp server
#[derive(Debug)]
pub struct Initializer {
    pub(crate) config: Config,
    listener: Option<TcpListener>,
}

impl Initializer {
    /// Construct Initializer from config.
    pub fn from_config(config: Config) -> Self {
        Self {
            config,
            listener: None,
        }
    }

    /// Set kvsd root directory.
    pub fn set_root_dir(&mut self, root_dir: impl Into<PathBuf>) {
        self.config.kvsd.root_dir = Some(root_dir.into());
    }

    /// Set tcp listener.
    /// the initializer call the bind system call if the tcp listener is not set at startup.
    pub fn set_listener(&mut self, listener: TcpListener) {
        self.listener = Some(listener);
    }

    pub(crate) async fn load_config_file(path: impl AsRef<Path>) -> Result<Self> {
        let f = fs::File::open(path).await?;
        let config = serde_yaml::from_reader::<_, Config>(f.into_std().await)?;

        Ok(Initializer::from_config(config))
    }

    /// Running initialize process.
    /// start the graceful shutdown process when shutdown future returns Poll::Ready.
    pub async fn run_kvsd(self, shutdown: impl Future) -> Result<(), KvsdError> {
        let builder = core::Builder::from_config(self.config.kvsd);
        let kvsd = builder.build().await?;
        let request_sender = kvsd.request_channel();

        tokio::spawn(kvsd.run());

        let listener = match self.listener {
            Some(listener) => listener,
            None => {
                let addr = self.config.server.listen_addr();
                info!(%addr, "Listening");
                TcpListener::bind(addr).await?
            }
        };

        let server = Server::new(self.config.server);

        server.run(request_sender, listener, shutdown).await?;

        Ok(())
    }

    /// Initialize kvsd directory structure.
    pub async fn init_dir(&mut self) -> Result<(), KvsdError> {
        let root_dir = self.config.kvsd.root_dir.clone().unwrap();

        info!(path=%root_dir.display(), "Initialize kvsd root directory");

        // Create root kvsd directory.
        tokio::fs::create_dir_all(root_dir.as_path()).await?;

        // Namespaces.
        let namespaces = root_dir.join(filepath::NAMESPACES);
        tokio::fs::create_dir_all(namespaces.as_path()).await?;

        let initial_namespaces = vec![
            namespaces.join(filepath::NS_SYSTEM),
            namespaces.join("default/default"),
        ];

        for ns in &initial_namespaces {
            tokio::fs::create_dir_all(ns).await?;
        }

        Ok(())
    }
}
