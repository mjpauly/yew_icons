use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_pl (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-pl" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#fff" d = "M512 512H0V0h512z" /> { "
    " } < path fill = "#dc143c" d = "M512 512H0V256h512z" /> { "
  " } </ g > </ svg > } }