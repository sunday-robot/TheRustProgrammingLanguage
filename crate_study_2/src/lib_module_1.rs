// lib.rsから、crate::crate_study_2::lib_module_1モジュールの中身として取り込まれるファイル。
// Cであれば以下のような形で取り込まれるような感じ？
// (lib.cpp)
// namespace crate_study_2 {
//   namespace lib_module_1 {
//     #include "lib_module_1.cpp"
//   }
// }
// C++17では以下のように書けるらしい。
// namespace crate_study_2::lib_module_1 {
//   #include "lib_module_1.cpp"
// }

// 以下で定義しているモジュールのフルネーム(Rustではフルパス名?)は、
// crate::hostting
// ではなく、
// crate::crate_study_2::lib_module_1::hosting
// であることに注意。
//
// C++とも、JavaやC#とも異なり分かりづらい…
pub mod hosting {
    pub fn add_to_waitlist() {}
}
