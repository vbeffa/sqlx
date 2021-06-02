use sqlx_wasm_test::test_type;

test_type!(null<Option<i16>>(Postgres,
    "NULL::int2" == None::<i16>
));

test_type!(null_vec<Vec<Option<i16>>>(Postgres,
    "array[10,NULL,50]::int2[]" == vec![Some(10_i16), None, Some(50)],
));

test_type!(bool<bool>(Postgres,
    "false::boolean" == false,
    "true::boolean" == true
));

test_type!(bool_vec<Vec<bool>>(Postgres,
    "array[true,false,true]::bool[]" == vec![true, false, true],
));

test_type!(byte_vec<Vec<u8>>(Postgres,
    "E'\\\\xDEADBEEF'::bytea"
        == vec![0xDE_u8, 0xAD, 0xBE, 0xEF],
    "E'\\\\x'::bytea"
        == Vec::<u8>::new(),
    "E'\\\\x0000000052'::bytea"
        == vec![0_u8, 0, 0, 0, 0x52]
));
