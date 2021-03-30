// Module to hold custom code

// public structure used by callback functions
pub struct ClientInfo {
    pub client_fd   : i32,
    pub client_id   : u64,
}

// made up result enum for callbacks to return
pub enum CBResult {
    Keep,
    Disconnect,
}

/// Define default callbacks to use, when CallbackStruct is made using new
pub fn default_rx_cb(ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
    println!("default rx callback for client_id {}.", ci.client_id );
    Ok(CBResult::Keep)
}
pub fn default_tx_cb(ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
    println!("default tx callback for client_id {}.", ci.client_id );
    Ok(CBResult::Keep)
}
pub fn default_conn_cb(ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
    println!("default conn callback for client_id {}.", ci.client_id );
    Ok(CBResult::Keep)
}
pub fn default_dcon_cb(ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
    println!("default dconn callback for client_id {}.", ci.client_id );
    Ok(CBResult::Disconnect)
}

// structure of callback function pointers
pub struct CallbackStruct {
    pub rx_cb         : fn(ci : &ClientInfo) -> std::result::Result<CBResult, std::io::Error>,
    pub tx_cb         : fn(ci : &ClientInfo) -> std::result::Result<CBResult, std::io::Error>,
    pub conn_cb       : fn(ci : &ClientInfo) -> std::result::Result<CBResult, std::io::Error>,
    pub dcon_cb       : fn(ci : &ClientInfo) -> std::result::Result<CBResult, std::io::Error>,
}

// implementation of methods for Callback Structure
impl CallbackStruct {
    // create a new default CallbackStruct instance.
    pub fn new() -> CallbackStruct {
        CallbackStruct {
            rx_cb : (default_rx_cb),
            tx_cb : (default_tx_cb),
            conn_cb : (default_conn_cb),
            dcon_cb : (default_dcon_cb),
        }
    }
    // interface to call the rx_cb callback for this structure
    pub fn on_rx(&self, ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
        (self.rx_cb)(&ci)
    }
    // interface to call the tx_cb callback for this structure
    pub fn on_tx(&self, ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
        (self.tx_cb)(&ci)
    }
    // interface to call the conn_cb callback for this structure
    pub fn on_conn(&self, ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
        (self.conn_cb)(&ci)
    }
    // interface to call the dcon_cb callback for this structure.
    pub fn on_dcon(&self, ci : &ClientInfo ) -> std::result::Result<CBResult, std::io::Error> {
        (self.dcon_cb)(&ci)
    }
}

