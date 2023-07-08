#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Below is the function stub for deal. Add as many helper functions
    as you like, but the deal function should not be modified. Just
    fill it in.
    
    Test your code by running 'cargo test' from the war_rs directory.
*/

fn deal(shuf: &[u8; 52]) -> [u8; 52] 
{
  // deal cards for 2 players as Vectors
  let mut player1: Vec<u8> = Vec::new();
  let mut player2: Vec<u8> = Vec::new();

  // deal cards from last index of deck
  for (i, &card) in shuf.iter().rev().enumerate() 
    {
        // even index
        if i % 2 == 0 {
          player1.push(card);
        } 
        // odd index
        else 
        {
          player2.push(card);
        }
    }
  // Play!
  return play(&mut player1, &mut player2);
}

fn play(player1: &mut Vec<u8>, player2: &mut Vec<u8>) -> [u8;52]
{
  // loop until one player loses
  while !player1.is_empty() && !player2.is_empty()
    {
      // start dealing at index 0
      let card1 = player1.remove(0);
      let card2 = player2.remove(0);

      if card1 == card2
      {
        // War!
         war(card1, card2, player1, player2);
      }
        // call compare function since Ace is a special case
      else if compare(card1, card2)
      {
        // winner takes both cards in descending order
        player1.push(card1);
        player1.push(card2);
      }
      else 
      {
        player2.push(card2);
        player2.push(card1);
      }
    }
  if player1.is_empty() 
    {
      return convert_to_array(player2.to_vec());
    } 
  else 
  {
    return convert_to_array(player1.to_vec());
  }
    
}

fn war(cardA: u8, cardB: u8, player1: &mut Vec<u8>, player2: &mut Vec<u8>) 
{
  // collect all cards in war
  let mut stack: Vec<u8> = Vec::new();
  
  stack.push(cardA);
  stack.push(cardB);
  
  // Perform the war
  loop 
    {
      // Check if any player runs out of cards
      if player1.len() < 2 
      {
        if player1.len() == 1
        {
          // collect the last card (facedown card)
          stack.push(player1.remove(0));
          stack.push(player2.remove(0));
        }
        // player2 wins, takes all card
        player2.extend(sort(&mut stack));
        // exit war
        return;
      } 
      else if player2.len() < 2 
      {
        if player2.len() == 1
        {
          stack.push(player1.remove(0));
          stack.push(player2.remove(0));
        }
        player1.extend(sort(&mut stack));
        return;
      }
      
      // Deal facedown cards
      stack.push(player1.remove(0));
      stack.push(player2.remove(0));

      // Reveal face-up cards
      let card1 = player1.remove(0);
      let card2 = player2.remove(0);
      stack.push(card1);
      stack.push(card2);

      //sort stack in descending order
      stack = sort(&mut stack);
      
      if card1 == card2 
      {
        // Continue the war
        continue;
      }
      // winner gets all cards in war
      else if compare(card1, card2)
      {
        player1.extend(sort(&mut stack));
      } else 
      {
        player2.extend(sort(&mut stack));
      }
        break;
    }
}

fn compare(cardA: u8, cardB: u8) -> bool
// only called after checking that cardA != card B
{
  // check for Aces first
  if cardA == 1
  {
    return true;
  }
  else if cardB == 1
  {
    return false;
  }
  else if cardA > cardB
  {
    return true;
  }
  return false;
}

fn convert_to_array(vec: Vec<u8>) -> [u8; 52] 
{
  // convert winner deck to array
  let mut array: [u8; 52] = [0; 52];

  // assign values through each index
  for (i, val) in vec.iter().enumerate() 
    {
      if i < 52 
      {
        array[i] = *val;
      } 
      else 
      {
        break;
      }
    }
  array
}

fn sort(vec: &mut Vec<u8>) -> Vec<u8>
{
  // sort so that 1 is the largest card
  let mut ordered: Vec<u8> = Vec::new();
  vec.sort();
  vec.reverse();

  // put 1's into the new vector first, then extend the rest
  while vec.contains(&1)
    {
      // 1 is at the last index of vec based on normal sorting -> pop
      ordered.push(vec.pop().unwrap());
    }
  ordered.extend(vec.iter());
  return ordered;
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;