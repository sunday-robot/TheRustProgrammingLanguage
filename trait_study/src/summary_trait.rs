pub trait SummaryTrait {
    // 普通のメソッド
    // JavaならString summarize(int maxLength);などとするもの。
    // Rustの場合は、第1引数として"&self"を記述する。("self"を"this"などと書くと文法エラー)
    // 呼び出すときはxx.summarize(10)の様に記述するので、仕様としてカッコ悪いと思う。
    fn summarize(&self, max_length:i32) -> String;

    // 以下の様に"&self"ではなく、"self"としてもエラーにはならないが、おそらく参照私と価私のような違いがあるのだと思うがわからない。
    // よくわからないが、"&"を書かないと、summarize2()に所有権が移ってしまうとのことなのでたぶんNG"
    // fn summarize2(self) -> String;

    // クラスメソッド
    // Javaの場合はstatic String summarizeUtil(int x);などとするもの。
    // Rustの場合は、普通の関数宣言となる。
    fn summarize_util(x: i32) -> String;
}
