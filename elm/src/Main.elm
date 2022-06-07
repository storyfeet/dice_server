module Main exposing(main)
import Html exposing(..)
import Html.Attributes exposing(..)
import Browser 
import MyForms exposing(..)
import Message exposing(..)
import Out exposing(OutModel)
import In exposing(InAuth)




type alias Model =
    { loginStatus : LoginStatus
    }



type LoginStatus
    = Out OutModel
    | In InAuth



    

init : () -> (Model ,Cmd Msg)
init _ = 
    ({ loginStatus= Out Out.init
    }, Cmd.none)




update: Msg -> Model -> (Model ,Cmd Msg)
update mes mod = 
    case (mes, mod.loginStatus) of 
        (OutMsg m,Out os)-> 
            let (l,c ) = Out.update m os in ({mod | loginStatus = Out l}, c)
        (GotLogin (Ok s),_) -> ({mod|loginStatus = In {key="", name=s}} ,Cmd.none)
        (GotSignup (Ok s),_) -> ({mod|loginStatus = In {key="", name=s}} ,Cmd.none)
        _ -> (mod ,Cmd.none)
        

            
            

view : Model -> Html Msg
view md = div [] 
    [ h1 [] [text "Elm Dice"]
    , case md.loginStatus of
        Out m -> Out.view m
        In a -> p [] [text ("welcome " ++ a.name)]
    ]



subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none

main = Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }
