// Database
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::common::db;

// POST /log/exec
pub async fn log_exec(exec: db::LogExecObject, addr: Option<SocketAddr>) -> Result<impl warp::Reply, warp::Rejection> {
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let remote_addr = match addr {
        Some(inetaddr)  => inetaddr.ip(),
        None => return Err(warp::reject::not_found())
    };
    if remote_addr != localhost {
        return Err(warp::reject::not_found());
    }
    // Input to this function is untrusted
    let conn: rusqlite::Connection = db::db_open();
    db::insert_exec(&conn, exec);
    return Ok(warp::reply());
}
