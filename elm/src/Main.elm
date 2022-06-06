module Main exposing(main)
import Html exposing(..)
import Html.Attributes exposing(..)
import Browser 
import MyForms exposing(..)
import Message exposing(..)


type alias Model =
    { login : Login
    
    }


type alias Auth =
    { name:String
    , key:String
    }

type Login 
    = Out LoginModel
    | In Auth



    

init : () -> (Model ,Cmd Msg)
init _ = 
    ({ login= Out {name="", password=""}
    }, Cmd.none)




update: Msg -> Model -> (Model ,Cmd Msg)
update mes mod = 
    case (mes, mod.login) of 
        (LoginSubmit,Out lm) -> (mod,loginRequest lm)
        (GotLogin (Ok s),_) -> ({mod|login = In {key="", name=s}} ,Cmd.none)
        _ -> (mod ,Cmd.none)

            
            
loginForm
    = qform "login" "/login" LoginSubmit
        [ qInput "name" "text" (\s ->LoginUpdate <| Name s)
        , qInput "pass" "password" (\s -> LoginUpdate <| Name s)
        ]

    

view : Model -> Html Msg
view md = div [] 
    [ h1 [] [text "Elm Dice"]
    , case md.login of
        Out _ -> loginForm
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
