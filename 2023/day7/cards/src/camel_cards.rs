pub mod hand {
  use std::collections::HashMap;
  use core::cmp::Ordering;

  #[derive(PartialEq, PartialOrd)]
  #[derive(Debug)]
  pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
  }

  #[derive(Debug)]
  pub struct Hand {
    pub cards: String,
    pub bid: u32
  }

  impl Hand {
    pub fn get_type(&self) -> HandType {
      let result = match self {
        x if x.is_five_of_a_kind() => HandType::FiveOfAKind,
        x if x.is_four_of_a_kind() => HandType::FourOfAKind,
        x if x.is_full_house() => HandType::FullHouse,
        x if x.is_three_of_a_kind() => HandType::ThreeOfAKind,
        x if x.is_two_pair() => HandType::TwoPair,
        x if x.is_one_pair() => HandType::OnePair,
        _ => HandType::HighCard
      };

      result
    }

    pub fn is_five_of_a_kind(&self) -> bool {
      let mut map = HashMap::new();
      
      for c in self.cards.chars() {
        map.entry(c).and_modify(|count| *count += 1).or_insert(1);
      }

      let mut joker_count = 0;
      if map.contains_key(&'J') {
        joker_count = map[&'J'];
      }

      map.remove(&'J');

      if map.into_values().filter(|n| *n + joker_count == 5).count() == 1 {
        return true;
      }

      false
    }

    pub fn is_four_of_a_kind(&self) -> bool {
      let mut map = HashMap::new();

      for c in self.cards.chars() {
        map.entry(c).and_modify(|count| *count += 1).or_insert(1);
      }

      let mut joker_count = 0;
      if map.contains_key(&'J') {
        joker_count = map[&'J'];
      }

      map.remove(&'J');

      if map.into_values().filter(|n| *n + joker_count == 4).count() == 1 {
        return true;
      }

      false
    }

    pub fn is_full_house(&self) -> bool {
      let mut map = HashMap::new();

      for c in self.cards.chars() {
        map.entry(c).and_modify(|count| *count += 1).or_insert(1);
      }

      map.remove(&'J');

      if map.len() == 2 {
        return true;
      }
      
      false
    }

    pub fn is_three_of_a_kind(&self) -> bool {
      let mut map = HashMap::new();

      for c in self.cards.chars() {
        map.entry(c).and_modify(|count| *count += 1).or_insert(1);
      }

      let mut joker_count = 0;
      if map.contains_key(&'J') {
        joker_count = map[&'J'];
      }

      map.remove(&'J');

      if map.len() > 2 {
        if map.into_values().filter(|n| *n + joker_count == 3).count() == 1 {
          return true;
        }
      }

      false
    }

    pub fn is_two_pair(&self) -> bool {
      let mut map = HashMap::new();

      for c in self.cards.chars() {
        map.entry(c).and_modify(|count| *count += 1).or_insert(1);
      }

      let mut joker_count = 0;
      if map.contains_key(&'J') {
        joker_count = map[&'J'];
      }

      map.remove(&'J');

      if map.into_values().filter(|n| *n + joker_count == 2).count() == 2 {
        return true;
      }

      false
    }

    pub fn is_one_pair(&self) -> bool {
      let mut map = HashMap::new();

      for c in self.cards.chars() {
        map.entry(c).and_modify(|count| *count += 1).or_insert(1);
      }

      let mut joker_count = 0;
      if map.contains_key(&'J') {
        joker_count = map[&'J'];
      }

      map.remove(&'J');

      if map.into_values().filter(|n| *n + joker_count == 2).count() == 1 {
        return true;
      }

      false
    }
  }

  impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
      fn points_from_char(c: char) -> u32 {
        match c {
          x if x.is_digit(10) => x.to_digit(10).unwrap(),
          x if x == 'T' => 10,
          x if x == 'J' => 11,
          x if x == 'Q' => 12,
          x if x == 'K' => 13,
          x if x == 'A' => 14,
          x if x == 'J' => 1,
          _ => 0
        }
      }

      if self.get_type() < other.get_type() {
        return Ordering::Less;
      }
      if self.get_type() > other.get_type() {
        return Ordering::Greater;
      }

      for (lhs, rhs) in self.cards.chars().zip(other.cards.chars()) {
        let a = points_from_char(lhs);
        let b = points_from_char(rhs);

        if a == b {
          continue;
        }

        return a.cmp(&b);
      }

      return Ordering::Equal;
    }    
  }

  impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
    }
  }

  impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
      self.cards == other.cards
    }
  }

  impl Eq for Hand {

  }
}

#[cfg(test)]
mod tests {
  use super::hand::*;

