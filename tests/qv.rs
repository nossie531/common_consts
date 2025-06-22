use common_consts::prelude::*;

#[test]
fn test() {
    let values = vec![
        qv::ZERO,
        qv::FEW,
        qv::SOME,
        qv::SEVERAL,
        qv::HANDFUL,
        qv::MANY,
    ];

    assert!(values.windows(2).all(|w| w[0] < w[1]));
}
