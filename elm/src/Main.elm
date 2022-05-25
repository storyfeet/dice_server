module Main exposing(main)
import Html exposing(..)
import Html.Attributes exposing(..)
import Html.Events exposing(onSubmit)
import Browser 


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



qform : String -> String -> Msg -> List(Html msg) -> Html msg
qform name ac mes content =
    Html.form [action ac , onSubmit mes] 
        [ div []
            (( h2 [] [text name])::content ++ [(qInput name "submit")])
        ]
            
            
loginForm
    = qform "login" "/login" LoginSubmit
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
    | LoginSubmit form.Msg

subscriptions : Model -> Sub msg
subscriptions model =
  Sub.none

main = Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }
