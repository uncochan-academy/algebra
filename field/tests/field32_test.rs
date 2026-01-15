use field32::field32::{create_number, gyakugen, takousiki, Field};

// 掛け算のテスト: a * 1 = a
#[test]
fn test_kakeru_identity() {
    let a = create_number(0b10101); // x⁴ + x² + 1
    let one = create_number(1);
    let result = a.kakeru(one);
    assert_eq!(result.value, a.value);
}

// 掛け算のテスト: a * 0 = 0
#[test]
fn test_kakeru_zero() {
    let a = create_number(0b11011);
    let zero = create_number(0);
    let result = a.kakeru(zero);
    assert_eq!(result.value, 0);
}

// 掛け算の交換法則: a * b = b * a
#[test]
fn test_kakeru_commutative() {
    let a = create_number(0b101); // x² + 1
    let b = create_number(0b1101); // x³ + x² + 1
    let ab = a.kakeru(b);
    let ba = b.kakeru(a);
    assert_eq!(ab.value, ba.value);
}

// 逆元のテスト: a * a⁻¹ = 1
#[test]
fn test_gyakugen() {
    let a = create_number(0b10); // x
    let inv = gyakugen(a).unwrap();
    let one = a.kakeru(inv);
    assert_eq!(one.value, 1);
}

// 逆元のテスト: 別の値でも確認
#[test]
fn test_gyakugen_another() {
    let a = create_number(0b111); // x² + x + 1
    let inv = gyakugen(a).unwrap();
    let one = a.kakeru(inv);
    assert_eq!(one.value, 1);
}

// 0の逆元はエラー
#[test]
fn test_gyakugen_zero_error() {
    let zero = create_number(0);
    let result = gyakugen(zero);
    assert!(result.is_err());
}

// takousiki関数のテスト
#[test]
fn test_takousiki() {
    let a = create_number(0b101); // x² + 1
    let s = takousiki(a);
    assert_eq!(s, "x² + 1");
}

// takousiki: 0の場合
#[test]
fn test_takousiki_zero() {
    let zero = create_number(0);
    let s = takousiki(zero);
    assert_eq!(s, "0");
}
