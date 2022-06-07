module In exposing(..)
import Json.Decode exposing (map3,Decoder,field,int,string,dict)


type alias AData dt = 
    { auth : InAuth
    , data : dt
    }

type alias InAuth =
    { k:String
    , expires:String
    , data: String
    }

authDecoder : Decoder InAuth
authDecoder = 
    map3 InAuth
        (field "k" string)
        (field "expires" string)
        (field "data" string)

aStringDecoder : Decoder (AData String)
aStringDecoder =
    map2 (AData String)
        (dict "auth" authDecoder)
        (field "data" string)

