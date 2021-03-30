pub mod my_mod;

fn main() {
    println!("callback test.");
    let ci = my_mod::ClientInfo {
        client_fd : 5,
        client_id : 1,
    };

    let cbs = my_mod::CallbackStruct::new();

    cbs.on_conn(&ci).unwrap();
    cbs.on_rx(&ci).unwrap();
    cbs.on_tx(&ci).unwrap();
    cbs.on_dcon(&ci).unwrap();

}
