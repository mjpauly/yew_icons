use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_ga (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-ga" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#ffe700" d = "M640 480H0V0h640z" /> { "
    " } < path fill = "#36a100" d = "M640 160H0V0h640z" /> { "
    " } < path fill = "#006dbc" d = "M640 480H0V320h640z" /> { "
  " } </ g > </ svg > } }