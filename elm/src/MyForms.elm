module MyForms exposing(..)
import Message exposing(..)

import Html exposing(..)
import Html.Attributes exposing(..)
import Html.Events exposing(..)

import Http exposing(..)
import Url.Builder exposing(..)




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




updateName: a ->{ b |name:a}->{ b |name:a}
updateName s f
    = {f | name=s}
updatePass: a ->{ b |password:a}->{ b |password:a}
updatePass p f 
    = {f| password=p}




