use crate :: IconProps ; # [inline (never)] pub fn lucide_cpu (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "4" y = "4" width = "16" height = "16" rx = "2" ry = "2" /> < rect x = "9" y = "9" width = "6" height = "6" /> < line x1 = "9" y1 = "2" x2 = "9" y2 = "4" /> < line x1 = "15" y1 = "2" x2 = "15" y2 = "4" /> < line x1 = "9" y1 = "21" x2 = "9" y2 = "22" /> < line x1 = "15" y1 = "20" x2 = "15" y2 = "22" /> < line x1 = "20" y1 = "9" x2 = "22" y2 = "9" /> < line x1 = "20" y1 = "14" x2 = "22" y2 = "14" /> < line x1 = "2" y1 = "9" x2 = "4" y2 = "9" /> < line x1 = "2" y1 = "14" x2 = "4" y2 = "14" /> </ svg > } }