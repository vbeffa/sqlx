use sqlx::postgres::types::PgRange;
use sqlx_wasm_test::test_type;
use std::ops::Bound;

const EXC2: Bound<i32> = Bound::Excluded(2);
const EXC3: Bound<i32> = Bound::Excluded(3);
const INC1: Bound<i32> = Bound::Included(1);
const INC2: Bound<i32> = Bound::Included(2);
const UNB: Bound<i32> = Bound::Unbounded;

test_type!(int4range<PgRange<i32>>(Postgres,
    "'(,)'::int4range" == PgRange::from((UNB, UNB)),
    "'(,]'::int4range" == PgRange::from((UNB, UNB)),
    "'(,2)'::int4range" == PgRange::from((UNB, EXC2)),
    "'(,2]'::int4range" == PgRange::from((UNB, EXC3)),
    "'(1,)'::int4range" == PgRange::from((INC2, UNB)),
    "'(1,]'::int4range" == PgRange::from((INC2, UNB)),
    "'(1,2]'::int4range" == PgRange::from((INC2, EXC3)),
    "'[,)'::int4range" == PgRange::from((UNB, UNB)),
    "'[,]'::int4range" == PgRange::from((UNB, UNB)),
    "'[,2)'::int4range" == PgRange::from((UNB, EXC2)),
    "'[,2]'::int4range" == PgRange::from((UNB, EXC3)),
    "'[1,)'::int4range" == PgRange::from((INC1, UNB)),
    "'[1,]'::int4range" == PgRange::from((INC1, UNB)),
    "'[1,2)'::int4range" == PgRange::from((INC1, EXC2)),
    "'[1,2]'::int4range" == PgRange::from((INC1, EXC3)),
));
