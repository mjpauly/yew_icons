use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_td (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-td" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#000067" d = "M0 0h214v480H0z" /> { "
    " } < path fill = "red" d = "M426 0h214v480H426z" /> { "
    " } < path fill = "#ff0" d = "M214 0h212v480H214z" /> { "
  " } </ g > </ svg > } }