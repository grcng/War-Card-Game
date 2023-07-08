defmodule War do
  @moduledoc """
    Documentation for War.
  """

  @doc """
    Function stub for deal/1 is given below. Feel free to add
    as many additional helper functions as you want.

    The tests for the deal function can be found in test/war_test.exs.
    You can add your five test cases to this file.

    Run the tester by executing 'mix test' from the war directory
    (the one containing mix.exs)
  """

  def deal(shuf) do
    [player1, player2] = deal_cards(shuf)
    
    # play!
    play(player1, player2, [])
  end

  # deals cards to players
  def deal_cards(deck) do
  # convert aces
    deal_cards(convert(deck), [], [])
  end
  
  def deal_cards([], player1, player2) do
    [player1, player2]
  end
  
  def deal_cards([h | t], player1, player2) do
  # deal in alternating sequence
    deal_cards(tl(t), [h | player1], [hd(t) | player2])
  end

  
  def play([], [], stack) do
    # when players have empty stacks, return the play stack
    convert_back(stack)
  end
  
  def play(player1, [], stack) do
    # convert aces back in the winner's deck
    convert_back(player1++Enum.sort(stack, :desc))
  end
  
  def play([], player2, stack) do
    convert_back(player2++Enum.sort(stack, :desc))
  end
  
  def play([h1 | t1], [h2 | t2], stack) do
    cond do
      # war!
      h1 == h2 ->
        if length(t1) == 0 || length(t2) == 0 do
        # check for empty list
          play(t1, t2, Enum.sort(stack++[h1]++[h2], :desc) )
        else
          # add facedown cards to stack and continue play
          play(tl(t1), tl(t2), Enum.sort(stack++[h1]++[h2]++[hd(t1)]++[hd(t2)], :desc) )
        end
      # normal play
      h1 > h2 ->
        play(t1++(Enum.sort(stack++[h1]++[h2], :desc)), t2, [])
      true ->
        play(t1, t2++(Enum.sort(stack++[h1]++[h2], :desc)), [])
    end
  end
 
  # converts aces to 14 for easier sorting
  def convert(deck) do
    Enum.map(deck, fn
    1 -> 14
    x -> x
    end)
  end

  # converts aces back from 14 to 1
  def convert_back(deck) do
    Enum.map(deck, fn
    14 -> 1
    x -> x
    end)
  end

  

end