module Main exposing(main)
import Html exposing(..)
import Html.Attributes exposing(..)
import Browser 
import MyForms exposing(..)
import Message exposing(..)
import Out exposing(OutModel)
import AData exposing(AData)
import Http exposing (Error)
import Err exposing (errorToString)




type alias Model =
    { loginStatus : LoginStatus
    , errs : List Error
    }



type LoginStatus
    = Out OutModel
    | In (AData String)



    

init : () -> (Model ,Cmd Msg)
init _ = 
    ({ loginStatus= Out Out.init
      ,  errs = []
    }, Cmd.none)




update: Msg -> Model -> (Model ,Cmd Msg)
update mes mod = 
    case (mes, mod.loginStatus) of 
        (OutMsg m,Out os)-> 
            let (l,c ) = Out.update m os in ({mod | loginStatus = Out l}, c)
        (GotLogin (Ok ad),_) -> ({mod|loginStatus = In ad} ,Cmd.none)
        (GotSignup (Ok ad),_) -> ({mod|loginStatus = In ad} ,Cmd.none)
        (GotLogin (Err e),_) -> ({mod |errs = e :: mod.errs} , Cmd.none)
        _ -> (mod ,Cmd.none)
        
-- VIEW      
            

view : Model -> Html Msg
view md = div [] 
    [ h1 [] [text "Elm Dice"]
    , case md.loginStatus of
        Out m -> Out.view m
        In a -> p [] [text ("welcome " ++ a.data)]
    , md.errs |> List.map (\e -> p [] [errorToString e |> text]) |> div []
    ]

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions _ =
  Sub.none


-- Main

main : Program () Model Msg
main = Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }
