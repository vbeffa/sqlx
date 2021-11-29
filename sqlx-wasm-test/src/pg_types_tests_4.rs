use sqlx_wasm_test::test_decode_type;

test_decode_type!(bool_tuple<(bool,)>(Postgres, "row(true)" == (true,)));

test_decode_type!(num_tuple<(i32, i64, f64,)>(Postgres, "row(10,515::int8,3.124::float8)" == (10,515,3.124)));

test_decode_type!(empty_tuple<()>(Postgres, "row()" == ()));

test_decode_type!(string_tuple<(String, String, String)>(Postgres,
    "row('one','two','three')"
        == ("one".to_string(), "two".to_string(), "three".to_string()),

    "row('', '\"', '\"\"\"\"\"\"')"
        == ("".to_string(), "\"".to_string(), "\"\"\"\"\"\"".to_string()),

    "row('Hello, World', '', 'Goodbye')"
        == ("Hello, World".to_string(), "".to_string(), "Goodbye".to_string())
));
