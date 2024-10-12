use std::net::ToSocketAddrs;

use error_stack::ResultExt;

use crate::error::cli::config::cluster::ValidateServersError;



pub(super) fn validate_servers(servers: &[String]) -> error_stack::Result<(), ValidateServersError> {
    let unvalidated_bootstrap_servers: Result<Vec<_>, std::io::Error> = servers
    .iter()
    .map(|address|address.to_socket_addrs())
    .collect();

    unvalidated_bootstrap_servers
        .change_context(ValidateServersError::ValidationFailed)
        .attach_printable_lazy(|| {
            format!("Failed to parse socket addresses for bootstrap_servers: {:?}", servers)
        }).map(|_| ())
}