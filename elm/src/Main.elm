module Main exposing(main)
import Html exposing(..)
import Html.Attributes exposing(..)
import Browser exposing (element)


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
update _ md = 
    (md, Cmd.none)

qInput nam typ = div []
    [ text (nam ++":")
    , input [type_ typ, name nam] []
    ]



qform : String -> String -> List(Html msg) -> Html msg
qform name ac content =
    Html.form [action ac] 
        [ div []
            (( h2 [] [text name])::content ++ [(qInput name "submit")])
        ]
            
            
loginForm
    = qform "login" "/login" 
        [ qInput "name" "text"
        , qInput "pass" "password"
        ]



view : Model -> Html Msg
view md = div [] 
    [ h1 [] [text "Elm Dice"]
    , case md.login of
        Out -> loginForm
        In a -> p [] [text ("welcome " ++ a.name)]
    ]


type Msg
    = Happy
    | Sad

subscriptions : Model -> Sub msg
subscriptions model =
  Sub.none

main = Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }
