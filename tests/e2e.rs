use dotenv::dotenv;
use std::env;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

use rustix3::models::TgId;
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
    let base = env::var("PANEL_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:2053/".into());
    let user = env::var("PANEL_USERNAME").unwrap_or_else(|_| "admin".into());
    let pass = env::var("PANEL_PASSWORD").unwrap_or_else(|_| "admin".into());

    let client = Client::new(user, pass, base).await.expect("login");
    log::info!("connected");

    let list_before = client.get_inbounds_list().await.expect("list");
    log::info!("list_before = {:#?}", list_before);
    assert!(list_before.is_ok());

    let remark = format!("e2e-{}", Uuid::new_v4());
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
        tg_id: TgId::Int(0),
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
        tg_id: TgId::Int(0),
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
    log::info!("list_after = {:#?}", list_after);

    let last_online = client.get_last_online().await.expect("last_online");
    assert!(last_online.is_ok());

    let cuuid = Uuid::new_v4().to_string();
    let email = "testclient".to_string();
    let user_obj = User {
        id: cuuid.clone(),
        flow: String::new(),
        email: email.clone(),
        limit_ip: 0,
        total_gb: 0,
        expiry_time: 0,
        enable: true,
        tg_id: TgId::Int(0),
        sub_id: String::new(),
        reset: 0,
    };

    let remark2 = format!("e2e-del-by-email-{}", Uuid::new_v4());
    let tmp_inb_req = CreateInboundRequest {
        up: 0,
        down: 0,
        total: 0,
        remark: remark2.clone(),
        enable: true,
        expiry_time: 0,
        listen: String::new(),
        port: 31002,
        protocol: InboundProtocols::Vless,
        settings: Settings {
            clients: vec![user_obj.clone()],
            decryption: "none".into(),
            fallbacks: Vec::<Fallback>::new(),
        },
        stream_settings: "{}".into(),
        sniffing: "{}".into(),
        allocate: "{}".into(),
    };
    let tmp_created = client
        .add_inbound(&tmp_inb_req)
        .await
        .expect("add_inbound_tmp");
    assert!(tmp_created.is_ok());
    let tmp_inbound_id = tmp_created.object.id;

    let tmp = client.get_inbounds_list().await.expect("tmp inbound");
    assert!(tmp.is_ok());
    log::info!("tmp inbound = {:#?}", tmp);

    // let del_by_email = client
    //     .del_client_by_email(tmp_inbound_id, &email)
    //     .await
    //     .expect("del_client_by_email");
    // assert!(del_by_email.is_ok()); // todo

    let res = client
        .delete_inbound(tmp_inbound_id)
        .await
        .expect("del_tmp_inbound");

    assert!(res.is_ok());

    let srv_status = client.server_status().await.expect("server_status");
    assert!(srv_status.is_ok());

    let db_bytes = client.server_get_db().await.expect("server_get_db");
    assert!(!db_bytes.is_empty(), "db should not be empty");


    let imported_db = client
        .import_db_upload("file", db_bytes.clone())
        .await
        .expect("import_db_upload");
    assert!(imported_db.is_ok());

    let xver = client.get_xray_version().await.expect("xray_version");
    assert!(xver.is_ok());
    let current_version = xver.object.clone();

    let cfg = client.get_config_json().await.expect("get_config_json");
    assert!(cfg.is_ok());

    let cpu_hist = client.cpu_history(2).await.expect("cpu_history_1min"); // todo bucket
    assert!(cpu_hist.is_ok());


    if let Some(first) = cpu_hist.object.first() {
        assert!(first.t > 0, "cpu history timestamp should be > 0");
    }

    let new_uuid = client.get_new_uuid().await.expect("get_new_uuid");
    assert!(new_uuid.is_ok());

    let parsed = Uuid::parse_str(&new_uuid.object.uuid);
    assert!(parsed.is_ok(), "server UUID should be valid");

    let x25519 = client.get_new_x25519_cert().await.expect("get_new_x25519");
    assert!(x25519.is_ok());

    let mldsa = client.get_new_mldsa65().await.expect("get_new_mldsa65");
    assert!(mldsa.is_ok());

    let mlkem = client.get_new_mlkem768().await.expect("get_new_mlkem768");
    assert!(mlkem.is_ok());

    let venc = client.get_new_vless_enc().await.expect("get_new_vless_enc");
    assert!(venc.is_ok());

    // let ech = client.get_new_ech_cert().await.expect("get_new_ech_cert");
    // assert!(ech.is_ok()); //  todo

    let stopped = client.stop_xray_service().await.expect("stop_xray_service");
    assert!(stopped.is_ok());

    sleep(Duration::from_secs(1)).await;

    let restarted = client
        .restart_xray_service()
        .await
        .expect("restart_xray_service");
    assert!(restarted.is_ok());

    sleep(Duration::from_secs(2)).await;

    log::info!("ver: {:#?}", current_version.get(0).expect("version"));

    let reinstall = client
        .install_xray_version(current_version.get(0).expect("version"))
        .await
        .expect("install_xray_version");
    assert!(reinstall.is_ok());

    let geo_all = client.update_geofile().await.expect("update_geofile");
    assert!(geo_all.is_ok());

    // let geo_one = client
    //     .update_geofile_by_name("geoip")
    //     .await
    //     .expect("update_geofile_by_name");
    // assert!(geo_one.is_ok()); // todo

    // let logs = client.logs(50).await.expect("logs_count");
    // assert!(logs.is_ok()); // todo

    // let xlogs = client.xray_logs(50).await.expect("xray_logs_count");
    // assert!(xlogs.is_ok()); // todo


    let remark = format!("e2e-{}", Uuid::new_v4());
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


    let inbds = client.get_inbounds_list().await.expect("list_for_import");
    assert!(inbds.is_ok());

    log::info!("{:#?}", inbds);

    // let import_inb = client // todo fix cannot unmarshal object into Go struct field Inbound.settings of type string
    //     .import_inbound(&inbds.object.get(0).expect("object"))
    //     .await
    //     .expect("import_inbounds");
    // assert!(import_inb.is_ok());
}
