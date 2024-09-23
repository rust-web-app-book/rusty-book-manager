use mockall::predicate::*;
use mockall::*;
#[automock]
trait 任意のトレイト名 {
    fn 任意のメソッド名(&self, x: u32) -> u32;
}
fn トレイトオブジェクトを引数に取る関数(
    x: &dyn 任意のトレイト名, v: u32
) -> u32 {
    x.任意のメソッド名(v)
}
fn main() {
    let mut mock = Mock任意のトレイト名::new();
    mock.expect_任意のメソッド名().returning(|x| x + 1);
    assert_eq!(10, トレイトオブジェクトを引数に取る関数(&mock, 9));
}
