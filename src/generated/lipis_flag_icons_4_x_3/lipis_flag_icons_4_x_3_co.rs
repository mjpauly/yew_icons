use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_co (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-co" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#ffe800" d = "M0 0h640v480H0z" /> { "
    " } < path fill = "#00148e" d = "M0 240h640v240H0z" /> { "
    " } < path fill = "#da0010" d = "M0 360h640v120H0z" /> { "
  " } </ g > </ svg > } }