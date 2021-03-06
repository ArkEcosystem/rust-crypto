use arkecosystem_crypto::configuration::network;
use arkecosystem_crypto::enums::Network;
use arkecosystem_crypto::transactions::deserializer;
use *;

#[test]
#[ignore]
fn test_signed_with_a_passphrase() {
    let fixture = json_transaction("delegate_resignation", "passphrase");
    let serialized = &fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(serialized);

    network::set(Network::Devnet);

    assert_eq!(transaction.version, 1);
    assert_eq!(transaction.network, 30);
}
