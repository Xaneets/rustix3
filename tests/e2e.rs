use dotenv::dotenv;
use env_logger;
use std::env;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

use rustix3::{
    client::Client,
    inbounds::InboundProtocols,
    models::{ClientRequest, ClientSettings, CreateInboundRequest, Fallback, Settings, User},
};

#[tokio::test]
async fn e2e_full_flow() {
    dotenv().ok();
    env_logger::init();

    log::info!("Starting full flow");
    log::trace!("Starting full flow2");
    let base = env::var("PANEL_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:2053/".into());
    let user = env::var("PANEL_USERNAME").unwrap_or_else(|_| "admin".into());
    let pass = env::var("PANEL_PASSWORD").unwrap_or_else(|_| "admin".into());

    let client = Client::new(user, pass, base).await.expect("login");
    log::info!("connected");

    let list_before = client.get_inbounds_list().await.expect("list");
    log::info!("list_before = {:#?}", list_before);
    assert!(list_before.is_ok());

    let remark = format!("e2e-{}", Uuid::new_v4().to_string());
    let req = CreateInboundRequest {
        up: 0,
        down: 0,
        total: 0,
        remark: remark.clone(),
        enable: true,
        expiry_time: 0,
        listen: String::new(),
        port: 31001,
        protocol: InboundProtocols::Vless,
        settings: Settings {
            clients: vec![],
            decryption: "none".into(),
            fallbacks: Vec::<Fallback>::new(),
        },
        stream_settings: "{}".into(),
        sniffing: "{}".into(),
        allocate: "{}".into(),
    };

    let created = client.add_inbound(&req).await.expect("add_inbound");

    assert!(created.is_ok());

    let inbounds = client.get_inbounds_list().await.expect("list");
    log::info!("inbounds = {:#?}", inbounds);

    let inbound_id = created.object.id;

    let by_id = client
        .get_inbound_by_id(inbound_id)
        .await
        .expect("get_by_id");
    assert!(by_id.is_ok());
    assert_eq!(by_id.object.remark, remark);

    let mut updated_req = req;
    updated_req.remark = format!("{}-upd", remark);
    let updated = client
        .update_inbound(inbound_id, &updated_req)
        .await
        .expect("update_inbound");
    assert!(updated.is_ok());
    assert_eq!(updated.object.remark, updated_req.remark);

    let cuuid = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", cuuid);
    let user_obj = User {
        id: cuuid.clone(),
        flow: String::new(),
        email: email.clone(),
        limit_ip: 0,
        total_gb: 0,
        expiry_time: 0,
        enable: true,
        tg_id: String::new(),
        sub_id: String::new(),
        reset: 0,
    };
    let add_client_req = ClientRequest {
        id: inbound_id,
        settings: ClientSettings {
            clients: vec![user_obj.clone()],
        },
    };
    let add_client = client
        .add_client_to_inbound(&add_client_req)
        .await
        .expect("add_client");
    assert!(add_client.is_ok());

    let inbounds = client.get_inbounds_list().await.expect("list");
    log::info!("inbounds = {:#?}", inbounds);

    sleep(Duration::from_millis(200)).await;

    let traffic_by_email = client
        .get_client_traffic_by_email(email.clone())
        .await
        .expect("traffic_by_email");
    assert!(traffic_by_email.is_ok());
    assert_eq!(traffic_by_email.object.email, email);

    let traffic_by_id = client
        .get_client_traffic_by_id(cuuid.clone())
        .await
        .expect("traffic_by_id");
    assert!(traffic_by_id.is_ok());

    let mut updated_user = user_obj;
    updated_user.limit_ip = 1;
    let upd_client_req = ClientRequest {
        id: inbound_id,
        settings: ClientSettings {
            clients: vec![updated_user],
        },
    };
    let upd_client = client
        .update_client(&cuuid, &upd_client_req)
        .await
        .expect("update_client");
    assert!(upd_client.is_ok());

    let clear_ips = client.clear_client_ips(&email).await.expect("clear_ips");
    assert!(clear_ips.is_ok());

    let reset_client = client
        .reset_client_traffic(inbound_id, &email)
        .await
        .expect("reset_client");
    assert!(reset_client.is_ok());

    let reset_all_clients = client
        .reset_all_client_traffics(inbound_id)
        .await
        .expect("reset_all_clients");
    assert!(reset_all_clients.is_ok());

    let reset_all_inbounds = client
        .reset_all_inbound_traffics()
        .await
        .expect("reset_all_inbounds");
    assert!(reset_all_inbounds.is_ok());

    let onlines = client.online_clients().await.expect("online_clients");
    assert!(onlines.is_ok());

    let cuuid = Uuid::new_v4().to_string();
    let email = format!("{}@example.com", cuuid);
    let user_obj = User {
        id: cuuid.clone(),
        flow: String::new(),
        email: email.clone(),
        limit_ip: 0,
        total_gb: 0,
        expiry_time: 0,
        enable: true,
        tg_id: String::new(),
        sub_id: String::new(),
        reset: 0,
    };
    let add_client_req = ClientRequest {
        id: inbound_id,
        settings: ClientSettings {
            clients: vec![user_obj.clone()],
        },
    };
    let add_client = client
        .add_client_to_inbound(&add_client_req)
        .await
        .expect("add_client");
    assert!(add_client.is_ok());

    let inbounds = client.get_inbounds_list().await.expect("list");
    log::info!("inbounds = {:#?}", inbounds);

    let del_client = client
        .delete_client(inbound_id, &cuuid)
        .await
        .expect("delete_client");
    assert!(del_client.is_ok());

    let inbounds = client.get_inbounds_list().await.expect("list");
    log::info!("inbounds = {:#?}", inbounds);

    let del_depleted = client
        .delete_depleted_clients(inbound_id)
        .await
        .expect("delete_depleted");
    assert!(del_depleted.is_ok());

    let del_inbound = client
        .delete_inbound(inbound_id)
        .await
        .expect("delete_inbound");
    assert!(del_inbound.is_ok());

    let list_after = client.get_inbounds_list().await.expect("list_after");
    assert!(list_after.is_ok());
}
