// クレート情報を管理するためのファイル
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
// モジュールをそれぞれ宣言する。　
pub mod module_a;
mod module_b;