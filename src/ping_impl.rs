use std::net::IpAddr;
use std::time::Duration;

#[cfg(windows)]
mod windows_impl {
    use std::net::IpAddr;
    use std::time::Duration;
    use winping::{AsyncPinger, Buffer};

    pub struct PingClient {
        pinger: AsyncPinger,
    }

    pub fn create_client() -> Result<PingClient, String> {
        let mut pinger = AsyncPinger::new();
        pinger.set_timeout(2000); // 2秒
        Ok(PingClient { pinger })
    }

    pub async fn ping_once(client: &PingClient, ip: IpAddr, _seq: u16) -> Result<Duration, String> {
        let buffer = Buffer::new();
        let result = client.pinger.send(ip, buffer).await;
        match result.result {
            Ok(rtt_ms) => Ok(Duration::from_millis(rtt_ms as u64)),
            Err(e) => Err(format!("{e}")),
        }
    }
}

#[cfg(not(windows))]
mod unix_impl {
    use std::net::IpAddr;
    use std::time::Duration;
    use surge_ping::{Client, Config, PingIdentifier, PingSequence};

    pub struct PingClient {
        client: Client,
    }

    pub fn create_client() -> Result<PingClient, String> {
        let config = Config::default();
        Client::new(&config)
            .map(|client| PingClient { client })
            .map_err(|e| format!("socket error: {e} (may need root/sudo)"))
    }

    pub async fn ping_once(client: &PingClient, ip: IpAddr, seq: u16) -> Result<Duration, String> {
        let payload = [0u8; 56];
        let mut pinger = client
            .client
            .pinger(ip, PingIdentifier(rand::random()))
            .await;
        pinger.timeout(Duration::from_secs(2));
        match pinger.ping(PingSequence(seq), &payload).await {
            Ok((_packet, duration)) => Ok(duration),
            Err(e) => Err(format!("{e}")),
        }
    }
}

// ====================================================================
// 公開API（OS差を吸収）
// ====================================================================

#[cfg(windows)]
pub use windows_impl::PingClient;

#[cfg(not(windows))]
pub use unix_impl::PingClient;

pub fn create_client() -> Result<PingClient, String> {
    #[cfg(windows)]
    return windows_impl::create_client();

    #[cfg(not(windows))]
    return unix_impl::create_client();
}

pub async fn ping_once(client: &PingClient, ip: IpAddr, seq: u16) -> Result<Duration, String> {
    #[cfg(windows)]
    return windows_impl::ping_once(client, ip, seq).await;

    #[cfg(not(windows))]
    return unix_impl::ping_once(client, ip, seq).await;
}
