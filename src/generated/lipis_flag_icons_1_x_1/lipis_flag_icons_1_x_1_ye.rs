use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ye (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-ye" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#fff" d = "M0 0h512v504.3H0z" /> { "
    " } < path fill = "#f10600" d = "M0 0h512v167.9H0z" /> { "
    " } < path d = "M0 344.1h512V512H0z" /> { "
  " } </ g > </ svg > } }