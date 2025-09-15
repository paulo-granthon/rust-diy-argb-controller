use core::cell::Cell;

use crate::{r#static::Static, state::STATE};

pub struct MenuItem<'a, A>
where
    A: FnMut(),
{
    pub name: &'a str,
    pub action: A,
}

pub struct Menu<'a, A, const N: usize>
where
    A: FnMut(),
{
    pub items: [MenuItem<'a, A>; N],
    pub selected: Cell<usize>,
}

impl<'a, A, const N: usize> Menu<'a, A, N>
where
    A: FnMut(),
{
    pub const fn new(items: [MenuItem<'a, A>; N]) -> Self {
        Menu {
            items,
            selected: Cell::new(0),
        }
    }

    pub fn next(&self) {
        self.selected
            .set(self.selected.get().wrapping_add(1) % self.items.len())
    }

    #[allow(dead_code)]
    pub fn previous(&self) {
        self.selected
            .set(self.selected.get().wrapping_sub(1) % self.items.len())
    }

    pub fn current_item(&self) -> &MenuItem<'a, A> {
        &self.items[self.selected.get()]
    }
}

pub static MENU: Static<Menu<'static, fn(), 2>> = Static::new(Menu::new([
    MenuItem {
        name: "Offset",
        action: || STATE.with(|s| s.phase_offset.set((s.phase_offset.get() + 1) % 8)),
    },
    MenuItem {
        name: "Brightness",
        action: || STATE.with(|s| s.brightness.set(s.brightness.get().wrapping_sub(32))),
    },
]));
