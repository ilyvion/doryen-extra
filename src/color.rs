/* BSD 3-Clause License
 *
 * Copyright © 2019, Alexander Krivács Schrøder <alexschrod@gmail.com>.
 * Copyright © 2008-2019, Jice and the libtcod contributors.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

//! # Color representation and operations.
//!
//! A color is defined by its red, green and blue components in the range 0 to 255.
//! You can use the following predefined colors (hover over a color to see its full name and
//! RGB values:
//!
//! <table class="color">
//! <tbody><tr><td></td><th colspan="8">STANDARD COLORS</th></tr>
//! <tr><td></td><td>DESATURATED</td><td>LIGHTEST</td><td>LIGHTER</td><td>LIGHT</td><td>NORMAL</td><td>DARK</td><td>DARKER</td><td>DARKEST</td></tr>
//! <tr><td>RED</td><td title="DESATURATED_RED (128, 64, 64)" style="background-color: rgb(128, 64, 64); --darkreader-inline-bgcolor:#663333;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_RED (255, 191, 191)" style="background-color: rgb(255, 191, 191); --darkreader-inline-bgcolor:#590000;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_RED (255, 166, 166)" style="background-color: rgb(255, 166, 166); --darkreader-inline-bgcolor:#680000;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_RED (255, 115, 115)" style="background-color: rgb(255, 115, 115); --darkreader-inline-bgcolor:#870000;" data-darkreader-inline-bgcolor=""></td><td title="RED (255, 0, 0)" style="background-color: rgb(255, 0, 0); --darkreader-inline-bgcolor:#cc0000;" data-darkreader-inline-bgcolor=""></td><td title="DARK_RED (191, 0, 0)" style="background-color: rgb(191, 0, 0); --darkreader-inline-bgcolor:#bf0000;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_RED (128, 0, 0)" style="background-color: rgb(128, 0, 0); --darkreader-inline-bgcolor:#800000;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_RED (64, 0, 0)" style="background-color: rgb(64, 0, 0); --darkreader-inline-bgcolor:#400000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>FLAME</td><td title="DESATURATED_FLAME (128, 80, 64)" style="background-color: rgb(128, 80, 64); --darkreader-inline-bgcolor:#664033;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_FLAME (255, 207, 191)" style="background-color: rgb(255, 207, 191); --darkreader-inline-bgcolor:#591600;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_FLAME (255, 188, 166)" style="background-color: rgb(255, 188, 166); --darkreader-inline-bgcolor:#681a00;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_FLAME (255, 149, 115)" style="background-color: rgb(255, 149, 115); --darkreader-inline-bgcolor:#872100;" data-darkreader-inline-bgcolor=""></td><td title="FLAME (255, 63, 0)" style="background-color: rgb(255, 63, 0); --darkreader-inline-bgcolor:#cc3200;" data-darkreader-inline-bgcolor=""></td><td title="DARK_FLAME (191, 47, 0)" style="background-color: rgb(191, 47, 0); --darkreader-inline-bgcolor:#bf2f00;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_FLAME (128, 32, 0)" style="background-color: rgb(128, 32, 0); --darkreader-inline-bgcolor:#802000;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_FLAME (64, 16, 0)" style="background-color: rgb(64, 16, 0); --darkreader-inline-bgcolor:#401000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>ORANGE</td><td title="DESATURATED_ORANGE (128, 96, 64)" style="background-color: rgb(128, 96, 64); --darkreader-inline-bgcolor:#664d33;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_ORANGE (255, 223, 191)" style="background-color: rgb(255, 223, 191); --darkreader-inline-bgcolor:#592d00;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_ORANGE (255, 210, 166)" style="background-color: rgb(255, 210, 166); --darkreader-inline-bgcolor:#683400;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_ORANGE (255, 185, 115)" style="background-color: rgb(255, 185, 115); --darkreader-inline-bgcolor:#874300;" data-darkreader-inline-bgcolor=""></td><td title="ORANGE (255, 127, 0)" style="background-color: rgb(255, 127, 0); --darkreader-inline-bgcolor:#cc6600;" data-darkreader-inline-bgcolor=""></td><td title="DARK_ORANGE (191, 95, 0)" style="background-color: rgb(191, 95, 0); --darkreader-inline-bgcolor:#bf5f00;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_ORANGE (128, 64, 0)" style="background-color: rgb(128, 64, 0); --darkreader-inline-bgcolor:#804000;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_ORANGE (64, 32, 0)" style="background-color: rgb(64, 32, 0); --darkreader-inline-bgcolor:#402000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>AMBER</td><td title="DESATURATED_AMBER (128, 112, 64)" style="background-color: rgb(128, 112, 64); --darkreader-inline-bgcolor:#665933;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_AMBER (255, 239, 191)" style="background-color: rgb(255, 239, 191); --darkreader-inline-bgcolor:#594300;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_AMBER (255, 233, 166)" style="background-color: rgb(255, 233, 166); --darkreader-inline-bgcolor:#684f00;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_AMBER (255, 220, 115)" style="background-color: rgb(255, 220, 115); --darkreader-inline-bgcolor:#876500;" data-darkreader-inline-bgcolor=""></td><td title="AMBER (255, 191, 0)" style="background-color: rgb(255, 191, 0); --darkreader-inline-bgcolor:#cc9900;" data-darkreader-inline-bgcolor=""></td><td title="DARK_AMBER (191, 143, 0)" style="background-color: rgb(191, 143, 0); --darkreader-inline-bgcolor:#bf8f00;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_AMBER (128, 96, 0)" style="background-color: rgb(128, 96, 0); --darkreader-inline-bgcolor:#806000;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_AMBER (64, 48, 0)" style="background-color: rgb(64, 48, 0); --darkreader-inline-bgcolor:#403000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>YELLOW</td><td title="DESATURATED_YELLOW (128, 128, 64)" style="background-color: rgb(128, 128, 64); --darkreader-inline-bgcolor:#666633;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_YELLOW (255, 255, 191)" style="background-color: rgb(255, 255, 191); --darkreader-inline-bgcolor:#595900;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_YELLOW (255, 255, 166)" style="background-color: rgb(255, 255, 166); --darkreader-inline-bgcolor:#686800;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_YELLOW (255, 255, 115)" style="background-color: rgb(255, 255, 115); --darkreader-inline-bgcolor:#878700;" data-darkreader-inline-bgcolor=""></td><td title="YELLOW (255, 255, 0)" style="background-color: rgb(255, 255, 0); --darkreader-inline-bgcolor:#cccc00;" data-darkreader-inline-bgcolor=""></td><td title="DARK_YELLOW (191, 191, 0)" style="background-color: rgb(191, 191, 0); --darkreader-inline-bgcolor:#bfbf00;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_YELLOW (128, 128, 0)" style="background-color: rgb(128, 128, 0); --darkreader-inline-bgcolor:#808000;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_YELLOW (64, 64, 0)" style="background-color: rgb(64, 64, 0); --darkreader-inline-bgcolor:#404000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>LIME</td><td title="DESATURATED_LIME (112, 128, 64)" style="background-color: rgb(112, 128, 64); --darkreader-inline-bgcolor:#596633;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_LIME (239, 255, 191)" style="background-color: rgb(239, 255, 191); --darkreader-inline-bgcolor:#435900;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_LIME (233, 255, 166)" style="background-color: rgb(233, 255, 166); --darkreader-inline-bgcolor:#4f6800;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_LIME (220, 255, 115)" style="background-color: rgb(220, 255, 115); --darkreader-inline-bgcolor:#658700;" data-darkreader-inline-bgcolor=""></td><td title="LIME (191, 255, 0)" style="background-color: rgb(191, 255, 0); --darkreader-inline-bgcolor:#99cc00;" data-darkreader-inline-bgcolor=""></td><td title="DARK_LIME (143, 191, 0)" style="background-color: rgb(143, 191, 0); --darkreader-inline-bgcolor:#8fbf00;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_LIME (96, 128, 0)" style="background-color: rgb(96, 128, 0); --darkreader-inline-bgcolor:#608000;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_LIME (48, 64, 0)" style="background-color: rgb(48, 64, 0); --darkreader-inline-bgcolor:#304000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>CHARTREUSE</td><td title="DESATURATED_CHARTREUSE (96, 128, 64)" style="background-color: rgb(96, 128, 64); --darkreader-inline-bgcolor:#4d6633;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_CHARTREUSE (223, 255, 191)" style="background-color: rgb(223, 255, 191); --darkreader-inline-bgcolor:#2d5900;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_CHARTREUSE (210, 255, 166)" style="background-color: rgb(210, 255, 166); --darkreader-inline-bgcolor:#346800;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_CHARTREUSE (185, 255, 115)" style="background-color: rgb(185, 255, 115); --darkreader-inline-bgcolor:#448700;" data-darkreader-inline-bgcolor=""></td><td title="CHARTREUSE (127, 255, 0)" style="background-color: rgb(127, 255, 0); --darkreader-inline-bgcolor:#66cc00;" data-darkreader-inline-bgcolor=""></td><td title="DARK_CHARTREUSE (95, 191, 0)" style="background-color: rgb(95, 191, 0); --darkreader-inline-bgcolor:#5fbf00;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_CHARTREUSE (64, 128, 0)" style="background-color: rgb(64, 128, 0); --darkreader-inline-bgcolor:#408000;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_CHARTREUSE (32, 64, 0)" style="background-color: rgb(32, 64, 0); --darkreader-inline-bgcolor:#204000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>GREEN</td><td title="DESATURATED_GREEN (64, 128, 64)" style="background-color: rgb(64, 128, 64); --darkreader-inline-bgcolor:#336633;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_GREEN (191, 255, 191)" style="background-color: rgb(191, 255, 191); --darkreader-inline-bgcolor:#005900;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_GREEN (166, 255, 166)" style="background-color: rgb(166, 255, 166); --darkreader-inline-bgcolor:#006800;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_GREEN (115, 255, 115)" style="background-color: rgb(115, 255, 115); --darkreader-inline-bgcolor:#008700;" data-darkreader-inline-bgcolor=""></td><td title="GREEN (0, 255, 0)" style="background-color: rgb(0, 255, 0); --darkreader-inline-bgcolor:#00cc00;" data-darkreader-inline-bgcolor=""></td><td title="DARK_GREEN (0, 191, 0)" style="background-color: rgb(0, 191, 0); --darkreader-inline-bgcolor:#00bf00;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_GREEN (0, 128, 0)" style="background-color: rgb(0, 128, 0); --darkreader-inline-bgcolor:#008000;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_GREEN (0, 64, 0)" style="background-color: rgb(0, 64, 0); --darkreader-inline-bgcolor:#004000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>SEA</td><td title="DESATURATED_SEA (64, 128, 96)" style="background-color: rgb(64, 128, 96); --darkreader-inline-bgcolor:#33664d;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_SEA (191, 255, 223)" style="background-color: rgb(191, 255, 223); --darkreader-inline-bgcolor:#00592d;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_SEA (166, 255, 210)" style="background-color: rgb(166, 255, 210); --darkreader-inline-bgcolor:#006834;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_SEA (115, 255, 185)" style="background-color: rgb(115, 255, 185); --darkreader-inline-bgcolor:#008744;" data-darkreader-inline-bgcolor=""></td><td title="SEA (0, 255, 127)" style="background-color: rgb(0, 255, 127); --darkreader-inline-bgcolor:#00cc66;" data-darkreader-inline-bgcolor=""></td><td title="DARK_SEA (0, 191, 95)" style="background-color: rgb(0, 191, 95); --darkreader-inline-bgcolor:#00bf5f;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_SEA (0, 128, 64)" style="background-color: rgb(0, 128, 64); --darkreader-inline-bgcolor:#008040;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_SEA (0, 64, 32)" style="background-color: rgb(0, 64, 32); --darkreader-inline-bgcolor:#004020;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>TURQUOISE</td><td title="DESATURATED_TURQUOISE (64, 128, 112)" style="background-color: rgb(64, 128, 112); --darkreader-inline-bgcolor:#336659;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_TURQUOISE (191, 255, 239)" style="background-color: rgb(191, 255, 239); --darkreader-inline-bgcolor:#005943;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_TURQUOISE (166, 255, 233)" style="background-color: rgb(166, 255, 233); --darkreader-inline-bgcolor:#00684f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_TURQUOISE (115, 255, 220)" style="background-color: rgb(115, 255, 220); --darkreader-inline-bgcolor:#008765;" data-darkreader-inline-bgcolor=""></td><td title="TURQUOISE (0, 255, 191)" style="background-color: rgb(0, 255, 191); --darkreader-inline-bgcolor:#00cc99;" data-darkreader-inline-bgcolor=""></td><td title="DARK_TURQUOISE (0, 191, 143)" style="background-color: rgb(0, 191, 143); --darkreader-inline-bgcolor:#00bf8f;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_TURQUOISE (0, 128, 96)" style="background-color: rgb(0, 128, 96); --darkreader-inline-bgcolor:#008060;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_TURQUOISE (0, 64, 48)" style="background-color: rgb(0, 64, 48); --darkreader-inline-bgcolor:#004030;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>CYAN</td><td title="DESATURATED_CYAN (64, 128, 128)" style="background-color: rgb(64, 128, 128); --darkreader-inline-bgcolor:#336666;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_CYAN (191, 255, 255)" style="background-color: rgb(191, 255, 255); --darkreader-inline-bgcolor:#005959;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_CYAN (166, 255, 255)" style="background-color: rgb(166, 255, 255); --darkreader-inline-bgcolor:#006868;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_CYAN (115, 255, 255)" style="background-color: rgb(115, 255, 255); --darkreader-inline-bgcolor:#008787;" data-darkreader-inline-bgcolor=""></td><td title="CYAN (0, 255, 255)" style="background-color: rgb(0, 255, 255); --darkreader-inline-bgcolor:#00cccc;" data-darkreader-inline-bgcolor=""></td><td title="DARK_CYAN (0, 191, 191)" style="background-color: rgb(0, 191, 191); --darkreader-inline-bgcolor:#00bfbf;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_CYAN (0, 128, 128)" style="background-color: rgb(0, 128, 128); --darkreader-inline-bgcolor:#008080;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_CYAN (0, 64, 64)" style="background-color: rgb(0, 64, 64); --darkreader-inline-bgcolor:#004040;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>SKY</td><td title="DESATURATED_SKY (64, 112, 128)" style="background-color: rgb(64, 112, 128); --darkreader-inline-bgcolor:#335966;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_SKY (191, 239, 255)" style="background-color: rgb(191, 239, 255); --darkreader-inline-bgcolor:#004359;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_SKY (166, 233, 255)" style="background-color: rgb(166, 233, 255); --darkreader-inline-bgcolor:#004f68;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_SKY (115, 220, 255)" style="background-color: rgb(115, 220, 255); --darkreader-inline-bgcolor:#006587;" data-darkreader-inline-bgcolor=""></td><td title="SKY (0, 191, 255)" style="background-color: rgb(0, 191, 255); --darkreader-inline-bgcolor:#0099cc;" data-darkreader-inline-bgcolor=""></td><td title="DARK_SKY (0, 143, 191)" style="background-color: rgb(0, 143, 191); --darkreader-inline-bgcolor:#008fbf;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_SKY (0, 96, 128)" style="background-color: rgb(0, 96, 128); --darkreader-inline-bgcolor:#006080;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_SKY (0, 48, 64)" style="background-color: rgb(0, 48, 64); --darkreader-inline-bgcolor:#003040;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>AZURE</td><td title="DESATURATED_AZURE (64, 96, 128)" style="background-color: rgb(64, 96, 128); --darkreader-inline-bgcolor:#334d66;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_AZURE (191, 223, 255)" style="background-color: rgb(191, 223, 255); --darkreader-inline-bgcolor:#2a2d2f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_AZURE (166, 210, 255)" style="background-color: rgb(166, 210, 255); --darkreader-inline-bgcolor:#323537;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_AZURE (115, 185, 255)" style="background-color: rgb(115, 185, 255); --darkreader-inline-bgcolor:#004487;" data-darkreader-inline-bgcolor=""></td><td title="AZURE (0, 127, 255)" style="background-color: rgb(0, 127, 255); --darkreader-inline-bgcolor:#0066cc;" data-darkreader-inline-bgcolor=""></td><td title="DARK_AZURE (0, 95, 191)" style="background-color: rgb(0, 95, 191); --darkreader-inline-bgcolor:#005fbf;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_AZURE (0, 64, 128)" style="background-color: rgb(0, 64, 128); --darkreader-inline-bgcolor:#004080;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_AZURE (0, 32, 64)" style="background-color: rgb(0, 32, 64); --darkreader-inline-bgcolor:#002040;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>BLUE</td><td title="DESATURATED_BLUE (64, 64, 128)" style="background-color: rgb(64, 64, 128); --darkreader-inline-bgcolor:#333366;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_BLUE (191, 191, 255)" style="background-color: rgb(191, 191, 255); --darkreader-inline-bgcolor:#2a2d2f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_BLUE (166, 166, 255)" style="background-color: rgb(166, 166, 255); --darkreader-inline-bgcolor:#323537;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_BLUE (115, 115, 255)" style="background-color: rgb(115, 115, 255); --darkreader-inline-bgcolor:#000087;" data-darkreader-inline-bgcolor=""></td><td title="BLUE (0, 0, 255)" style="background-color: rgb(0, 0, 255); --darkreader-inline-bgcolor:#0000cc;" data-darkreader-inline-bgcolor=""></td><td title="DARK_BLUE (0, 0, 191)" style="background-color: rgb(0, 0, 191); --darkreader-inline-bgcolor:#0000bf;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_BLUE (0, 0, 128)" style="background-color: rgb(0, 0, 128); --darkreader-inline-bgcolor:#000080;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_BLUE (0, 0, 64)" style="background-color: rgb(0, 0, 64); --darkreader-inline-bgcolor:#000040;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>HAN</td><td title="DESATURATED_HAN (80, 64, 128)" style="background-color: rgb(80, 64, 128); --darkreader-inline-bgcolor:#403366;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_HAN (207, 191, 255)" style="background-color: rgb(207, 191, 255); --darkreader-inline-bgcolor:#2a2d2f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_HAN (188, 166, 255)" style="background-color: rgb(188, 166, 255); --darkreader-inline-bgcolor:#323537;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_HAN (149, 115, 255)" style="background-color: rgb(149, 115, 255); --darkreader-inline-bgcolor:#210087;" data-darkreader-inline-bgcolor=""></td><td title="HAN (63, 0, 255)" style="background-color: rgb(63, 0, 255); --darkreader-inline-bgcolor:#3200cc;" data-darkreader-inline-bgcolor=""></td><td title="DARK_HAN (47, 0, 191)" style="background-color: rgb(47, 0, 191); --darkreader-inline-bgcolor:#2f00bf;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_HAN (32, 0, 128)" style="background-color: rgb(32, 0, 128); --darkreader-inline-bgcolor:#200080;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_HAN (16, 0, 64)" style="background-color: rgb(16, 0, 64); --darkreader-inline-bgcolor:#100040;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>VIOLET</td><td title="DESATURATED_VIOLET (96, 64, 128)" style="background-color: rgb(96, 64, 128); --darkreader-inline-bgcolor:#4d3366;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_VIOLET (223, 191, 255)" style="background-color: rgb(223, 191, 255); --darkreader-inline-bgcolor:#2a2d2f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_VIOLET (210, 166, 255)" style="background-color: rgb(210, 166, 255); --darkreader-inline-bgcolor:#323537;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_VIOLET (185, 115, 255)" style="background-color: rgb(185, 115, 255); --darkreader-inline-bgcolor:#440087;" data-darkreader-inline-bgcolor=""></td><td title="VIOLET (127, 0, 255)" style="background-color: rgb(127, 0, 255); --darkreader-inline-bgcolor:#6600cc;" data-darkreader-inline-bgcolor=""></td><td title="DARK_VIOLET (95, 0, 191)" style="background-color: rgb(95, 0, 191); --darkreader-inline-bgcolor:#5f00bf;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_VIOLET (64, 0, 128)" style="background-color: rgb(64, 0, 128); --darkreader-inline-bgcolor:#400080;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_VIOLET (32, 0, 64)" style="background-color: rgb(32, 0, 64); --darkreader-inline-bgcolor:#200040;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>PURPLE</td><td title="DESATURATED_PURPLE (111, 64, 128)" style="background-color: rgb(111, 64, 128); --darkreader-inline-bgcolor:#583366;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_PURPLE (239, 191, 255)" style="background-color: rgb(239, 191, 255); --darkreader-inline-bgcolor:#430059;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_PURPLE (233, 166, 255)" style="background-color: rgb(233, 166, 255); --darkreader-inline-bgcolor:#4f0068;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_PURPLE (220, 115, 255)" style="background-color: rgb(220, 115, 255); --darkreader-inline-bgcolor:#650087;" data-darkreader-inline-bgcolor=""></td><td title="PURPLE (191, 0, 255)" style="background-color: rgb(191, 0, 255); --darkreader-inline-bgcolor:#9900cc;" data-darkreader-inline-bgcolor=""></td><td title="DARK_PURPLE (143, 0, 191)" style="background-color: rgb(143, 0, 191); --darkreader-inline-bgcolor:#8f00bf;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_PURPLE (95, 0, 128)" style="background-color: rgb(95, 0, 128); --darkreader-inline-bgcolor:#5f0080;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_PURPLE (48, 0, 64)" style="background-color: rgb(48, 0, 64); --darkreader-inline-bgcolor:#300040;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>FUCHSIA</td><td title="DESATURATED_FUCHSIA (128, 64, 128)" style="background-color: rgb(128, 64, 128); --darkreader-inline-bgcolor:#663366;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_FUCHSIA (255, 191, 255)" style="background-color: rgb(255, 191, 255); --darkreader-inline-bgcolor:#590059;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_FUCHSIA (255, 166, 255)" style="background-color: rgb(255, 166, 255); --darkreader-inline-bgcolor:#680068;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_FUCHSIA (255, 115, 255)" style="background-color: rgb(255, 115, 255); --darkreader-inline-bgcolor:#870087;" data-darkreader-inline-bgcolor=""></td><td title="FUCHSIA (255, 0, 255)" style="background-color: rgb(255, 0, 255); --darkreader-inline-bgcolor:#cc00cc;" data-darkreader-inline-bgcolor=""></td><td title="DARK_FUCHSIA (191, 0, 191)" style="background-color: rgb(191, 0, 191); --darkreader-inline-bgcolor:#bf00bf;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_FUCHSIA (128, 0, 128)" style="background-color: rgb(128, 0, 128); --darkreader-inline-bgcolor:#800080;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_FUCHSIA (64, 0, 64)" style="background-color: rgb(64, 0, 64); --darkreader-inline-bgcolor:#400040;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>MAGENTA</td><td title="DESATURATED_MAGENTA (128, 64, 111)" style="background-color: rgb(128, 64, 111); --darkreader-inline-bgcolor:#663358;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_MAGENTA (255, 191, 239)" style="background-color: rgb(255, 191, 239); --darkreader-inline-bgcolor:#590043;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_MAGENTA (255, 166, 233)" style="background-color: rgb(255, 166, 233); --darkreader-inline-bgcolor:#68004f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_MAGENTA (255, 115, 220)" style="background-color: rgb(255, 115, 220); --darkreader-inline-bgcolor:#870065;" data-darkreader-inline-bgcolor=""></td><td title="MAGENTA (255, 0, 191)" style="background-color: rgb(255, 0, 191); --darkreader-inline-bgcolor:#cc0099;" data-darkreader-inline-bgcolor=""></td><td title="DARK_MAGENTA (191, 0, 143)" style="background-color: rgb(191, 0, 143); --darkreader-inline-bgcolor:#bf008f;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_MAGENTA (128, 0, 95)" style="background-color: rgb(128, 0, 95); --darkreader-inline-bgcolor:#80005f;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_MAGENTA (64, 0, 48)" style="background-color: rgb(64, 0, 48); --darkreader-inline-bgcolor:#400030;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>PINK</td><td title="DESATURATED_PINK (128, 64, 96)" style="background-color: rgb(128, 64, 96); --darkreader-inline-bgcolor:#66334d;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_PINK (255, 191, 223)" style="background-color: rgb(255, 191, 223); --darkreader-inline-bgcolor:#59002d;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_PINK (255, 166, 210)" style="background-color: rgb(255, 166, 210); --darkreader-inline-bgcolor:#680034;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_PINK (255, 115, 185)" style="background-color: rgb(255, 115, 185); --darkreader-inline-bgcolor:#870044;" data-darkreader-inline-bgcolor=""></td><td title="PINK (255, 0, 127)" style="background-color: rgb(255, 0, 127); --darkreader-inline-bgcolor:#cc0066;" data-darkreader-inline-bgcolor=""></td><td title="DARK_PINK (191, 0, 95)" style="background-color: rgb(191, 0, 95); --darkreader-inline-bgcolor:#bf005f;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_PINK (128, 0, 64)" style="background-color: rgb(128, 0, 64); --darkreader-inline-bgcolor:#800040;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_PINK (64, 0, 32)" style="background-color: rgb(64, 0, 32); --darkreader-inline-bgcolor:#400020;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>CRIMSON</td><td title="DESATURATED_CRIMSON (128, 64, 79)" style="background-color: rgb(128, 64, 79); --darkreader-inline-bgcolor:#66333f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTEST_CRIMSON (255, 191, 207)" style="background-color: rgb(255, 191, 207); --darkreader-inline-bgcolor:#590016;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_CRIMSON (255, 166, 188)" style="background-color: rgb(255, 166, 188); --darkreader-inline-bgcolor:#68001a;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_CRIMSON (255, 115, 149)" style="background-color: rgb(255, 115, 149); --darkreader-inline-bgcolor:#870021;" data-darkreader-inline-bgcolor=""></td><td title="CRIMSON (255, 0, 63)" style="background-color: rgb(255, 0, 63); --darkreader-inline-bgcolor:#cc0032;" data-darkreader-inline-bgcolor=""></td><td title="DARK_CRIMSON (191, 0, 47)" style="background-color: rgb(191, 0, 47); --darkreader-inline-bgcolor:#bf002f;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_CRIMSON (128, 0, 31)" style="background-color: rgb(128, 0, 31); --darkreader-inline-bgcolor:#80001f;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_CRIMSON (64, 0, 16)" style="background-color: rgb(64, 0, 16); --darkreader-inline-bgcolor:#400010;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td></td><th colspan="8">METALLIC COLORS</th></tr>
//! <tr><td>BRASS</td><td title="BRASS (191, 151, 96)" style="background-color: rgb(191, 151, 96); --darkreader-inline-bgcolor:#684e2a;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>COPPER</td><td title="COPPER (196, 136, 124)" style="background-color: rgb(196, 136, 124); --darkreader-inline-bgcolor:#593028;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>GOLD</td><td title="GOLD (229, 191, 0)" style="background-color: rgb(229, 191, 0); --darkreader-inline-bgcolor:#ccaa00;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>SILVER</td><td title="SILVER (203, 203, 203)" style="background-color: rgb(203, 203, 203); --darkreader-inline-bgcolor:#27292b;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td></td><th colspan="8">MISCELLANEOUS COLORS</th></tr>
//! <tr><td>CELADON</td><td title="CELADON (172, 255, 175)" style="background-color: rgb(172, 255, 175); --darkreader-inline-bgcolor:#006504;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>PEACH</td><td title="PEACH (255, 159, 127)" style="background-color: rgb(255, 159, 127); --darkreader-inline-bgcolor:#802000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td></td><th colspan="8">GRAYSCALE &amp; SEPIA</th></tr>
//! <tr><td colspan="2">&nbsp;</td><td>LIGHTEST</td><td>LIGHTER</td><td>LIGHT</td><td>NORMAL</td><td>DARK</td><td>DARKER</td><td>DARKEST</td></tr>
//! <tr><td>GRAY</td><td>&nbsp;</td><td title="LIGHTEST_GRAY (223, 223, 223)" style="background-color: rgb(223, 223, 223); --darkreader-inline-bgcolor:#212325;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_GRAY (191, 191, 191)" style="background-color: rgb(191, 191, 191); --darkreader-inline-bgcolor:#2a2d2f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_GRAY (159, 159, 159)" style="background-color: rgb(159, 159, 159); --darkreader-inline-bgcolor:#343739;" data-darkreader-inline-bgcolor=""></td><td title="GRAY (127, 127, 127)" style="background-color: rgb(127, 127, 127); --darkreader-inline-bgcolor:#3d4043;" data-darkreader-inline-bgcolor=""></td><td title="DARK_GRAY (95, 95, 95)" style="background-color: rgb(95, 95, 95); --darkreader-inline-bgcolor:#3d4043;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_GRAY (63, 63, 63)" style="background-color: rgb(63, 63, 63); --darkreader-inline-bgcolor:#3c4042;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_GRAY (31, 31, 31)" style="background-color: rgb(31, 31, 31); --darkreader-inline-bgcolor:#1d1f21;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>SEPIA</td><td>&nbsp;</td><td title="LIGHTEST_SEPIA (222, 211, 195)" style="background-color: rgb(222, 211, 195); --darkreader-inline-bgcolor:#382e1f;" data-darkreader-inline-bgcolor=""></td><td title="LIGHTER_SEPIA (191, 171, 143)" style="background-color: rgb(191, 171, 143); --darkreader-inline-bgcolor:#4b3e2b;" data-darkreader-inline-bgcolor=""></td><td title="LIGHT_SEPIA (158, 134, 100)" style="background-color: rgb(158, 134, 100); --darkreader-inline-bgcolor:#594b37;" data-darkreader-inline-bgcolor=""></td><td title="SEPIA (127, 101, 63)" style="background-color: rgb(127, 101, 63); --darkreader-inline-bgcolor:#665133;" data-darkreader-inline-bgcolor=""></td><td title="DARK_SEPIA (94, 75, 47)" style="background-color: rgb(94, 75, 47); --darkreader-inline-bgcolor:#5e4b2f;" data-darkreader-inline-bgcolor=""></td><td title="DARKER_SEPIA (63, 50, 31)" style="background-color: rgb(63, 50, 31); --darkreader-inline-bgcolor:#3f321f;" data-darkreader-inline-bgcolor=""></td><td title="DARKEST_SEPIA (31, 24, 15)" style="background-color: rgb(31, 24, 15); --darkreader-inline-bgcolor:#1f180f;" data-darkreader-inline-bgcolor=""></td></tr><tr><td></td><th colspan="8">BLACK AND WHITE</th></tr>
//! <tr><td>BLACK</td><td title="BLACK (0, 0, 0)" style="background-color: rgb(0, 0, 0); --darkreader-inline-bgcolor:#000000;" data-darkreader-inline-bgcolor=""></td></tr>
//! <tr><td>WHITE</td><td title="WHITE (255, 255, 255)" style="background-color: rgb(255, 255, 255); --darkreader-inline-bgcolor:#181a1b;" data-darkreader-inline-bgcolor=""></td></tr>
//! </tbody></table>

use crate::util::FloorRem;
use std::ops::{Add, Mul, Sub};

pub use Color as Colour;

/// A struct representing a 24-bit RGB color with alpha
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct Color {
    /// The red component of the color
    pub r: u8,
    /// The green component of the color
    pub g: u8,
    /// The blue component of the color
    pub b: u8,
    /// The opacity of the color
    pub a: u8,
}

impl Color {
    /// Returns a new Color from RGB values.
    ///
    /// # Parameters
    /// * `r` - The color's amount of red.
    /// * `g` - The color's amount of green.
    /// * `b` - The color's amount of blue.
    ///
    /// # Example
    /// ```
    /// # use doryen_extra::color::Color;
    /// let white = Color::new(255, 255, 255);
    /// ```
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    /// Returns a new Color from RGB values with an opacity.
    ///
    /// # Parameters
    /// * `r` - The color's amount of red.
    /// * `g` - The color's amount of green.
    /// * `b` - The color's amount of blue.
    /// * `a` - The color's opacity.
    ///
    /// # Example
    /// ```
    /// # use doryen_extra::color::Color;
    /// let translucent_white = Color::new_with_alpha(255, 255, 255, 127);
    /// ```
    pub const fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Returns a new Color from HSV values.
    ///
    /// The saturation and value parameters are automatically clamped to 0 and 1.
    ///
    /// Use `set_hsv()` to fill an existing struct with HSV values.
    ///
    /// # Parameters
    /// * `hue` - The color's hue in degrees.
    /// * `saturation` - The color's saturation, from 0 to 1.
    /// * `value` - The color's value, from 0 to 1.
    ///
    /// # Example
    /// ```
    /// # use doryen_extra::color::Color;
    /// let light_blue = Color::new_hsv(240.0, 0.75, 1.0);
    /// ```
    pub fn new_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        let mut color = Color::new(0, 0, 0);
        color.set_hsv(hue, saturation, value);

        color
    }

    /// Returns a new Color from HSV values with the given opacity.
    ///
    /// The saturation, value and opacity parameters are automatically clamped to 0 and 1.
    ///
    /// Use `set_hsv()` to fill an existing struct with HSV values.
    ///
    /// # Parameters
    /// * `hue` - The color's hue in degrees.
    /// * `saturation` - The color's saturation, from 0 to 1.
    /// * `value` - The color's value, from 0 to 1.
    /// * `opacity` - The color's opacity, from 0 to 1.
    ///
    /// # Example
    /// ```
    /// # use doryen_extra::color::Color;
    /// let translucent_light_blue = Color::new_hsv_with_opacity(240.0, 0.75, 1.0, 0.5);
    /// ```
    pub fn new_hsv_with_opacity(hue: f32, saturation: f32, value: f32, opacity: f32) -> Self {
        let a = (opacity.max(0.0).min(1.0) * 255.0).round() as u8;
        let mut color = Color::new_with_alpha(0, 0, 0, a);
        color.set_hsv(hue, saturation, value);

        color
    }

    /// Sets a colors values from HSV values.
    ///
    /// # Parameters
    /// * `hue` - The color's hue in degrees.
    /// * `saturation` - The color's saturation, from 0 to 1.
    /// * `value` - The color's value, from 0 to 1.
    #[allow(clippy::many_single_char_names)]
    pub fn set_hsv(&mut self, hue: f32, saturation: f32, value: f32) {
        let saturation = saturation.max(0.0).min(1.0);
        let value = value.max(0.0).min(1.0);

        if saturation == 0.0 {
            /* achromatic (gray) */
            let value = (value * 255.0).round() as u8;
            self.r = value;
            self.g = value;
            self.b = value;
            return;
        }

        let mut hue = hue.floor_modulo(360.0);
        hue /= 60.0; /* sector 0 to 5 */
        let hue_section = hue.floor() as i32;
        let hue_fraction = hue - hue_section as f32;

        let p = ((value * (1.0 - saturation)) * 255.0).round() as u8;
        let q = ((value * (1.0 - saturation * hue_fraction)) * 255.0).round() as u8;
        let t = ((value * (1.0 - saturation * (1.0 - hue_fraction))) * 255.0).round() as u8;
        let v = (value * 255.0).round() as u8;

        let (r, g, b) = match hue_section {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };
        self.r = r;
        self.g = g;
        self.b = b;
    }

    /// Get a tuple of HSV values from a color.
    pub fn get_hsv(self) -> (f32, f32, f32) {
        let hue = self.get_hue();
        let saturation = self.get_saturation();
        let value = self.get_value();

        (hue, saturation, value)
    }

    /// Change a color's hue.
    ///
    /// # Parameters
    /// * `hue` - The color's hue in degrees.
    pub fn set_hue(&mut self, hue: f32) {
        let saturation = self.get_saturation();
        let value = self.get_value();

        self.set_hsv(hue, saturation, value);
    }

    /// Return a color's hue in degrees.
    pub fn get_hue(self) -> f32 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = f32::from(max) - f32::from(min);
        if delta == 0.0 {
            return 0.0;
        }

        let mut hue = if self.r == max {
            (f32::from(self.g) - f32::from(self.b)) / delta
        } else if self.g == max {
            2.0 + (f32::from(self.b) - f32::from(self.r)) / delta
        } else {
            4.0 + (f32::from(self.r) - f32::from(self.g)) / delta
        };
        hue *= 60.0;
        hue.floor_modulo(360.0)
    }

    /// Returns a color's saturation in the range \[0, 1\].
    pub fn get_saturation(self) -> f32 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let delta = f32::from(max) - f32::from(min);
        if max == 0 {
            0.0
        } else {
            delta / f32::from(max)
        }
    }

    /// Change a color's saturation.
    ///
    /// # Parameters
    /// * `saturation` - The color's saturation, from 0 to 1.
    pub fn set_saturation(&mut self, saturation: f32) {
        let hue = self.get_hue();
        let value = self.get_value();

        self.set_hsv(hue, saturation, value);
    }

    /// Returns a color's value in the range \[0, 1\].
    pub fn get_value(self) -> f32 {
        f32::from(self.r.max(self.g).max(self.b)) / 255.0
    }

    /// Change a color's value.
    ///
    /// # Parameters
    /// * `value` - The color's value, from 0 to 1.
    pub fn set_value(&mut self, value: f32) {
        let hue = self.get_hue();
        let saturation = self.get_saturation();

        self.set_hsv(hue, saturation, value);
    }

    /// Shift a color's hue by an amount.
    ///
    /// # Parameters
    /// * `hue_shift` - The distance to shift the hue, in degrees.
    pub fn shift_hue(&mut self, hue_shift: f32) {
        if hue_shift == 0.0 {
            return;
        }
        self.set_hsv(
            self.get_hue() + hue_shift,
            self.get_saturation(),
            self.get_value(),
        );
    }

    /// Scale a color's saturation and value.
    ///
    /// # Parameters
    /// * `saturation_coefficient` - Multiplier for this color's saturation.
    /// * `value_coefficient` - Multiplier for this color's value.
    pub fn scale_hsv(&mut self, saturation_coefficient: f32, value_coefficient: f32) {
        if (saturation_coefficient - 1.0).abs() < 0.001 && (value_coefficient - 1.0).abs() < 0.001 {
            return;
        }
        self.set_hsv(
            self.get_hue(),
            self.get_saturation() * saturation_coefficient,
            self.get_value() * value_coefficient,
        );
    }

    /// Generates an interpolated gradient of colors using RGB interpolation.
    ///
    /// Using RGB interpolation between colors is almost always the wrong choice and tends to
    /// produce really ugly results. You almost certainly don't want to use this; use
    /// `generate_gradient_hsv()` instead.
    ///
    /// # Parameters
    /// * `key_colors` -  The colors to make gradients between.
    /// * `gradient_spans` -  How many interpolated colors to generate between each
    /// pair of key colors.
    ///
    /// # Panics
    /// * If `gradient_spans`' length isn't one less than `key_colors`' length.
    ///
    /// # Example
    /// ```
    /// # use doryen_extra::color::Color;
    /// // Generates every grayscale color between black and white
    /// let grayscale = Color::generate_gradient_rgb(&[Color::BLACK,Color::WHITE], &[254]);
    /// ```
    pub fn generate_gradient_rgb(key_colors: &[Color], gradient_spans: &[usize]) -> Vec<Color> {
        if key_colors.is_empty() {
            return vec![];
        }

        assert_eq!(
            key_colors.len() - 1,
            gradient_spans.len(),
            "gradient_spans should have one fewer values in it than key_colors"
        );

        let mut result =
            Vec::with_capacity(key_colors.len() + gradient_spans.iter().sum::<usize>());
        for (span, colors) in key_colors.windows(2).enumerate() {
            let start_color = colors[0];
            let end_color = colors[1];
            for s in 0..=gradient_spans[span] {
                let coefficient = s as f32 / (gradient_spans[span] + 1) as f32;
                result.push(start_color.lerp_rgb(end_color, coefficient));
            }
        }
        result.push(*key_colors.last().unwrap());

        result
    }

    /// Generates an interpolated gradient of colors using HSV interpolation.
    ///
    /// # Parameters
    /// * `key_colors` -  The colors to make gradients between.
    /// * `gradient_spans` -  How many interpolated colors to generate between each
    /// pair of key colors.
    ///
    /// # Panics
    /// * If `gradient_spans`' length isn't one less than `key_colors`' length.
    ///
    /// # Example
    /// ```
    /// # use doryen_extra::color::Color;
    /// // Generates every grayscale color between black and white
    /// let grayscale = Color::generate_gradient_hsv(&[Color::BLACK,Color::WHITE], &[254]);
    /// ```
    pub fn generate_gradient_hsv(key_colors: &[Color], gradient_spans: &[usize]) -> Vec<Color> {
        if key_colors.is_empty() {
            return vec![];
        }

        assert_eq!(
            key_colors.len() - 1,
            gradient_spans.len(),
            "gradient_spans should have one fewer values in it than key_colors"
        );

        let mut result =
            Vec::with_capacity(key_colors.len() + gradient_spans.iter().sum::<usize>());
        for (span, colors) in key_colors.windows(2).enumerate() {
            let start_color = colors[0];
            let end_color = colors[1];
            for s in 0..=gradient_spans[span] {
                let coefficient = s as f32 / (gradient_spans[span] + 1) as f32;
                result.push(start_color.lerp_hsv(end_color, coefficient));
            }
        }
        result.push(*key_colors.last().unwrap());

        result
    }

    /// Interpolate two colors together using their RGB representation and return the result.
    ///
    /// You almost certainly don't want to use this; use `lerp_hsv()` instead.
    ///
    /// # Parameters
    /// * `other` - The second color.
    /// * `coefficient` - The coefficient. 0 for entirely the first color, 1 for entirely the second.
    ///
    /// # Panics
    ///
    /// If `coefficient` is outside the range \[0, 1\].
    pub fn lerp_rgb(self, other: Color, coefficient: f32) -> Color {
        assert!(
            coefficient >= 0.0 && coefficient <= 1.0,
            "coefficient is outside the acceptable range [0, 1]"
        );

        Self::new_with_alpha(
            (f32::from(self.r) + (f32::from(other.r) - f32::from(self.r)) * coefficient) as u8,
            (f32::from(self.g) + (f32::from(other.g) - f32::from(self.g)) * coefficient) as u8,
            (f32::from(self.b) + (f32::from(other.b) - f32::from(self.b)) * coefficient) as u8,
            (f32::from(self.a) + (f32::from(other.a) - f32::from(self.a)) * coefficient) as u8,
        )
    }

    /// Interpolate two colors together using their HSV representation and return the result.
    ///
    /// # Parameters
    /// * `other` - The second color.
    /// * `coefficient` - The coefficient. 0 for entirely the first color, 1 for entirely the second.
    ///
    /// # Panics
    ///
    /// If `coefficient` is outside the range \[0, 1\].
    pub fn lerp_hsv(self, other: Color, coefficient: f32) -> Color {
        assert!(
            coefficient >= 0.0 && coefficient <= 1.0,
            "coefficient is outside the acceptable range [0, 1]"
        );
        let (self_hue, self_saturation, self_value) = self.get_hsv();
        let (other_hue, other_saturation, other_value) = other.get_hsv();

        let hue_diff = other_hue - self_hue;
        let hue_delta = hue_diff
            + if hue_diff.abs() > 180.0 {
                if hue_diff < 0.0 {
                    360.0
                } else {
                    -360.0
                }
            } else {
                0.0
            };

        let hue_interpolated = self_hue + coefficient * hue_delta;

        let opacity_interpolated =
            (f32::from(self.a) + (f32::from(other.a) - f32::from(self.a)) * coefficient) / 255.0;

        Self::new_hsv_with_opacity(
            hue_interpolated,
            self_saturation + (other_saturation - self_saturation) * coefficient,
            self_value + (other_value - self_value) * coefficient,
            opacity_interpolated,
        )
    }
}

