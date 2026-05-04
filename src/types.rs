use std::net::IpAddr;

/// 1回分のping結果
#[derive(Clone, PartialEq)]
pub struct PingResult {
    pub seq: u32,
    pub host: String,
    pub success: bool,
    pub time_ms: Option<f64>,
    pub error: Option<String>,
}

/// ホスト名/IP文字列をIpAddrに解決する
pub fn resolve_host(host: &str) -> Result<IpAddr, String> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(ip);
    }
    use std::net::ToSocketAddrs;
    let addr = format!("{host}:0");
    addr.to_socket_addrs()
        .map_err(|e| format!("dns error: {e}"))?
        .next()
        .map(|sa| sa.ip())
        .ok_or_else(|| "no address resolved".to_string())
}
