use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_gg (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-gg" viewBox = "0 0 512 512" > < path fill = "#fff" d = "M0 0h512v512H0z" /> { "
  " } < path fill = "#e8112d" d = "M192 0h128v512H192z" /> { "
  " } < path fill = "#e8112d" d = "M0 187.7h512v136.6H0z" /> { "
  " } < path id = "a" fill = "#f9dd16" d = "m46 305.8 23.3-25h210v-49.7h-210L46 206.2z" /> { "
  " } < use href = "#a" width = "36" height = "24" transform = "matrix(0 1.06667 -.9375 0 496 -17)" /> { "
  " } < use href = "#a" width = "36" height = "24" transform = "matrix(0 -1.06667 .9375 0 16 529)" /> { "
  " } < use href = "#a" width = "36" height = "24" transform = "rotate(180 256 256)" /> </ svg > } }