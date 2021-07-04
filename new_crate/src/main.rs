// メインファイル

// モジュールをインポートする。
use new_crate::module_a;
use new_crate::module_b;

/**
 * メイン関数
 */
fn main() {

}

/**
 * テスト用の関数
 */
#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(0, 1));
    assert_eq!(1, add(1, 0));
    assert_eq!(2, add(1, 1));
}

/**
 * 足し算関数
 */
pub fn add(x:i32, y:i32) -> i32 {
    return x + y;
}