use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_pl (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-pl" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#fff" d = "M640 480H0V0h640z" /> { "
    " } < path fill = "#dc143c" d = "M640 480H0V240h640z" /> { "
  " } </ g > </ svg > } }