module Err exposing(..)
import Http exposing(..)
errorToString : Http.Error -> String
errorToString error =
    case error of
        BadUrl url ->
            "The URL " ++ url ++ " was invalid"
        Timeout ->
            "Unable to reach the server, try again"
        NetworkError ->
            "Unable to reach the server, check your network connection"
        BadStatus 500 ->
            "The server had a problem, try again later"
        BadStatus 400 ->
            "Verify your information and try again"
        BadStatus _ ->
            "Unknown error"
        BadBody errorMessage ->
            errorMessage


mapWithN : (number -> a -> b) -> number -> List a -> List b
mapWithN fn n ls =
    case ls of
        [] -> []
        h :: t -> (fn n h) :: mapWithN fn (n+1) t

