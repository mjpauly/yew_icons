use crate :: IconProps ; # [inline (never)] pub fn simple_icons_setapp (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M13.0949 8.1332a.619.619 0 0 1 0-.874l2.7712-2.7733a.619.619 0 0 1 .877 0l2.7703 2.7722a.619.619 0 0 1 0 .8751l-2.7703 2.7722a.619.619 0 0 1-.877 0zm-1.5331-1.5331L8.7906 3.8299a.618.618 0 0 1 0-.877L11.5618.1815a.619.619 0 0 1 .876 0l2.7732 2.7712a.619.619 0 0 1 0 .877L12.4378 6.6a.619.619 0 0 1-.876 0zm0 2.1902a.619.619 0 0 1 .876 0l2.7732 2.7712a.619.619 0 0 1 0 .877l-2.7732 2.7712a.619.619 0 0 1-.876 0l-2.7712-2.7692a.618.618 0 0 1 0-.877zm-4.3044 2.1151L4.4862 8.1332a.619.619 0 0 1 0-.876l2.7712-2.7713a.619.619 0 0 1 .8761 0l2.7722 2.7712a.621.621 0 0 1 0 .8761l-2.7732 2.7722a.619.619 0 0 1-.876 0zm9.4847 2.1902 2.7723 2.7712a.618.618 0 0 1 0 .875l-2.7703 2.7723a.619.619 0 0 1-.876 0l-2.7732-2.7722a.621.621 0 0 1 0-.8751l2.7732-2.7722a.619.619 0 0 1 .875 0zm-4.3043 4.3033 2.7722 2.7722a.618.618 0 0 1 0 .876l-2.7722 2.7713a.619.619 0 0 1-.876 0l-2.7712-2.7712a.619.619 0 0 1 0-.877l2.7712-2.7713a.619.619 0 0 1 .876 0zm-1.532-1.5321a.619.619 0 0 1 0 .875l-2.7723 2.7733a.621.621 0 0 1-.876 0l-2.7723-2.7722a.619.619 0 0 1 0-.8751l2.7722-2.7722a.619.619 0 0 1 .8761 0z" /></ svg > } }