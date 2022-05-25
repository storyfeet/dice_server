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
    = Out
    | In Auth



    

init : () -> (Model ,Cmd Msg)
init _ = 
    ({ login= Out
    }, Cmd.none)


update: Msg -> Model -> (Model ,Cmd Msg)
update mes mod = 
    case mes of 
        Sad -> 
    (md, Cmd.none)

            
            
loginForm
    = qform "login" "/login" Sad
        [ qInput "name" "text" (\s ->LoginUpdate <| Name s)
        , qInput "pass" "password" (\s -> LoginUpdate <| Name )
        ]

    

view : Model -> Html Msg
view md = div [] 
    [ h1 [] [text "Elm Dice"]
    , case md.login of
        Out -> loginForm
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
