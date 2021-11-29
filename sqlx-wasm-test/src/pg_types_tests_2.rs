use sqlx_wasm_test::{test_prepared_type, test_type};

// BYTEA cannot be decoded by-reference from a simple query as postgres sends it as hex
test_prepared_type!(byte_slice<&[u8]>(Postgres,
    "E'\\\\xDEADBEEF'::bytea"
        == &[0xDE_u8, 0xAD, 0xBE, 0xEF][..],
    "E'\\\\x0000000052'::bytea"
        == &[0_u8, 0, 0, 0, 0x52][..]
));

test_type!(str<&str>(Postgres,
    "'this is foo'" == "this is foo",
    "''" == "",
    "'identifier'::name" == "identifier",
    "'five'::char(4)" == "five",
    "'more text'::varchar" == "more text",
));

test_type!(string<String>(Postgres,
    "'this is foo'" == format!("this is foo"),
));

test_type!(string_vec<Vec<String>>(Postgres,
    "array['one','two','three']::text[]"
        == vec!["one","two","three"],

    "array['', '\"']::text[]"
        == vec!["", "\""],

    "array['Hello, World', '', 'Goodbye']::text[]"
        == vec!["Hello, World", "", "Goodbye"]
));
