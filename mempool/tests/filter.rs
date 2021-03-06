use nimiq_hash::{Hash, Blake2bHash};
use nimiq_keys::Address;
use nimiq_mempool::filter::{MempoolFilter, Rules};
use nimiq_primitives::coin::Coin;
use nimiq_primitives::networks::NetworkId;
use nimiq_transaction::Transaction;

#[test]
fn it_can_blacklist_transactions() {
    let mut f: MempoolFilter = Default::default();

    let tx = Transaction::new_basic(
        Address::from([32u8; Address::SIZE]),
        Address::from([213u8; Address::SIZE]),
        Coin::from_u64(100).unwrap(),
        Coin::from_u64(1).unwrap(),
        123,
        NetworkId::Main,
    );

    let hash: Blake2bHash = tx.hash();
    f.blacklist(hash.clone());
    assert!(f.blacklisted(&hash));
    f.remove(&hash);
    assert!(!f.blacklisted(&hash));
}

#[test]
fn it_accepts_and_rejects_transactions() {
    let mut s: Rules = Rules::default();
    s.tx_fee = Coin::from_u64(1).unwrap();

    let f = MempoolFilter::new(s, MempoolFilter::DEFAULT_BLACKLIST_SIZE);

    let mut tx = Transaction::new_basic(
        Address::from([32u8; Address::SIZE]),
        Address::from([213u8; Address::SIZE]),
        Coin::from_u64(0).unwrap(),
        Coin::from_u64(0).unwrap(),
        0,
        NetworkId::Main,
    );

    assert!(!f.accepts_transaction(&tx));
    tx.fee = Coin::from_u64(1).unwrap();
    assert!(f.accepts_transaction(&tx));
}
