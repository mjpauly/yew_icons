use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_tt (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-tt" viewBox = "0 0 512 512" > < path fill = "#fff" d = "M0 0h512v512H0z" style = "width:0" /> { "
  " } < g fill - rule = "evenodd" > { "
    " } < path fill = "#e00000" d = "M371 512 0 1v510.7l371 .3zM141 0l371 511V.2L141 0z" /> { "
    " } < path d = "M22.2.2h94.9l374.5 511.3h-97.9L22.2.2z" /> { "
  " } </ g > </ svg > } }