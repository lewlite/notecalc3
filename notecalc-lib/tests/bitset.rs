use notecalc_lib::helper::{content_y, BitFlag256};

#[test]
fn test_single_row() {
    let b = BitFlag256::single_row(0);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001
    );
    assert_eq!(
        b.bitset[1],
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    );
    assert_eq!(true, b.need(content_y(0)));
    assert_eq!(false, b.need(content_y(1)));
}

#[test]
fn test_single_row2() {
    let b = BitFlag256::single_row(32);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    );
    assert_eq!(false, b.need(content_y(0)));
    assert_eq!(false, b.need(content_y(1)));
    assert_eq!(true, b.need(content_y(32)));
}

#[test]
fn test_single_row3() {
    let b = BitFlag256::single_row(128);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001
    );
    assert_eq!(false, b.need(content_y(0)));
    assert_eq!(false, b.need(content_y(1)));
    assert_eq!(true, b.need(content_y(128)));
}

#[test]
fn test_single_row4() {
    let b = BitFlag256::single_row(128 + 32);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000
    );
    assert_eq!(false, b.need(content_y(0)));
    assert_eq!(false, b.need(content_y(1)));
    assert_eq!(false, b.need(content_y(128)));
    assert_eq!(false, b.need(content_y(32)));
    assert_eq!(true, b.need(content_y(128 + 32)));
}

#[test]
fn test_single_row5() {
    let b = BitFlag256::single_row(255);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b10000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000
    );
    assert_eq!(false, b.need(content_y(0)));
    assert_eq!(false, b.need(content_y(1)));
    assert_eq!(false, b.need(content_y(128)));
    assert_eq!(false, b.need(content_y(127)));
    assert_eq!(true, b.need(content_y(255)));
}

#[test]
fn test_all_rows_starting_at() {
    let b = BitFlag256::all_rows_starting_at(192);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000
    );
    for i in 0..256 {
        assert_eq!(i >= 192, b.need(content_y(i)));
    }
}

#[test]
fn test_all_rows_starting_at2() {
    let b = BitFlag256::all_rows_starting_at(128);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111
    );
    for i in 0..256 {
        assert_eq!(i >= 128, b.need(content_y(i)));
    }
}

#[test]
fn test_all_rows_starting_at3() {
    let b = BitFlag256::all_rows_starting_at(64);
    assert_eq!(
        b.bitset[0],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111
    );
    for i in 0..256 {
        assert_eq!(i >= 64, b.need(content_y(i)));
    }
}

#[test]
fn test_all_rows_starting_at4() {
    let b = BitFlag256::all_rows_starting_at(0);
    assert_eq!(
        b.bitset[0],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111
    );
    assert_eq!(
        b.bitset[1],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111
    );
    for i in 0..256 {
        assert_eq!(true, b.need(content_y(i)));
    }
}

#[test]
fn test_all_rows_multiple() {
    let b = BitFlag256::multiple(&[32, 64, 128, 192, 255]);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b10000000_00000000_00000000_00000000__00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000001
    );
    for i in 0..256 {
        assert_eq!(
            i == 32 || i == 64 || i == 128 || i == 192 || i == 255,
            b.need(content_y(i))
        );
    }
}

#[test]
fn test_all_rows_range_incl() {
    let b = BitFlag256::range_incl(32, 64);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000001__11111111_11111111_11111111_11111111__00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000
    );
    for i in 0..256 {
        assert_eq!(i >= 32 && i <= 64, b.need(content_y(i)), "{}", i);
    }
}

#[test]
fn test_all_rows_range_incl2() {
    let b = BitFlag256::range_incl(32, 192);
    assert_eq!(
        b.bitset[0],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b00000000_00000000_00000000_00000000__00000000_00000000_00000000_000000001__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111
    );
    for i in 0..256 {
        assert_eq!(i >= 32 && i <= 192, b.need(content_y(i)), "{}", i);
    }
}

#[test]
fn test_all_rows_range_incl3() {
    let b = BitFlag256::range_incl(32, 32);
    assert_eq!(
        b.bitset[0],
        0b00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000
    );
    assert_eq!(
        b.bitset[1],
        0b00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000__00000000_00000000_00000000_00000000
    );
    for i in 0..256 {
        assert_eq!(i == 32, b.need(content_y(i)), "{}", i);
    }
}

#[test]
fn test_all_rows_range_incl4() {
    let b = BitFlag256::range_incl(0, 255);
    assert_eq!(
        b.bitset[0],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111
    );
    assert_eq!(
        b.bitset[1],
        0b11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111__11111111_11111111_11111111_11111111
    );
    for i in 0..256 {
        assert_eq!(true, b.need(content_y(i)), "{}", i);
    }
}
