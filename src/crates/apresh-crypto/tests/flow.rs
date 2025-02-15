#[test]
fn test_flow() {
    let (host_public_key, host_secret_key) = apresh_crypto::generate([42; 32]);
    let content = b"Hello, Apresh!";

    let (message, _secret_key) =
        apresh_crypto::encrypt_for(&host_public_key.as_bytes()[..], content, [0; 12], [43; 32])
            .unwrap();

    let decrypted = apresh_crypto::extract(&host_secret_key.to_bytes(), &message);

    assert_eq!(content.to_vec(), decrypted);
}
