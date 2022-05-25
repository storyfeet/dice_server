module MyForms exposing(..)
import Message exposing(..)

import Html exposing(..)
import Html.Attributes exposing(..)
import Html.Events exposing(..)


qInput : String -> String -> (String -> Msg) -> Html Msg
qInput nam typ mes= 
    div []
    [ text (nam ++":")
    , input [type_ typ, name nam, onInput mes] []
    ]



qform : String -> String -> Msg -> List(Html Msg) -> Html Msg
qform nam ac mes content =
    let frontMid  = (h2 [] [text nam])::content
        full = frontMid ++ [(input [type_ "submit", value nam] []) ]
    in
    Html.form [action ac,onSubmit mes ] 
        [ div []
            full  
        ] 

type alias LoginModel =
    { name: String
    , password: String
    }

    
type alias CreateModel =
    { name: String
    , password: String
    }


 
