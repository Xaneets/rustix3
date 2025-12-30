use rustix3::inbounds::InboundProtocols;

#[test]
fn inbound_protocol_shadowsocks_serializes() {
    let json = serde_json::to_string(&InboundProtocols::ShadowsSocks).unwrap();
    assert_eq!(json, "\"shadowsocks\"");
}

#[test]
fn inbound_protocol_unknown_deserializes() {
    let val: InboundProtocols = serde_json::from_str("\"new-proto\"").unwrap();
    assert!(matches!(val, InboundProtocols::Unknown));
}
