module Message exposing(..)

type Msg
    = Happy String
    | Sad
    | LoginUpdate FormUpdate
    | SignupUpdate FormUpdate

type FormUpdate 
    = Name String
    | Pass String
    | Other String
