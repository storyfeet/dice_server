module Main exposing(main)
import Html exposing(..)
import Html.Attributes exposing(..)

type alias Model = 
    {name:String}


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
            
            



view = div [] 
    [ h1 [] [text "Elm Dice"]
    , qform "login" "/login" 
        [ qInput "name" "text"
        , qInput "pass" "password"
        ]
    ]


main = view
