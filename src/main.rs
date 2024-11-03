use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank })
        }
    }

    // Deckをシャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    let mut hand: Vec<Card> = Vec::new();
    // 5枚カードを引く
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    println!("入れ替えたいカードの番号を入力してください(例: 1 2 3)");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap()) // 文字列を数値に変換する
        .collect::<Vec<usize>>();  // Vecに変換する

    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    println!("---Hand---");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }
}
