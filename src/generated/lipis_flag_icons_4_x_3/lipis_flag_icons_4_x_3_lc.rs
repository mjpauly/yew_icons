use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_lc (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-lc" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#65cfff" d = "M0 0h640v480H0z" /> { "
    " } < path fill = "#fff" d = "m318.9 42 162.7 395.3-322.6.9L318.9 42z" /> { "
    " } < path d = "m319 96.5 140.8 340-279 .8L319 96.5z" /> { "
    " } < path fill = "#ffce00" d = "m318.9 240.1 162.7 197.6-322.6.5 159.9-198.1z" /> { "
  " } </ g > </ svg > } }