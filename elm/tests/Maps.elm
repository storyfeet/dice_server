module Maps exposing (testMapN)

import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Test exposing (..)
import Err


letters = ["a","b","c"]
letters2 = [("a",0),("b",1),("c",2)]

tMapN _ =
    Expect.equal (Err.mapWithN (\a n ->(n,a) ) 0 letters) letters2

testMapN = 
    test "Can I make map with N work"
    tMapN

