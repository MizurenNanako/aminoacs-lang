use std::io::Read;

use jieba_rs::Jieba;
use pinyin::ToPinyin;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("unable to read stdin");
    let jieba = Jieba::new();
    let cut = jieba.cut(input.as_str(), false);
    let mut v = String::new();

    for w in &cut {
        print!("{w} ")
    }
    println!();

    fn get_pinyin(word: &str) -> (String, bool) {
        let mut t = String::new();
        let mut flag = true;
        for yin in word.to_pinyin() {
            match yin {
                Some(yin) => t.push_str(yin.with_tone()),
                None => {
                    t.push_str(word);
                    flag = false
                }
            }
        }
        let tt: String = t.chars().rev().collect();
        (tt, flag)
    }

    enum StrState {
        Init,
        Punct,
        Word,
    }

    let mut last_state = StrState::Init;

    for word in cut.iter() {
        match &mut get_pinyin(word) {
            (word, true) => {
                if let StrState::Word = last_state {
                    v.push(' ')
                }
                v.push_str(&mut word.clone());
                last_state = StrState::Word;
            }
            (punct, false) => {
                v.push_str(&mut punct.clone());
                last_state = StrState::Punct;
            }
        }
    }

    println!("{v}")
}
