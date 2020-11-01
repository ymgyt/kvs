use clap::{App, Arg, ArgMatches, SubCommand};

use crate::cli::{server_addr, SERVER};
use crate::common::info;
use crate::server::tcp::{Config, Server};
use crate::{KvsError, Result};

const ARG_MAX_CONN: &str = "max_connection";

pub(super) fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name(SERVER)
        .about("Running kvs server")
        .arg(
            Arg::with_name(ARG_MAX_CONN)
                .long("max-connections")
                .takes_value(true)
                .help("Max tcp connections"),
        )
}

pub async fn run(m: &ArgMatches<'_>) -> Result<()> {
    let addr = server_addr(m);
    let config = Config::default()
        .set_max_tcp_connections(m.value_of(ARG_MAX_CONN).and_then(|s| s.parse().ok()));
    let server = Server::new(config);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Listening {}", addr);
    server.run(listener).await.map_err(KvsError::from)
}
