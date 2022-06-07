module Message exposing(..)
import Http

type Msg
    = Happy String
    | GotLogin (Result Http.Error String)
    | GotSignup (Result Http.Error String)
    | OutMsg OutMsg

type OutMsg
    = OUpdateName String
    | OUpdatePass String
    | OSubmit


type InMessage
    = Submit