// Enums-to-color
impl Color {
    /// Takes a `Name` and `Level` value and returns the corresponding color constant.
    #[allow(clippy::too_many_lines)]
    pub fn by_name_and_level(name: Name, level: Level) -> Self {
        match name {
            Name::Red => match level {
                Level::Desaturated => Self::DESATURATED_RED,
                Level::Lightest => Self::LIGHTEST_RED,
                Level::Lighter => Self::LIGHTER_RED,
                Level::Light => Self::LIGHT_RED,
                Level::Normal => Self::RED,
                Level::Dark => Self::DARK_RED,
                Level::Darker => Self::DARKER_RED,
                Level::Darkest => Self::DARKEST_RED,
            },
            Name::Flame => match level {
                Level::Desaturated => Self::DESATURATED_FLAME,
                Level::Lightest => Self::LIGHTEST_FLAME,
                Level::Lighter => Self::LIGHTER_FLAME,
                Level::Light => Self::LIGHT_FLAME,
                Level::Normal => Self::FLAME,
                Level::Dark => Self::DARK_FLAME,
                Level::Darker => Self::DARKER_FLAME,
                Level::Darkest => Self::DARKEST_FLAME,
            },
            Name::Orange => match level {
                Level::Desaturated => Self::DESATURATED_ORANGE,
                Level::Lightest => Self::LIGHTEST_ORANGE,
                Level::Lighter => Self::LIGHTER_ORANGE,
                Level::Light => Self::LIGHT_ORANGE,
                Level::Normal => Self::ORANGE,
                Level::Dark => Self::DARK_ORANGE,
                Level::Darker => Self::DARKER_ORANGE,
                Level::Darkest => Self::DARKEST_ORANGE,
            },
            Name::Amber => match level {
                Level::Desaturated => Self::DESATURATED_AMBER,
                Level::Lightest => Self::LIGHTEST_AMBER,
                Level::Lighter => Self::LIGHTER_AMBER,
                Level::Light => Self::LIGHT_AMBER,
                Level::Normal => Self::AMBER,
                Level::Dark => Self::DARK_AMBER,
                Level::Darker => Self::DARKER_AMBER,
                Level::Darkest => Self::DARKEST_AMBER,
            },
            Name::Yellow => match level {
                Level::Desaturated => Self::DESATURATED_YELLOW,
                Level::Lightest => Self::LIGHTEST_YELLOW,
                Level::Lighter => Self::LIGHTER_YELLOW,
                Level::Light => Self::LIGHT_YELLOW,
                Level::Normal => Self::YELLOW,
                Level::Dark => Self::DARK_YELLOW,
                Level::Darker => Self::DARKER_YELLOW,
                Level::Darkest => Self::DARKEST_YELLOW,
            },
            Name::Lime => match level {
                Level::Desaturated => Self::DESATURATED_LIME,
                Level::Lightest => Self::LIGHTEST_LIME,
                Level::Lighter => Self::LIGHTER_LIME,
                Level::Light => Self::LIGHT_LIME,
                Level::Normal => Self::LIME,
                Level::Dark => Self::DARK_LIME,
                Level::Darker => Self::DARKER_LIME,
                Level::Darkest => Self::DARKEST_LIME,
            },
            Name::Chartreuse => match level {
                Level::Desaturated => Self::DESATURATED_CHARTREUSE,
                Level::Lightest => Self::LIGHTEST_CHARTREUSE,
                Level::Lighter => Self::LIGHTER_CHARTREUSE,
                Level::Light => Self::LIGHT_CHARTREUSE,
                Level::Normal => Self::CHARTREUSE,
                Level::Dark => Self::DARK_CHARTREUSE,
                Level::Darker => Self::DARKER_CHARTREUSE,
                Level::Darkest => Self::DARKEST_CHARTREUSE,
            },
            Name::Green => match level {
                Level::Desaturated => Self::DESATURATED_GREEN,
                Level::Lightest => Self::LIGHTEST_GREEN,
                Level::Lighter => Self::LIGHTER_GREEN,
                Level::Light => Self::LIGHT_GREEN,
                Level::Normal => Self::GREEN,
                Level::Dark => Self::DARK_GREEN,
                Level::Darker => Self::DARKER_GREEN,
                Level::Darkest => Self::DARKEST_GREEN,
            },
            Name::Sea => match level {
                Level::Desaturated => Self::DESATURATED_SEA,
                Level::Lightest => Self::LIGHTEST_SEA,
                Level::Lighter => Self::LIGHTER_SEA,
                Level::Light => Self::LIGHT_SEA,
                Level::Normal => Self::SEA,
                Level::Dark => Self::DARK_SEA,
                Level::Darker => Self::DARKER_SEA,
                Level::Darkest => Self::DARKEST_SEA,
            },
            Name::Turquoise => match level {
                Level::Desaturated => Self::DESATURATED_TURQUOISE,
                Level::Lightest => Self::LIGHTEST_TURQUOISE,
                Level::Lighter => Self::LIGHTER_TURQUOISE,
                Level::Light => Self::LIGHT_TURQUOISE,
                Level::Normal => Self::TURQUOISE,
                Level::Dark => Self::DARK_TURQUOISE,
                Level::Darker => Self::DARKER_TURQUOISE,
                Level::Darkest => Self::DARKEST_TURQUOISE,
            },
            Name::Cyan => match level {
                Level::Desaturated => Self::DESATURATED_CYAN,
                Level::Lightest => Self::LIGHTEST_CYAN,
                Level::Lighter => Self::LIGHTER_CYAN,
                Level::Light => Self::LIGHT_CYAN,
                Level::Normal => Self::CYAN,
                Level::Dark => Self::DARK_CYAN,
                Level::Darker => Self::DARKER_CYAN,
                Level::Darkest => Self::DARKEST_CYAN,
            },
            Name::Sky => match level {
                Level::Desaturated => Self::DESATURATED_SKY,
                Level::Lightest => Self::LIGHTEST_SKY,
                Level::Lighter => Self::LIGHTER_SKY,
                Level::Light => Self::LIGHT_SKY,
                Level::Normal => Self::SKY,
                Level::Dark => Self::DARK_SKY,
                Level::Darker => Self::DARKER_SKY,
                Level::Darkest => Self::DARKEST_SKY,
            },
            Name::Azure => match level {
                Level::Desaturated => Self::DESATURATED_AZURE,
                Level::Lightest => Self::LIGHTEST_AZURE,
                Level::Lighter => Self::LIGHTER_AZURE,
                Level::Light => Self::LIGHT_AZURE,
                Level::Normal => Self::AZURE,
                Level::Dark => Self::DARK_AZURE,
                Level::Darker => Self::DARKER_AZURE,
                Level::Darkest => Self::DARKEST_AZURE,
            },
            Name::Blue => match level {
                Level::Desaturated => Self::DESATURATED_BLUE,
                Level::Lightest => Self::LIGHTEST_BLUE,
                Level::Lighter => Self::LIGHTER_BLUE,
                Level::Light => Self::LIGHT_BLUE,
                Level::Normal => Self::BLUE,
                Level::Dark => Self::DARK_BLUE,
                Level::Darker => Self::DARKER_BLUE,
                Level::Darkest => Self::DARKEST_BLUE,
            },
            Name::Han => match level {
                Level::Desaturated => Self::DESATURATED_HAN,
                Level::Lightest => Self::LIGHTEST_HAN,
                Level::Lighter => Self::LIGHTER_HAN,
                Level::Light => Self::LIGHT_HAN,
                Level::Normal => Self::HAN,
                Level::Dark => Self::DARK_HAN,
                Level::Darker => Self::DARKER_HAN,
                Level::Darkest => Self::DARKEST_HAN,
            },
            Name::Violet => match level {
                Level::Desaturated => Self::DESATURATED_VIOLET,
                Level::Lightest => Self::LIGHTEST_VIOLET,
                Level::Lighter => Self::LIGHTER_VIOLET,
                Level::Light => Self::LIGHT_VIOLET,
                Level::Normal => Self::VIOLET,
                Level::Dark => Self::DARK_VIOLET,
                Level::Darker => Self::DARKER_VIOLET,
                Level::Darkest => Self::DARKEST_VIOLET,
            },
            Name::Purple => match level {
                Level::Desaturated => Self::DESATURATED_PURPLE,
                Level::Lightest => Self::LIGHTEST_PURPLE,
                Level::Lighter => Self::LIGHTER_PURPLE,
                Level::Light => Self::LIGHT_PURPLE,
                Level::Normal => Self::PURPLE,
                Level::Dark => Self::DARK_PURPLE,
                Level::Darker => Self::DARKER_PURPLE,
                Level::Darkest => Self::DARKEST_PURPLE,
            },
            Name::Fuchsia => match level {
                Level::Desaturated => Self::DESATURATED_FUCHSIA,
                Level::Lightest => Self::LIGHTEST_FUCHSIA,
                Level::Lighter => Self::LIGHTER_FUCHSIA,
                Level::Light => Self::LIGHT_FUCHSIA,
                Level::Normal => Self::FUCHSIA,
                Level::Dark => Self::DARK_FUCHSIA,
                Level::Darker => Self::DARKER_FUCHSIA,
                Level::Darkest => Self::DARKEST_FUCHSIA,
            },
            Name::Magenta => match level {
                Level::Desaturated => Self::DESATURATED_MAGENTA,
                Level::Lightest => Self::LIGHTEST_MAGENTA,
                Level::Lighter => Self::LIGHTER_MAGENTA,
                Level::Light => Self::LIGHT_MAGENTA,
                Level::Normal => Self::MAGENTA,
                Level::Dark => Self::DARK_MAGENTA,
                Level::Darker => Self::DARKER_MAGENTA,
                Level::Darkest => Self::DARKEST_MAGENTA,
            },
            Name::Pink => match level {
                Level::Desaturated => Self::DESATURATED_PINK,
                Level::Lightest => Self::LIGHTEST_PINK,
                Level::Lighter => Self::LIGHTER_PINK,
                Level::Light => Self::LIGHT_PINK,
                Level::Normal => Self::PINK,
                Level::Dark => Self::DARK_PINK,
                Level::Darker => Self::DARKER_PINK,
                Level::Darkest => Self::DARKEST_PINK,
            },
            Name::Crimson => match level {
                Level::Desaturated => Self::DESATURATED_CRIMSON,
                Level::Lightest => Self::LIGHTEST_CRIMSON,
                Level::Lighter => Self::LIGHTER_CRIMSON,
                Level::Light => Self::LIGHT_CRIMSON,
                Level::Normal => Self::CRIMSON,
                Level::Dark => Self::DARK_CRIMSON,
                Level::Darker => Self::DARKER_CRIMSON,
                Level::Darkest => Self::DARKEST_CRIMSON,
            },
        }
    }
}

