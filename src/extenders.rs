/* BSD 3-Clause License
 *
 * Copyright © 2019, Alexander Krivács Schrøder <alexschrod@gmail.com>.
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

//! # Extenders for doryen-rs types.

use crate::color::Color;
use crate::{Position, Rectangle, USize};
use doryen_rs::{Console, TextAlign};
use ilyvion_util::ownership::Borrowned;
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

/// Extends the `Console` from `doryen-rs`.
///
/// Replaces most instances of x/y and w/h with `Position` and `USize` respectively, and makes use
/// of `doryen-extra`'s `Color` type rather than `doryen-rs`'. Since the `ConsoleExtender` derefs
/// to `Console`, it lets you both use the replaced/extended methods and the underlying methods
/// that were not replaced with ease.
#[allow(missing_debug_implementations)] // Console doesn't implement Debug
pub struct ConsoleExtender<'b> {
    console: Borrowned<'b, Console>,
}

impl<'b> ConsoleExtender<'b> {
    /// Wraps the mutably borrowed console.
    pub fn extend(console: &'b mut Console) -> Self {
        Self {
            console: Borrowned::Borrowed(console),
        }
    }

    /// Creates a new offscreen console that you can blit on another console.
    /// Size is in cells (characters), not pixels.
    pub fn new(size: USize) -> Self {
        Self {
            console: Borrowned::Owned(Console::new(size.width, size.height)),
        }
    }

    /// Wraps the owned console.
    pub fn wrap(console: Console) -> Self {
        Self {
            console: Borrowned::Owned(console),
        }
    }
}

// The replaced methods
impl ConsoleExtender<'_> {
    /// Resizes the console.
    pub fn resize(&mut self, size: USize) {
        self.console.resize(size.width, size.height);
    }

    /// Associates a name with a color for this console.
    pub fn register_color<S: AsRef<str>>(&mut self, name: S, value: Color) {
        self.console.register_color(name.as_ref(), value.into());
    }

    /// Gets the background color of a cell.
    pub fn back(&self, position: Position) -> Option<Color> {
        self.console
            .get_back(position.x, position.y)
            .map(Into::into)
    }

    /// Gets the foreground color of a cell.
    pub fn fore(&self, position: Position) -> Option<Color> {
        self.console
            .get_fore(position.x, position.y)
            .map(Into::into)
    }

    /// Gets the ASCII code of a cell.
    pub fn ascii(&self, position: Position) -> Option<u16> {
        self.console.get_ascii(position.x, position.y)
    }

    /// Gets the background color of a cell with no boundary check.
    pub fn back_unchecked(&self, position: Position) -> Color {
        self.console.unsafe_get_back(position.x, position.y).into()
    }

    /// Gets the foreground color of a cell with no boundary check.
    pub fn fore_unchecked(&self, position: Position) -> Color {
        self.console.unsafe_get_fore(position.x, position.y).into()
    }

    /// Gets the ASCII code of a cell with no boundary check.
    pub fn ascii_unchecked(&self, position: Position) -> u16 {
        self.console.unsafe_get_ascii(position.x, position.y)
    }

    /// Sets the character at a specific position.
    pub fn set_ascii(&mut self, position: Position, ascii: u16) {
        self.console.ascii(position.x, position.y, ascii);
    }

    /// Sets the character color at a specific position.
    pub fn set_fore(&mut self, position: Position, color: Color) {
        self.console.fore(position.x, position.y, color.into());
    }

    /// Sets the background color at a specific position.
    pub fn set_back(&mut self, position: Position, color: Color) {
        self.console.back(position.x, position.y, color.into());
    }

    /// Sets the character at a specific position with no boundary check.
    pub fn set_ascii_unchecked(&mut self, position: Position, ascii: u16) {
        self.console.unsafe_ascii(position.x, position.y, ascii);
    }

    /// Sets the character color at a specific position with no boundary check.
    pub fn set_fore_unchecked(&mut self, position: Position, color: Color) {
        self.console
            .unsafe_fore(position.x, position.y, color.into());
    }

    /// Sets the background color at a specific position  with no boundary check
    pub fn set_back_unchecked(&mut self, position: Position, color: Color) {
        self.console
            .unsafe_back(position.x, position.y, color.into());
    }

    /// Fills the whole console with values
    pub fn clear(&mut self, fore: Option<Color>, back: Option<Color>, fill_char: Option<u16>) {
        self.console
            .clear(fore.map(Into::into), back.map(Into::into), fill_char);
    }

    /// Writes a multi-color string. Foreground color is defined by #[color_name] patterns inside the string.
    pub fn print_color<S: AsRef<str>>(
        &mut self,
        position: Position,
        text: S,
        align: TextAlign,
        back: Option<Color>,
    ) {
        self.console.print_color(
            position.x,
            position.y,
            text.as_ref(),
            align,
            back.map(Into::into),
        )
    }

    /// Compute the length of a string containing color codes.
    pub fn text_color_len<S: AsRef<str>>(text: S) -> usize {
        Console::text_color_len(text.as_ref())
    }

    /// Writes a string. If the string reaches the border of the console, it's truncated.
    /// If the string contains carriage return `"\n"`, multiple lines are printed.
    pub fn print<S: AsRef<str>>(
        &mut self,
        position: Position,
        text: S,
        align: TextAlign,
        fore: Option<Color>,
        back: Option<Color>,
    ) {
        self.console.print(
            position.x,
            position.y,
            text.as_ref(),
            align,
            fore.map(Into::into),
            back.map(Into::into),
        );
    }

    /// Draws a rectangle, possibly filling it with a character.
    pub fn rectangle(
        &mut self,
        rectangle: Rectangle,
        fore: Option<Color>,
        back: Option<Color>,
        fill_char: Option<u16>,
    ) {
        self.console.rectangle(
            rectangle.position.x,
            rectangle.position.y,
            rectangle.size.width,
            rectangle.size.height,
            fore.map(Into::into),
            back.map(Into::into),
            fill_char,
        );
    }

    /// Fills an area with values.
    pub fn area(
        &mut self,
        rectangle: Rectangle,
        fore: Option<Color>,
        back: Option<Color>,
        fill_char: Option<u16>,
    ) {
        self.console.area(
            rectangle.position.x,
            rectangle.position.y,
            rectangle.size.width,
            rectangle.size.height,
            fore.map(Into::into),
            back.map(Into::into),
            fill_char,
        );
    }

    /// Changes all the properties of a console cell at once.
    pub fn cell(
        &mut self,
        position: Position,
        ascii: Option<u16>,
        fore: Option<Color>,
        back: Option<Color>,
    ) {
        self.console.cell(
            position.x,
            position.y,
            ascii,
            fore.map(Into::into),
            back.map(Into::into),
        );
    }

    /// Blits (draw) a console onto another one.
    pub fn blit(
        &self,
        position: Position,
        destination: &mut Console,
        fore_alpha: f32,
        back_alpha: f32,
        key_color: Option<Color>,
    ) {
        self.console.blit(
            position.x,
            position.y,
            destination,
            fore_alpha,
            back_alpha,
            key_color.map(Into::into),
        );
    }

    /// Blits a region of this console onto another one.
    pub fn blit_ex(
        &self,
        source_rectangle: Rectangle,
        destination: &mut Console,
        destination_position: Position,
        fore_alpha: f32,
        back_alpha: f32,
        key_color: Option<Color>,
    ) {
        self.console.blit_ex(
            source_rectangle.position.x,
            source_rectangle.position.y,
            source_rectangle.size.width as i32,
            source_rectangle.size.height as i32,
            destination,
            destination_position.x,
            destination_position.y,
            fore_alpha,
            back_alpha,
            key_color.map(Into::into),
        );
    }
}

// The extended methods
impl ConsoleExtender<'_> {
    /// Returns the size of the console.
    pub fn get_size(&self) -> USize {
        USize::new(self.console.get_width(), self.console.get_height())
    }

    /// Draws a rectangle, possibly filling it with a character, possibly with a title centered
    /// at the top.
    pub fn print_frame<S: AsRef<str>>(
        &mut self,
        rectangle: Rectangle,
        fore: Option<Color>,
        back: Option<Color>,
        fill: Option<u16>,
        title: Option<S>,
    ) {
        self.rectangle(rectangle, fore.clone(), back.clone(), fill);

        if let Some(title) = title {
            let text = format!(" {} ", title.as_ref());
            let Rectangle {
                position: Position { x, y },
                size: USize { width: w, .. },
            } = rectangle;
            self.print(
                Position::new(x + (w / 2) as i32, y),
                &text,
                TextAlign::Center,
                fore.map(Into::into),
                back.map(Into::into),
            );
        }
    }

    /// Prints the provided character to the give position.
    pub fn print_char(
        &mut self,
        position: Position,
        character: char,
        fore: Option<Color>,
        back: Option<Color>,
    ) {
        self.cell(position, Some(character as u16), fore, back);
    }
}

impl Deref for ConsoleExtender<'_> {
    type Target = Console;

    fn deref(&self) -> &Self::Target {
        self.console.deref()
    }
}

impl DerefMut for ConsoleExtender<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.console.deref_mut()
    }
}

impl Borrow<Console> for ConsoleExtender<'_> {
    fn borrow(&self) -> &Console {
        self.console.borrow()
    }
}

impl BorrowMut<Console> for ConsoleExtender<'_> {
    fn borrow_mut(&mut self) -> &mut Console {
        self.console.borrow_mut()
    }
}

impl AsRef<Console> for ConsoleExtender<'_> {
    fn as_ref(&self) -> &Console {
        self.console.as_ref()
    }
}

impl AsMut<Console> for ConsoleExtender<'_> {
    fn as_mut(&mut self) -> &mut Console {
        self.console.as_mut()
    }
}
