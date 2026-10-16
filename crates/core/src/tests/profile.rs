use super::*;

#[test]
fn parses_only_default_and_rejects_ambiguous_or_partial_files() {
    let credentials = parse("# comment\n[other]\nsecret_id=ignored\n[default]\nsecret_id=test-id\nsecret_key=test=key\ntoken=test-token\n").unwrap();
    assert_eq!(credentials.secret_id(), "test-id");
    assert_eq!(credentials.secret_key(), "test=key");
    assert_eq!(credentials.token().unwrap(), "test-token");
    for text in [
        "",
        "[default]\nsecret_id=id",
        "[default]\nsecret_id=id\nsecret_id=other\nsecret_key=key",
        "[default]\ninvalid",
        "[default",
    ] {
        assert!(parse(text).is_err());
    }
}
