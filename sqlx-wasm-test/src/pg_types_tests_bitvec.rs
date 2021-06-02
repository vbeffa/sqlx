use sqlx_wasm_test::test_type;

test_type!(bitvec<sqlx::types::BitVec>(
    Postgres,
    // A full byte VARBIT
    "B'01101001'" == sqlx::types::BitVec::from_bytes(&[0b0110_1001]),
    // A VARBIT value missing five bits from a byte
    "B'110'" == {
        let mut bit_vec = sqlx::types::BitVec::with_capacity(4);
        bit_vec.push(true);
        bit_vec.push(true);
        bit_vec.push(false);
        bit_vec
    },
    // A BIT value
    "B'01101'::bit(5)" == {
        let mut bit_vec = sqlx::types::BitVec::with_capacity(5);
        bit_vec.push(false);
        bit_vec.push(true);
        bit_vec.push(true);
        bit_vec.push(false);
        bit_vec.push(true);
        bit_vec
    },
));