// Constants
#[allow(missing_docs)]
impl Color {
    /* color values */
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const DARKEST_GRAY: Color = Color::new(31, 31, 31);
    pub const DARKER_GRAY: Color = Color::new(63, 63, 63);
    pub const DARK_GRAY: Color = Color::new(95, 95, 95);
    pub const GRAY: Color = Color::new(127, 127, 127);
    pub const LIGHT_GRAY: Color = Color::new(159, 159, 159);
    pub const LIGHTER_GRAY: Color = Color::new(191, 191, 191);
    pub const LIGHTEST_GRAY: Color = Color::new(223, 223, 223);
    pub const DARKEST_GREY: Color = Color::DARKEST_GRAY;
    pub const DARKER_GREY: Color = Color::DARKER_GRAY;
    pub const DARK_GREY: Color = Color::DARK_GRAY;
    pub const GREY: Color = Color::GRAY;
    pub const LIGHT_GREY: Color = Color::LIGHT_GRAY;
    pub const LIGHTER_GREY: Color = Color::LIGHTER_GRAY;
    pub const LIGHTEST_GREY: Color = Color::LIGHTEST_GRAY;
    pub const WHITE: Color = Color::new(255, 255, 255);

    pub const DARKEST_SEPIA: Color = Color::new(31, 24, 15);
    pub const DARKER_SEPIA: Color = Color::new(63, 50, 31);
    pub const DARK_SEPIA: Color = Color::new(94, 75, 47);
    pub const SEPIA: Color = Color::new(127, 101, 63);
    pub const LIGHT_SEPIA: Color = Color::new(158, 134, 100);
    pub const LIGHTER_SEPIA: Color = Color::new(191, 171, 143);
    pub const LIGHTEST_SEPIA: Color = Color::new(222, 211, 195);

