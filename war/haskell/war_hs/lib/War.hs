module War (deal) where

import Data.List

deal :: [Int] -> [Int]
deal shuf = do
  let (player1, player2) = deal_cards shuf
  -- play!
  play player1 player2 []

-- deal cards to players in alternating sequence
deal_cards :: [Int] -> ([Int], [Int])
-- convert aces to 14 before dealing
deal_cards deck = deal_cards' (convert deck) [] []
    where
      deal_cards' [] player1 player2 = (player1, player2)
      deal_cards' (h : t) player1 player2 = deal_cards' (tail t) (h : player1) (head t : player2)

play :: [Int] -> [Int] -> [Int] -> [Int]
-- when both players have empty stacks
play [] [] stack = convert_back stack
-- when a player wins, convert aces back to 1 before return
play player1 [] stack = convert_back (player1 ++ reverse (sort stack))
play [] player2 stack = convert_back (player2 ++ reverse (sort stack))
-- play
play (h1 : t1) (h2 : t2) stack
  -- war!
  | h1 == h2 =
      -- check for empty stacks
      if null t1 || null t2 then
        play t1 t2 (reverse (sort (stack ++ [h1] ++ [h2])))
      else
        play (tail t1) (tail t2) (reverse (sort (stack ++ [h1] ++ [h2] ++ [head t1] ++ [head t2])))
  -- normal play
  | h1 > h2 = play (t1 ++ reverse (sort (stack ++ [h1] ++ [h2]))) t2 []
  | otherwise = play t1 (t2 ++ reverse (sort (stack ++ [h1] ++ [h2]))) []

-- convert aces 1 -> 14 for easier sorting/ comparing 
convert :: [Int] -> [Int]
convert = map (\x -> if x == 1 then 14 else x)

-- convert aces 14 -> 1
convert_back :: [Int] -> [Int]
convert_back = map (\x -> if x == 14 then 1 else x)
