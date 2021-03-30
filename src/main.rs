pub mod my_mod;

fn custom_rx_cb(ci : &my_mod::ClientInfo ) -> std::result::Result<my_mod::CBResult, std::io::Error> {
    println!("custom rx callback for client_id {}.", ci.client_id );
    Ok(my_mod::CBResult::Keep)
}

fn custom_rx2_cb(ci : &my_mod::ClientInfo ) -> std::result::Result<my_mod::CBResult, std::io::Error> {
    println!("custom rx2 callback for client_id {}.", ci.client_id );
    Ok(my_mod::CBResult::Keep)
}


fn  custom_tx_cb(ci : &my_mod::ClientInfo ) -> std::result::Result<my_mod::CBResult, std::io::Error> {
    println!("custom tx callback for client_id {}.", ci.client_id );
    Ok(my_mod::CBResult::Keep)
}

fn main() {
    println!("callback test.");

    // create an instance of the client info struct to use
    let ci = my_mod::ClientInfo {
        client_fd : 5,
        client_id : 1,
    };

    // create a instance of the callback structure initialized with 
    // default callbacks
    println!("Default callbacks............");
    let default_cbs = my_mod::CallbackStruct::new();
    default_cbs.on_conn(&ci).unwrap();
    default_cbs.on_rx(&ci).unwrap();
    default_cbs.on_tx(&ci).unwrap();
    default_cbs.on_dcon(&ci).unwrap();

    println!("Custom callbacks..............");
    // create an instance with custom callbacks for tx and rx
    let mut custom_cbs = my_mod::CallbackStruct {
        rx_cb : (custom_rx_cb),
        tx_cb : (custom_tx_cb),
        conn_cb : (my_mod::default_conn_cb),
        dcon_cb : (my_mod::default_dcon_cb),
    };

    custom_cbs.on_conn(&ci).unwrap();
    custom_cbs.on_rx(&ci).unwrap();
    custom_cbs.on_tx(&ci).unwrap();
    custom_cbs.on_dcon(&ci).unwrap();

    // change mapping on the fly.
    println!("update rx_cb to rx2 callback......");
    custom_cbs.rx_cb = custom_rx2_cb;
    custom_cbs.on_rx(&ci).unwrap();

    println!("done..");


}