    /* desaturated */
    pub const DESATURATED_RED: Color = Color::new(127, 63, 63);
    pub const DESATURATED_FLAME: Color = Color::new(127, 79, 63);
    pub const DESATURATED_ORANGE: Color = Color::new(127, 95, 63);
    pub const DESATURATED_AMBER: Color = Color::new(127, 111, 63);
    pub const DESATURATED_YELLOW: Color = Color::new(127, 127, 63);
    pub const DESATURATED_LIME: Color = Color::new(111, 127, 63);
    pub const DESATURATED_CHARTREUSE: Color = Color::new(95, 127, 63);
    pub const DESATURATED_GREEN: Color = Color::new(63, 127, 63);
    pub const DESATURATED_SEA: Color = Color::new(63, 127, 95);
    pub const DESATURATED_TURQUOISE: Color = Color::new(63, 127, 111);
    pub const DESATURATED_CYAN: Color = Color::new(63, 127, 127);
    pub const DESATURATED_SKY: Color = Color::new(63, 111, 127);
    pub const DESATURATED_AZURE: Color = Color::new(63, 95, 127);
    pub const DESATURATED_BLUE: Color = Color::new(63, 63, 127);
    pub const DESATURATED_HAN: Color = Color::new(79, 63, 127);
    pub const DESATURATED_VIOLET: Color = Color::new(95, 63, 127);
    pub const DESATURATED_PURPLE: Color = Color::new(111, 63, 127);
    pub const DESATURATED_FUCHSIA: Color = Color::new(127, 63, 127);
    pub const DESATURATED_MAGENTA: Color = Color::new(127, 63, 111);
    pub const DESATURATED_PINK: Color = Color::new(127, 63, 95);
    pub const DESATURATED_CRIMSON: Color = Color::new(127, 63, 79);

