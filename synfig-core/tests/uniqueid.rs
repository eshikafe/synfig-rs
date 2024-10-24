use synfig_core::uniqueid::*;

#[test]
fn test_id() {
    let a1 = UniqueID::new(Some(1000));
    let a2 = UniqueID::new(Some(1000));
    let a3 = UniqueID::new(Some(999));
    let _b = UniqueID::new(None);

    assert_eq!(a1, a2);
    assert_ne!(a3, a1);

    assert_eq!(a1.get_uid(), 1000);

    let a4 = a2.nil();
    assert_eq!(a4.get_uid(), 0);

}