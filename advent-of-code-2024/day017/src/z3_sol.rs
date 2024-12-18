use z3::ast::{Ast, BV};

pub fn solve_by_z3(ints: &[i64]) {
    let ctx = z3::Context::new(&z3::Config::new());
    let opt = z3::Optimize::new(&ctx);

    // set the constant to find
    let s = BV::new_const(&ctx, "s", 64);

    // represent a, b, c as BitVec
    let (mut a, mut b, mut c) = (
        s.clone(),
        BV::from_i64(&ctx, 0, 64),
        BV::from_i64(&ctx, 0, 64),
    );

    // init some constant
    let (bv_1, bv_5, bv_8) = (
        BV::from_i64(&ctx, 1, 64),
        BV::from_i64(&ctx, 5, 64),
        BV::from_i64(&ctx, 8, 64),
    );

    // set interval and expect output
    for x in ints {
        // do calculation
        b = a.bvsmod(&bv_8) ^ &bv_1;
        c = a.bvsdiv(&(&bv_1 << b.clone()));
        b = (b.clone() ^ &bv_5) ^ c;
        a = a.bvsdiv(&bv_8);

        // check output with x
        opt.assert(&(b.bvsmod(&bv_8))._eq(&BV::from_i64(&ctx, *x, 64)));
    }
    opt.assert(&a._eq(&BV::from_i64(&ctx, 0, 64)));

    // select min s
    opt.minimize(&s);

    // check the value
    assert_eq!(opt.check(&[]), z3::SatResult::Sat);

    // start solving
    println!(
        "solve2: {}",
        opt.get_model()
            .unwrap()
            .eval(&s, true)
            .unwrap()
            .as_i64()
            .unwrap()
    )
}