    /* lightest */
    pub const LIGHTEST_RED: Color = Color::new(255, 191, 191);
    pub const LIGHTEST_FLAME: Color = Color::new(255, 207, 191);
    pub const LIGHTEST_ORANGE: Color = Color::new(255, 223, 191);
    pub const LIGHTEST_AMBER: Color = Color::new(255, 239, 191);
    pub const LIGHTEST_YELLOW: Color = Color::new(255, 255, 191);
    pub const LIGHTEST_LIME: Color = Color::new(239, 255, 191);
    pub const LIGHTEST_CHARTREUSE: Color = Color::new(223, 255, 191);
    pub const LIGHTEST_GREEN: Color = Color::new(191, 255, 191);
    pub const LIGHTEST_SEA: Color = Color::new(191, 255, 223);
    pub const LIGHTEST_TURQUOISE: Color = Color::new(191, 255, 239);
    pub const LIGHTEST_CYAN: Color = Color::new(191, 255, 255);
    pub const LIGHTEST_SKY: Color = Color::new(191, 239, 255);
    pub const LIGHTEST_AZURE: Color = Color::new(191, 223, 255);
    pub const LIGHTEST_BLUE: Color = Color::new(191, 191, 255);
    pub const LIGHTEST_HAN: Color = Color::new(207, 191, 255);
    pub const LIGHTEST_VIOLET: Color = Color::new(223, 191, 255);
    pub const LIGHTEST_PURPLE: Color = Color::new(239, 191, 255);
    pub const LIGHTEST_FUCHSIA: Color = Color::new(255, 191, 255);
    pub const LIGHTEST_MAGENTA: Color = Color::new(255, 191, 239);
    pub const LIGHTEST_PINK: Color = Color::new(255, 191, 223);
    pub const LIGHTEST_CRIMSON: Color = Color::new(255, 191, 207);

