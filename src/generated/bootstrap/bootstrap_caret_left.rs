use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn bootstrap_caret_left (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/twbs/icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" class = "bi bi-caret-left" viewBox = "0 0 16 16" > < path d = "M10 12.796V3.204L4.519 8 10 12.796zm-.659.753-5.48-4.796a1 1 0 0 1 0-1.506l5.48-4.796A1 1 0 0 1 11 3.204v9.592a1 1 0 0 1-1.659.753z" /> </ svg > } }