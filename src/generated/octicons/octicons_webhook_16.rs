use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn octicons_webhook_16 (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 16 16" >< path d = "M5.5 4.25a2.25 2.25 0 014.5 0 .75.75 0 001.5 0 3.75 3.75 0 10-6.14 2.889l-2.272 4.258a.75.75 0 001.324.706L7 7.25a.75.75 0 00-.309-1.015A2.25 2.25 0 015.5 4.25z" />< path d = "M7.364 3.607a.75.75 0 011.03.257l2.608 4.349a3.75 3.75 0 11-.628 6.785.75.75 0 01.752-1.299 2.25 2.25 0 10-.033-3.88.75.75 0 01-1.03-.256L7.107 4.636a.75.75 0 01.257-1.03z" />< path d = "M2.9 8.776A.75.75 0 012.625 9.8 2.25 2.25 0 106 11.75a.75.75 0 01.75-.751h5.5a.75.75 0 010 1.5H7.425a3.751 3.751 0 11-5.55-3.998.75.75 0 011.024.274z" /></ svg > } }