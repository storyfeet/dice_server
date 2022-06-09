module Message exposing(..)
import Http
import AData exposing( AData)

type Msg
    = Happy String
    | GotLogin (Result Http.Error (AData String))
    | GotSignup (Result Http.Error (AData String))
    | OutMsg OutMsg

type OutMsg
    = OUpdateName String
    | OUpdatePass String
    | OLoginSelected
    | OSignupSelected
    | OSubmit


type InMessage
    = Submit
