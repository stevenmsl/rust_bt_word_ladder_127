use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Solution {}

impl Solution {
  pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    if begin_word == end_word {
      return 1;
    };
    if !word_list.contains(&end_word) {
      return 0;
    }
    /* end_word must exist in the word_list */
    let word_length = begin_word.len();
    let mut word_list = Self::to_char_hash_set(&word_list);
    let mut queue = VecDeque::from(vec![Self::to_char_vec(&begin_word)]);
    let mut level = 0;
    let target = Self::to_char_vec(&end_word);

    while queue.len() > 0 {
      level = level + 1;
      /*
        - visits neighbours layer by layer
          - Breadth-first search BFS
        - in the beginning, the queue will
          only have the start_word
      */

      /*
        - this is the number of vertices
          we need to visit for a partiular
          layer
      */
      let count = queue.len();

      for _ in 1..=count {
        if let Some(mut word) = queue.pop_front() {
          /*
            - how to explore the neighbors?
            - we use brute-force to generate
              all possible word combinations
              that are only one character
              different from the word we are
              visiting
            - we then test each combination
              with two things
              - is it the same as the target
                - we are done if it is
              - or is it in the word list?
                - if it is, remove the word
                  for the word list. We
                  will not visit it again
                - push it to the queue so we
                  can visit it at the next
                  level to explore its neighbors
          */
          for pos in 0..word_length {
            /*
              - remember what the original
                char is
            */
            let r = word[pos];
            for c in b'a'..=b'z' {
              let ch = c as char;
              if ch == r {
                continue;
              }
              word[pos] = ch;
              /* exit here */
              if word == target {
                /*
                  - why level + 1?
                    - don't forget we are visiting the
                      neighbors of a word, which are
                      at level + 1
                */
                return level + 1;
              } else {
                //println!("{:?} <> {:?}", word, target);
              }

              /*
                - found a neighbor
                  - push it into the queue so
                    can visit it in the next
                    round
                  - remove it from the word_list
                    so we won't visit it again;
                    this has the same effect as
                    marked a word in the word_list
                    as visited
              */

              if word_list.contains(&word) {
                word_list.remove(&word);
                /*
                  - you need to clone the word
                    as we are still borrowing
                    and mutating the word to
                    create new combinations
                */
                queue.push_back(word.clone());
              }
            }
            /*
              - you have to restore it
                before proceeds to the
                next pos
            */
            word[pos] = r;
          }
        } else {
          continue;
        }
      }
    }

    /*
      - can't find the target word
        or you already returned the
        value inside the loop and
        won't reach here
    */
    0
  }

  pub fn to_char_vec(input: &String) -> Vec<char> {
    input.to_ascii_lowercase().chars().collect()
  }

  pub fn to_char_hash_set(list: &Vec<String>) -> HashSet<Vec<char>> {
    let mut hash_set = HashSet::new();
    for word in list.into_iter() {
      hash_set.insert(Self::to_char_vec(word));
    }
    hash_set
  }

  pub fn text_fixture_1() -> Vec<String> {
    vec![
      "hot".to_string(),
      "dot".to_string(),
      "dog".to_string(),
      "lot".to_string(),
      "log".to_string(),
      "cog".to_string(),
    ]
  }

  pub fn text_fixture_2() -> Vec<String> {
    vec![
      "hot".to_string(),
      "dot".to_string(),
      "dog".to_string(),
      "lot".to_string(),
      "log".to_string(),
    ]
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let result = Solution::ladder_length(
      String::from("hit"),
      String::from("cog"),
      Solution::text_fixture_1(),
    );

    assert_eq!(result, 5);
  }

  #[test]
  fn sample_2() {
    let result = Solution::ladder_length(
      String::from("hit"),
      String::from("cog"),
      Solution::text_fixture_2(),
    );

    assert_eq!(result, 0);
  }
}
