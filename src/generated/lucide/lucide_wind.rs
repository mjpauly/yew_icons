use crate :: IconProps ; # [inline (never)] pub fn lucide_wind (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M17.7 7.7a2.5 2.5 0 1 1 1.8 4.3H2" /> < path d = "M9.6 4.6A2 2 0 1 1 11 8H2" /> < path d = "M12.6 19.4A2 2 0 1 0 14 16H2" /> </ svg > } }