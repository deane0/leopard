use async_trait::async_trait;
use http::HeaderName;
use pingora::prelude::*;

use crate::service::HostConfig;

pub struct ProxyApp {
    host_configs: Vec<HostConfig>,
}

impl ProxyApp {
    pub fn new(host_configs: Vec<HostConfig>) -> Self {
        Self { host_configs }
    }
}

#[async_trait]
impl ProxyHttp for ProxyApp {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let host_header = session
            .get_header(HeaderName::from_static("host"))
            .unwrap()
            .to_str()
            .unwrap();
        log::debug!("host header: {host_header}");

        let host_config = self
            .host_configs
            .iter()
            .find(|host_config| host_config.proxy_hostname == host_header)
            .unwrap();

        let proxy_to = HttpPeer::new(
            host_config.proxy_addr.as_str(),
            host_config.proxy_tls,
            host_config.proxy_hostname.clone(),
        );

        let peer = Box::new(proxy_to);

        Ok(peer)
    }
}
