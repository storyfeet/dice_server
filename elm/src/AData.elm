module AData exposing(..)
import Json.Decode exposing (map2,map3,Decoder,field,string,int)


type alias AData dt = 
    { auth : InAuth
    , data : dt
    }

type alias InAuth =
    { k:String
    , expires:Int
    , data: String
    }

authDecoder : Decoder InAuth
authDecoder = 
    map3 InAuth
        (field "k" string)
        (field "expires" int)
        (field "data" string)


aDecoder : Decoder a -> Decoder (AData a)
aDecoder dc = 
    map2 (AData)
        (field "auth" authDecoder)
        (field "data" dc)

aStringDecoder : Decoder (AData String)
aStringDecoder =
    aDecoder string

