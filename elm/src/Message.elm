module Message exposing(..)
import Http

type Msg
    = Happy String
    | LoginSubmit
    | LoginUpdate FormUpdate
    | SignupUpdate FormUpdate
    | GotLogin (Result Http.Error String)

type FormUpdate 
    = Name String
    | Pass String
    | Other String
