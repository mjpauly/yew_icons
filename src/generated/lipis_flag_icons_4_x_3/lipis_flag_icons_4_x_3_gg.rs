use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_gg (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-gg" viewBox = "0 0 640 480" > < path fill = "#fff" d = "M0 0h640v480H0z" /> { "
  " } < path fill = "#e8112d" d = "M256 0h128v480H256z" /> { "
  " } < path fill = "#e8112d" d = "M0 176h640v128H0z" /> { "
  " } < path id = "a" fill = "#f9dd16" d = "m110 286.7 23.3-23.4h210v-46.6h-210L110 193.3z" /> { "
  " } < use href = "#a" width = "36" height = "24" transform = "rotate(90 320 240)" /> { "
  " } < use href = "#a" width = "36" height = "24" transform = "rotate(-90 320 240)" /> { "
  " } < use href = "#a" width = "36" height = "24" transform = "rotate(180 320 240)" /> </ svg > } }