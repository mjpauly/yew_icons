use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ro (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-ro" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#00319c" d = "M0 0h170.7v512H0z" /> { "
    " } < path fill = "#ffde00" d = "M170.7 0h170.6v512H170.7z" /> { "
    " } < path fill = "#de2110" d = "M341.3 0H512v512H341.3z" /> { "
  " } </ g > </ svg > } }