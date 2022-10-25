use std::{ sync::atomic::{ AtomicU16, Ordering } };

use crossterm::{ cursor, Result };
use crate::screen::Screen;

pub enum Offset {
    Row(i16),
    Column(i16),
    None,
}

#[derive(Clone, Copy)]
pub struct Cursor {}

impl Cursor {
    pub fn cursor() -> (u16, u16) {
        unsafe { (Cursor::cy().load(Ordering::Relaxed), Cursor::cx().load(Ordering::Relaxed)) }
    }

    unsafe fn cx() -> &'static mut AtomicU16 {
        static mut CX: AtomicU16 = AtomicU16::new(0);
        &mut CX
    }

    unsafe fn cy() -> &'static mut AtomicU16 {
        static mut CY: AtomicU16 = AtomicU16::new(0);
        &mut CY
    }

    pub fn cx_add(step: u16) -> u16 {
        unsafe { Cursor::cx().fetch_add(step, Ordering::Relaxed) }
    }

    pub fn cy_add(step: u16) -> u16 {
        unsafe { Cursor::cy().fetch_add(step, Ordering::Relaxed) }
    }

    pub fn cx_sub(step: u16) -> u16 {
        unsafe { Cursor::cx().fetch_sub(step, Ordering::Relaxed) }
    }

    pub fn cy_sub(step: u16) -> u16 {
        unsafe { Cursor::cy().fetch_sub(step, Ordering::Relaxed) }
    }

    pub fn hide(screen: &mut Screen) -> Result<()> {
        screen.write_str(format!("{}", cursor::Hide).as_str())?;
        Ok(())
    }

    pub fn show(screen: &mut Screen) -> Result<()> {
        screen.write_str(format!("{}", cursor::Show).as_str())?;
        Ok(())
    }

    pub fn move_to(screen: &mut Screen, col: u16, row: u16) -> Result<()> {
        screen.write_str(format!("{}", cursor::MoveTo(col, row)).as_str())?;
        Ok(())
    }

    pub fn move_up(screen: &mut Screen) -> Result<Offset> {
        let (cy, cx) = Cursor::cursor();
        if cx > screen.min_row() {
            Cursor::cx_sub(1);
            Cursor::move_to(screen, cy, cx)?;
        } else {
            return Ok(Offset::Row(-1));
        }
        Ok(Offset::None)
    }

    pub fn move_down(screen: &mut Screen) -> Result<Offset> {
        let (cy, cx) = Cursor::cursor();
        if cx < screen.max_row() {
            Cursor::cx_add(1);
            Cursor::move_to(screen, cy, cx)?;
        } else {
            return Ok(Offset::Row(1));
        }
        Ok(Offset::None)
    }

    pub fn move_left(screen: &mut Screen) -> Result<Offset> {
        let (cy, cx) = Cursor::cursor();
        if cy > screen.min_col() {
            Cursor::cy_sub(1);
            Cursor::move_to(screen, cy, cx)?;
        } else {
            return Ok(Offset::Column(-1));
        }
        Ok(Offset::None)
    }

    pub fn move_right(screen: &mut Screen) -> Result<Offset> {
        let (cy, cx) = Cursor::cursor();
        if cy < screen.max_col() {
            Cursor::cy_add(1);
            Cursor::move_to(screen, cy, cx)?;
        } else {
            return Ok(Offset::Column(1));
        }
        Ok(Offset::None)
    }

    pub fn move_top(screen: &mut Screen) -> Result<()> {
        let (cy, cx) = Cursor::cursor();
        Cursor::cx_sub(cx);
        Cursor::move_to(screen, cy, 0)?;
        Ok(())
    }

    pub fn move_bottom(screen: &mut Screen) -> Result<()> {
        let (cy, cx) = Cursor::cursor();
        Cursor::cx_add(screen.max_row() - cx);
        Cursor::move_to(screen, cy, screen.max_row())?;
        Ok(())
    }

    pub fn move_left_border(screen: &mut Screen) -> Result<()> {
        let (cy, cx) = Cursor::cursor();
        Cursor::cy_sub(cy);
        Cursor::move_to(screen, 0, cx)?;
        Ok(())
    }

    pub fn move_right_border(screen: &mut Screen) -> Result<()> {
        let (cy, cx) = Cursor::cursor();
        Cursor::cy_add(screen.max_col() - cy);
        Cursor::move_to(screen, screen.max_col(), cx)?;
        Ok(())
    }
}