
// 以下の名前のモジュール(名前空間)を定義すると同時に、
// そのモジュールの内容は、front_of_house.rsに定義してある旨をコンパイラに伝える。
// crate::crate_study_2::lib_module_1
mod lib_module_1;

pub use crate::lib_module_1::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
