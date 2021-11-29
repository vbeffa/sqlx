use sqlx_wasm_test::test_type;

test_type!(i8(
    Postgres,
    "0::\"char\"" == 0_i8,
    "120::\"char\"" == 120_i8,
));

test_type!(u32(Postgres, "325235::oid" == 325235_u32,));

test_type!(i16(
    Postgres,
    "-2144::smallint" == -2144_i16,
    "821::smallint" == 821_i16,
));

test_type!(i32(
    Postgres,
    "94101::int" == 94101_i32,
    "-5101::int" == -5101_i32
));

test_type!(i32_vec<Vec<i32>>(Postgres,
    "'{5,10,50,100}'::int[]" == vec![5_i32, 10, 50, 100],
    "'{1050}'::int[]" == vec![1050_i32],
    "'{}'::int[]" == Vec::<i32>::new(),
    "'{1,3,-5}'::int[]" == vec![1_i32, 3, -5]
));

test_type!(i64(Postgres, "9358295312::bigint" == 9358295312_i64));

test_type!(f32(Postgres, "9419.122::real" == 9419.122_f32));

test_type!(f64(
    Postgres,
    "939399419.1225182::double precision" == 939399419.1225182_f64
));

test_type!(f64_vec<Vec<f64>>(Postgres,
    "'{939399419.1225182,-12.0}'::float8[]" == vec![939399419.1225182_f64, -12.0]
));
