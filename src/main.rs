use rust_bt_word_ladder_127::Solution;

fn main() {
    let word_list = Solution::text_fixture_2();

    println!(
        "{:?}",
        Solution::ladder_length(String::from("hit"), String::from("cog"), word_list)
    );
}
