use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_chevrons_up (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < polyline points = "17 11 12 6 7 11" /> { "
  " } < polyline points = "17 18 12 13 7 18" /> </ svg > } }