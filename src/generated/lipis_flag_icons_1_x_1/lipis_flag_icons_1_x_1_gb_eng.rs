use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_gb_eng (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-gb-eng" viewBox = "0 0 512 512" > < path fill = "#fff" d = "M0 0h512v512H0z" /> { "
  " } < path fill = "#ce1124" d = "M215 0h82v512h-82z" /> { "
  " } < path fill = "#ce1124" d = "M0 215h512v82H0z" /> </ svg > } }