  #[test]
  fn get_type() {
    let five_kind_hand = Hand {
      cards: String::from("AAAAA"),
      bid: 0
    };
    let four_kind_hand = Hand {
      cards: String::from("AA8AA"),
      bid: 0
    };
    let full_house_hand = Hand {
      cards: String::from("23332"),
      bid: 0
    };
    let three_kind_hand = Hand {
      cards: String::from("TTT98"),
      bid: 0
    };
    let two_pair_hand = Hand {
      cards: String::from("23432"),
      bid: 0
    };
    let one_pair_hand = Hand {
      cards: String::from("A23A4"),
      bid: 0
    };
    let high_card_hand = Hand {
      cards: String::from("23456"),
      bid: 0
    };
    
    let joker_five = Hand {
      cards: String::from("AAJAA"),
      bid: 0
    };
    let joker_four = Hand {
      cards: String::from("AAJAK"),
      bid: 0
    };
    let joker_full_house = Hand {
      cards: String::from("23J32"),
      bid: 0
    };
    let joker_three = Hand {
      cards: String::from("TTJ98"),
      bid: 0
    };
    let joker_two = Hand {
      cards: String::from("12JJ"),
      bid: 0
    };
    let joker_one = Hand {
      cards: String::from("1234J"),
      bid: 0
    };

    assert_eq!(HandType::FiveOfAKind, five_kind_hand.get_type());
    assert_eq!(HandType::FourOfAKind, four_kind_hand.get_type());
    assert_eq!(HandType::FullHouse, full_house_hand.get_type());
    assert_eq!(HandType::ThreeOfAKind, three_kind_hand.get_type());
    assert_eq!(HandType::TwoPair, two_pair_hand.get_type());
    assert_eq!(HandType::OnePair, one_pair_hand.get_type());
    assert_eq!(HandType::HighCard, high_card_hand.get_type());
    assert_eq!(HandType::FiveOfAKind, joker_five.get_type());
    assert_eq!(HandType::FourOfAKind, joker_four.get_type());
    assert_eq!(HandType::FullHouse, joker_full_house.get_type());
    assert_eq!(HandType::ThreeOfAKind, joker_three.get_type());
    assert_eq!(HandType::TwoPair, joker_two.get_type());
    assert_eq!(HandType::OnePair, joker_one.get_type())
  }

  #[test]
  fn is_five_of_a_kind() {
    let valid_hand = Hand {
      cards: String::from("AAAAA"),
      bid: 0
    };
    
    let invalid_hand = Hand {
      cards: String::from("AAAKK"),
      bid: 0
    };
    
    let joker_hand = Hand {
      cards: String::from("AAAAJ"),
      bid: 0
    };
    
    assert_eq!(true, valid_hand.is_five_of_a_kind());
    assert_eq!(false, invalid_hand.is_five_of_a_kind());
    assert_eq!(true, joker_hand.is_five_of_a_kind());
  }
  
  #[test]
  fn is_four_of_a_kind() {
    let valid_hand = Hand {
      cards: String::from("AAAAK"),
      bid: 0
    };
    
    let invalid_hand = Hand {
      cards: String::from("AAAAA"),
      bid: 0
    };
    
    let joker_hand = Hand {
      cards: String::from("AAAJK"),
      bid: 0
    };
    
    assert_eq!(true, valid_hand.is_four_of_a_kind());
    assert_eq!(false, invalid_hand.is_four_of_a_kind());
    assert_eq!(true, joker_hand.is_four_of_a_kind());
  }

  #[test]
  fn is_full_house() {
    let valid_hand = Hand {
      cards: String::from("AAAKK"),
      bid: 0
    };
    
    let invalid_hand = Hand {
      cards: String::from("AAAKQ"),
      bid: 0
    };
    
    let joker_hand = Hand {
      cards: String::from("AAAJQ"),
      bid: 0
    };
    
    
    assert_eq!(true, valid_hand.is_full_house());
    assert_eq!(false, invalid_hand.is_full_house());
    assert_eq!(true, joker_hand.is_full_house());
  }

  #[test]
  fn is_three_of_a_kind() {
    let valid_hand = Hand {
      cards: String::from("TTT98"),
      bid: 0
    };
    
    let invalid_hand = Hand {
      cards: String::from("TTT99"),
      bid: 0
    };

    let joker_hand = Hand {
      cards: String::from("TTJ98"),
      bid: 0
    };
    
    assert_eq!(true, valid_hand.is_three_of_a_kind());
    assert_eq!(false, invalid_hand.is_three_of_a_kind());
    assert_eq!(true, joker_hand.is_three_of_a_kind());
  }

  #[test]
  fn is_two_pair() {
    let valid_hand = Hand {
      cards: String::from("23432"),
      bid: 0
    };

    let invalid_hand = Hand {
      cards: String::from("12345"),
      bid: 0
    };

    let joker_hand = Hand {
      cards: String::from("12J34"),
      bid: 0
    };
    
    assert_eq!(true, valid_hand.is_two_pair());
    assert_eq!(false, invalid_hand.is_two_pair());
    assert_eq!(true, joker_hand.is_two_pair());
  }

  #[test]
  fn is_one_pair() {
    let valid_hand = Hand {
      cards: String::from("A23A4"),
      bid: 0
    };
    
    let invalid_hand = Hand {
      cards: String::from("A23K4"),
      bid: 0
    };

    let joker_hand = Hand {
      cards: String::from("AJ23K"),
      bid: 0
    };
    
    assert_eq!(true, valid_hand.is_one_pair());
    assert_eq!(false, invalid_hand.is_one_pair());
    assert_eq!(true, joker_hand.is_one_pair());
  }

  #[test]
  fn is_equal() {
    let hand1 = Hand {
      cards: String::from("A23A4"),
      bid: 0
    };

    let hand2 = Hand {
      cards: String::from("A23A4"),
      bid: 0
    };

    let less_hand = Hand {
      cards: String::from("AAAAK"),
      bid: 0
    };
    
    let great_hand = Hand {
      cards: String::from("AAAAA"),
      bid: 0
    };

    let joker_hand = Hand {
      cards: String::from("JAAAA"),
      bid: 0
    };
    
    assert_eq!(hand1, hand2);
    assert_eq!(true, less_hand < great_hand);
    assert_eq!(true, great_hand > less_hand);
    assert_eq!(true, joker_hand < great_hand);
    assert_eq!(false, joker_hand < less_hand);
  }
}