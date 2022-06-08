module Out exposing(..)
import Message exposing(..)
import Html exposing(..)
import MyForms exposing(..)
import Http exposing(..)
import Url.Builder exposing(..)
import AData exposing (aStringDecoder)



type alias OutModel =
    {  form : OutForm

    }

type OutForm
    = Login LoginModel
    | Signup LoginModel


init: OutModel
init = 
    { form = Login {name="" ,password = ""}
    }

view : OutModel -> Html Msg
view lm = 
    case lm.form of
        Login _ -> loginForm
        Signup _ -> signupForm

update: OutMsg -> OutModel -> (OutModel,Cmd Msg)
update msg om = 
    case (msg, om.form) of
        (OSubmit,Login lm) -> (om,loginRequest lm)
        (OSubmit,Signup lm) -> (om,signupRequest lm)
        (OUpdateName s,Login m ) -> ({om |form = Login <| updateName s m},Cmd.none)
        (OUpdateName s,Signup m ) -> ({om |form = Signup <| updateName s m},Cmd.none)
        (OUpdatePass s,Login m ) -> ({om |form = Login <| updatePass s m},Cmd.none)
        (OUpdatePass s,Signup m ) -> ({om |form = Signup <| updatePass s m},Cmd.none)
            
    

type alias LoginModel =
    { name: String
    , password: String
    }
    
loginForm: Html Msg
loginForm
    = qform "login" "/login" (OutMsg OSubmit)
        [ qInput "name" "text" (\s->s |> OUpdateName |> OutMsg)
        , qInput "pass" "password" (\s -> s |> OUpdatePass |> OutMsg)
        ]

signupForm: Html Msg
signupForm
    = qform "Signup" "/new_user" (OSubmit |>OutMsg)
        [ qInput "name" "text" (\s->s|>OUpdateName|>OutMsg)
        , qInput "pass" "password" (\s->s|>OUpdatePass|>OutMsg)
        ]

loginRequest : LoginModel -> Cmd Msg
loginRequest lm =
    Http.get
    { url = absolute ["login"] [string "name" lm.name, string "pass" lm.password]
    , expect = Http.expectJson GotLogin aStringDecoder
    }

        
signupRequest : LoginModel -> Cmd Msg
signupRequest lm =
    Http.get
    { url = absolute ["login"] [string "name" lm.name, string "pass" lm.password]
    , expect = Http.expectJson GotLogin aStringDecoder 
    }
    