    /* lighter */
    pub const LIGHTER_RED: Color = Color::new(255, 127, 127);
    pub const LIGHTER_FLAME: Color = Color::new(255, 159, 127);
    pub const LIGHTER_ORANGE: Color = Color::new(255, 191, 127);
    pub const LIGHTER_AMBER: Color = Color::new(255, 223, 127);
    pub const LIGHTER_YELLOW: Color = Color::new(255, 255, 127);
    pub const LIGHTER_LIME: Color = Color::new(223, 255, 127);
    pub const LIGHTER_CHARTREUSE: Color = Color::new(191, 255, 127);
    pub const LIGHTER_GREEN: Color = Color::new(127, 255, 127);
    pub const LIGHTER_SEA: Color = Color::new(127, 255, 191);
    pub const LIGHTER_TURQUOISE: Color = Color::new(127, 255, 223);
    pub const LIGHTER_CYAN: Color = Color::new(127, 255, 255);
    pub const LIGHTER_SKY: Color = Color::new(127, 223, 255);
    pub const LIGHTER_AZURE: Color = Color::new(127, 191, 255);
    pub const LIGHTER_BLUE: Color = Color::new(127, 127, 255);
    pub const LIGHTER_HAN: Color = Color::new(159, 127, 255);
    pub const LIGHTER_VIOLET: Color = Color::new(191, 127, 255);
    pub const LIGHTER_PURPLE: Color = Color::new(223, 127, 255);
    pub const LIGHTER_FUCHSIA: Color = Color::new(255, 127, 255);
    pub const LIGHTER_MAGENTA: Color = Color::new(255, 127, 223);
    pub const LIGHTER_PINK: Color = Color::new(255, 127, 191);
    pub const LIGHTER_CRIMSON: Color = Color::new(255, 127, 159);

