use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_cr (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-cr" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#0000b4" d = "M0 0h512v512H0z" /> { "
    " } < path fill = "#fff" d = "M0 80.5h512v343.7H0z" /> { "
    " } < path fill = "#d90000" d = "M0 168.2h512v168.2H0z" /> { "
  " } </ g > </ svg > } }