
pub struct ClientInfo {
    pub client_fd   : i32,
    pub client_id   : u64,
}

pub enum CBResult {
    Keep,
    Disconnect,
}
///////////////// default callbacks ////////////////
fn default_rx_cb(ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
    println!("rx callback for client_id {}.", ci.client_id );
    Ok(CBResult::Keep)
}
fn default_tx_cb(ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
    println!("tx callback for client_id {}.", ci.client_id );
    Ok(CBResult::Keep)
}
fn default_conn_cb(ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
    println!("conn callback for client_id {}.", ci.client_id );
    Ok(CBResult::Keep)
}
fn default_dcon_cb(ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
    println!("dconn callback for client_id {}.", ci.client_id );
    Ok(CBResult::Disconnect)
}
pub struct CallbackStruct {
    pub rx_cb         : fn(ci : &ClientInfo) -> std::result::Result<CBResult, std::io::Error>,
    pub tx_cb         : fn(ci : &ClientInfo) -> std::result::Result<CBResult, std::io::Error>,
    pub conn_cb       : fn(ci : &ClientInfo) -> std::result::Result<CBResult, std::io::Error>,
    pub dcon_cb       : fn(ci : &ClientInfo) -> std::result::Result<CBResult, std::io::Error>,
}
impl CallbackStruct {
    pub fn new() -> CallbackStruct {
        CallbackStruct {
            rx_cb : (default_rx_cb),
            tx_cb : (default_tx_cb),
            conn_cb : (default_conn_cb),
            dcon_cb : (default_dcon_cb),
        }
    }
    pub fn on_rx(&self, ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
        (self.rx_cb)(&ci)
    }
    pub fn on_tx(&self, ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
        (self.tx_cb)(&ci)
    }
    pub fn on_conn(&self, ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
        (self.conn_cb)(&ci)
    }
    pub fn on_dcon(&self, ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
        (self.dcon_cb)(&ci)
    }
}