    /* light */
    pub const LIGHT_RED: Color = Color::new(255, 63, 63);
    pub const LIGHT_FLAME: Color = Color::new(255, 111, 63);
    pub const LIGHT_ORANGE: Color = Color::new(255, 159, 63);
    pub const LIGHT_AMBER: Color = Color::new(255, 207, 63);
    pub const LIGHT_YELLOW: Color = Color::new(255, 255, 63);
    pub const LIGHT_LIME: Color = Color::new(207, 255, 63);
    pub const LIGHT_CHARTREUSE: Color = Color::new(159, 255, 63);
    pub const LIGHT_GREEN: Color = Color::new(63, 255, 63);
    pub const LIGHT_SEA: Color = Color::new(63, 255, 159);
    pub const LIGHT_TURQUOISE: Color = Color::new(63, 255, 207);
    pub const LIGHT_CYAN: Color = Color::new(63, 255, 255);
    pub const LIGHT_SKY: Color = Color::new(63, 207, 255);
    pub const LIGHT_AZURE: Color = Color::new(63, 159, 255);
    pub const LIGHT_BLUE: Color = Color::new(63, 63, 255);
    pub const LIGHT_HAN: Color = Color::new(111, 63, 255);
    pub const LIGHT_VIOLET: Color = Color::new(159, 63, 255);
    pub const LIGHT_PURPLE: Color = Color::new(207, 63, 255);
    pub const LIGHT_FUCHSIA: Color = Color::new(255, 63, 255);
    pub const LIGHT_MAGENTA: Color = Color::new(255, 63, 207);
    pub const LIGHT_PINK: Color = Color::new(255, 63, 159);
    pub const LIGHT_CRIMSON: Color = Color::new(255, 63, 111);

