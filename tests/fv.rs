use common_consts::prelude::*;

#[test]
fn test() {
    let values = vec![
        fv::NEVER,
        fv::ALMOST_NEVER,
        fv::RARELY,
        fv::OCCASIONALLY,
        fv::SOMETIMES,
        fv::OFTEN,
        fv::ALMOST,
        fv::ALWAYS,
    ];

    assert!(values.is_sorted());
}
