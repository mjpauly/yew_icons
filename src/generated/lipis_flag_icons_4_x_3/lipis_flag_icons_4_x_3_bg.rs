use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_bg (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-bg" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#d62612" d = "M0 320h640v160H0z" /> { "
    " } < path fill = "#fff" d = "M0 0h640v160H0z" /> { "
    " } < path fill = "#00966e" d = "M0 160h640v160H0z" /> { "
  " } </ g > </ svg > } }