    /* normal */
    pub const RED: Color = Color::new(255, 0, 0);
    pub const FLAME: Color = Color::new(255, 63, 0);
    pub const ORANGE: Color = Color::new(255, 127, 0);
    pub const AMBER: Color = Color::new(255, 191, 0);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const LIME: Color = Color::new(191, 255, 0);
    pub const CHARTREUSE: Color = Color::new(127, 255, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const SEA: Color = Color::new(0, 255, 127);
    pub const TURQUOISE: Color = Color::new(0, 255, 191);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const SKY: Color = Color::new(0, 191, 255);
    pub const AZURE: Color = Color::new(0, 127, 255);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const HAN: Color = Color::new(63, 0, 255);
    pub const VIOLET: Color = Color::new(127, 0, 255);
    pub const PURPLE: Color = Color::new(191, 0, 255);
    pub const FUCHSIA: Color = Color::new(255, 0, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 191);
    pub const PINK: Color = Color::new(255, 0, 127);
    pub const CRIMSON: Color = Color::new(255, 0, 63);

    /* dark */
    pub const DARK_RED: Color = Color::new(191, 0, 0);
    pub const DARK_FLAME: Color = Color::new(191, 47, 0);
    pub const DARK_ORANGE: Color = Color::new(191, 95, 0);
    pub const DARK_AMBER: Color = Color::new(191, 143, 0);
    pub const DARK_YELLOW: Color = Color::new(191, 191, 0);
    pub const DARK_LIME: Color = Color::new(143, 191, 0);
    pub const DARK_CHARTREUSE: Color = Color::new(95, 191, 0);
    pub const DARK_GREEN: Color = Color::new(0, 191, 0);
    pub const DARK_SEA: Color = Color::new(0, 191, 95);
    pub const DARK_TURQUOISE: Color = Color::new(0, 191, 143);
    pub const DARK_CYAN: Color = Color::new(0, 191, 191);
    pub const DARK_SKY: Color = Color::new(0, 143, 191);
    pub const DARK_AZURE: Color = Color::new(0, 95, 191);
    pub const DARK_BLUE: Color = Color::new(0, 0, 191);
    pub const DARK_HAN: Color = Color::new(47, 0, 191);
    pub const DARK_VIOLET: Color = Color::new(95, 0, 191);
    pub const DARK_PURPLE: Color = Color::new(143, 0, 191);
    pub const DARK_FUCHSIA: Color = Color::new(191, 0, 191);
    pub const DARK_MAGENTA: Color = Color::new(191, 0, 143);
    pub const DARK_PINK: Color = Color::new(191, 0, 95);
    pub const DARK_CRIMSON: Color = Color::new(191, 0, 47);

    /* darker */
    pub const DARKER_RED: Color = Color::new(127, 0, 0);
    pub const DARKER_FLAME: Color = Color::new(127, 31, 0);
    pub const DARKER_ORANGE: Color = Color::new(127, 63, 0);
    pub const DARKER_AMBER: Color = Color::new(127, 95, 0);
    pub const DARKER_YELLOW: Color = Color::new(127, 127, 0);
    pub const DARKER_LIME: Color = Color::new(95, 127, 0);
    pub const DARKER_CHARTREUSE: Color = Color::new(63, 127, 0);
    pub const DARKER_GREEN: Color = Color::new(0, 127, 0);
    pub const DARKER_SEA: Color = Color::new(0, 127, 63);
    pub const DARKER_TURQUOISE: Color = Color::new(0, 127, 95);
    pub const DARKER_CYAN: Color = Color::new(0, 127, 127);
    pub const DARKER_SKY: Color = Color::new(0, 95, 127);
    pub const DARKER_AZURE: Color = Color::new(0, 63, 127);
    pub const DARKER_BLUE: Color = Color::new(0, 0, 127);
    pub const DARKER_HAN: Color = Color::new(31, 0, 127);
    pub const DARKER_VIOLET: Color = Color::new(63, 0, 127);
    pub const DARKER_PURPLE: Color = Color::new(95, 0, 127);
    pub const DARKER_FUCHSIA: Color = Color::new(127, 0, 127);
    pub const DARKER_MAGENTA: Color = Color::new(127, 0, 95);
    pub const DARKER_PINK: Color = Color::new(127, 0, 63);
    pub const DARKER_CRIMSON: Color = Color::new(127, 0, 31);

    /* darkest */
    pub const DARKEST_RED: Color = Color::new(63, 0, 0);
    pub const DARKEST_FLAME: Color = Color::new(63, 15, 0);
    pub const DARKEST_ORANGE: Color = Color::new(63, 31, 0);
    pub const DARKEST_AMBER: Color = Color::new(63, 47, 0);
    pub const DARKEST_YELLOW: Color = Color::new(63, 63, 0);
    pub const DARKEST_LIME: Color = Color::new(47, 63, 0);
    pub const DARKEST_CHARTREUSE: Color = Color::new(31, 63, 0);
    pub const DARKEST_GREEN: Color = Color::new(0, 63, 0);
    pub const DARKEST_SEA: Color = Color::new(0, 63, 31);
    pub const DARKEST_TURQUOISE: Color = Color::new(0, 63, 47);
    pub const DARKEST_CYAN: Color = Color::new(0, 63, 63);
    pub const DARKEST_SKY: Color = Color::new(0, 47, 63);
    pub const DARKEST_AZURE: Color = Color::new(0, 31, 63);
    pub const DARKEST_BLUE: Color = Color::new(0, 0, 63);
    pub const DARKEST_HAN: Color = Color::new(15, 0, 63);
    pub const DARKEST_VIOLET: Color = Color::new(31, 0, 63);
    pub const DARKEST_PURPLE: Color = Color::new(47, 0, 63);
    pub const DARKEST_FUCHSIA: Color = Color::new(63, 0, 63);
    pub const DARKEST_MAGENTA: Color = Color::new(63, 0, 47);
    pub const DARKEST_PINK: Color = Color::new(63, 0, 31);
    pub const DARKEST_CRIMSON: Color = Color::new(63, 0, 15);

    /* metallic */
    pub const BRASS: Color = Color::new(191, 151, 96);
    pub const COPPER: Color = Color::new(197, 136, 124);
    pub const GOLD: Color = Color::new(229, 191, 0);
    pub const SILVER: Color = Color::new(203, 203, 203);

    /* miscellaneous */
    pub const CELADON: Color = Color::new(172, 255, 175);
    pub const PEACH: Color = Color::new(255, 159, 127);
}

impl Add for Color {
    type Output = Color;

    /// Add two colors together and return the result.
    fn add(self, rhs: Self) -> Self::Output {
        Self::new_with_alpha(
            self.r.saturating_add(rhs.r),
            self.g.saturating_add(rhs.g),
            self.b.saturating_add(rhs.b),
            self.a.saturating_add(rhs.a),
        )
    }
}

impl Sub for Color {
    type Output = Color;

    /// Subtract the right hand side from the left hand side and return the result.
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_with_alpha(
            self.r.saturating_sub(rhs.r),
            self.g.saturating_sub(rhs.g),
            self.b.saturating_sub(rhs.b),
            self.a.saturating_sub(rhs.a),
        )
    }
}

impl Mul for Color {
    type Output = Color;

    /// Multiply two colors together and return the result.
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_with_alpha(
            (i32::from(self.r) * i32::from(rhs.r) / 255) as u8,
            (i32::from(self.g) * i32::from(rhs.g) / 255) as u8,
            (i32::from(self.b) * i32::from(rhs.b) / 255) as u8,
            (i32::from(self.a) * i32::from(rhs.a) / 255) as u8,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    /// Multiply a color with a scalar value and return the result.
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new_with_alpha(
            (f32::from(self.r) * rhs).min(255.0).max(0.0) as u8,
            (f32::from(self.g) * rhs).min(255.0).max(0.0) as u8,
            (f32::from(self.b) * rhs).min(255.0).max(0.0) as u8,
            (f32::from(self.a) * rhs).min(255.0).max(0.0) as u8,
        )
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(c: Color) -> Self {
        (c.r, c.g, c.b)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<Color> for doryen_rs::Color {
    fn from(c: Color) -> Self {
        (c.r, c.g, c.b, c.a)
    }
}

impl From<doryen_rs::Color> for Color {
    fn from((r, g, b, a): doryen_rs::Color) -> Self {
        Self::new_with_alpha(r, g, b, a)
    }
}

/// Color names
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Name {
    Red,
    Flame,
    Orange,
    Amber,
    Yellow,
    Lime,
    Chartreuse,
    Green,
    Sea,
    Turquoise,
    Cyan,
    Sky,
    Azure,
    Blue,
    Han,
    Violet,
    Purple,
    Fuchsia,
    Magenta,
    Pink,
    Crimson,
}

/// Color levels
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Level {
    Desaturated,
    Lightest,
    Lighter,
    Light,
    Normal,
    Dark,
    Darker,
    Darkest,
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn hsv() {
        let red = Color::new_hsv(0., 1., 1.);
        let green = Color::new_hsv(120., 1., 1.);
        let blue = Color::new_hsv(240., 1., 1.);

        assert_eq!(red, Color::new(255, 0, 0));
        assert_eq!(green, Color::new(0, 255, 0));
        assert_eq!(blue, Color::new(0, 0, 255));

        let yellow = Color::new_hsv(60., 1., 1.);
        let cyan = Color::new_hsv(180., 1., 1.);
        let magenta = Color::new_hsv(300., 1., 1.);

        assert_eq!(yellow, Color::new(255, 255, 0));
        assert_eq!(cyan, Color::new(0, 255, 255));
        assert_eq!(magenta, Color::new(255, 0, 255));

        let black = Color::new_hsv(0., 0., 0.);
        let white = Color::new_hsv(0., 0., 1.);
        let gray = Color::new_hsv(0., 0., 0.5);
        let silver = Color::new_hsv(0., 0., 0.75);

        assert_eq!(black, Color::new(0, 0, 0));
        assert_eq!(white, Color::new(255, 255, 255));
        assert_eq!(gray, Color::new(128, 128, 128));
        assert_eq!(silver, Color::new(191, 191, 191));
    }

    #[test]
    fn lerp() {
        let black = Color::BLACK;
        let white = Color::WHITE;

        let left = black.lerp_rgb(white, 0.0);
        let right = black.lerp_rgb(white, 1.0);
        let middle = black.lerp_rgb(white, 0.5);

        assert_eq!(left, black);
        assert_eq!(right, white);
        assert_eq!(middle, Color::GRAY);

        let left = black.lerp_hsv(white, 0.0);
        let right = black.lerp_hsv(white, 1.0);
        let middle = black.lerp_hsv(white, 0.5);

        assert_eq!(left, black);
        assert_eq!(right, white);
        assert_eq!(middle, Color::new(128, 128, 128));

        let orange = Color::ORANGE;
        let cyan = Color::CYAN;

        let middle = orange.lerp_rgb(cyan, 0.5);
        assert_eq!(middle, Color::new(127, 191, 127));

        let middle = orange.lerp_hsv(cyan, 0.5);
        assert_eq!(middle, Color::new(64, 255, 0));

        let middle = Color::LIGHTEST_RED.lerp_rgb(Color::LIGHT_BLUE, 0.5);
        assert_eq!(middle, Color::new(159, 127, 223));

        let middle = Color::LIGHTEST_RED.lerp_hsv(Color::LIGHT_BLUE, 0.5);
        assert_eq!(middle, Color::LIGHTER_FUCHSIA);
    }

    #[test]
    fn gradient() {
        let grayscale = Color::generate_gradient_rgb(&[Color::BLACK, Color::WHITE], &[254]);

        assert_eq!(grayscale.len(), 256);
        for (i, color) in grayscale.iter().enumerate() {
            assert_eq!(color.r, i as u8);
            assert_eq!(color.g, i as u8);
            assert_eq!(color.b, i as u8);
        }

        let grayscale = Color::generate_gradient_hsv(&[Color::BLACK, Color::WHITE], &[254]);

        assert_eq!(grayscale.len(), 256);
        for (i, color) in grayscale.iter().enumerate() {
            assert_eq!(color.r, i as u8);
            assert_eq!(color.g, i as u8);
            assert_eq!(color.b, i as u8);
        }
    }
}
