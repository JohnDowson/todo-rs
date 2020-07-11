#![windows_subsystem = "windows"]
use druid::widget::{Button, Checkbox, Flex, List, RadioGroup, Scroll, TextBox, WidgetExt};
use druid::{
    AppDelegate, AppLauncher, Data, DelegateCtx, Env, EventCtx, Lens, Widget, WindowDesc, WindowId,
};
use std::sync::Arc;
mod todo_util;
use todo_util::*;

fn main() -> Result<(), druid::PlatformError> {
    let todo_list = load_or_new();
    let app_state = AppState {
        todo_list: Arc::from(todo_list),
        new: String::default(),
    };
    let main_window = WindowDesc::new(ui_builder)
        .title("todo.rs")
        .window_size((400.0, 400.0));
    AppLauncher::with_window(main_window)
        .delegate(Delegate::new())
        .launch(app_state)?;
    Ok(())
}

fn ui_builder() -> impl Widget<AppState> {
    let header = Flex::row()
        .with_child(TextBox::new().lens(AppState::new))
        .with_child(Button::new("Add item").on_click(
            |_ctx: &mut EventCtx, state: &mut AppState, _: &Env| {
                let new_item = TodoItem::new(state.new.to_owned());
                Arc::make_mut(&mut state.todo_list).push(new_item);
                state.new.clear()
            },
        ))
        .with_flex_spacer(0.1)
        .with_child(Button::new("Remove").on_click(
            |_ctx: &mut EventCtx, state: &mut AppState, _: &Env| {
                Arc::make_mut(&mut state.todo_list).retain(|item| !item.to_be_removed);
            },
        ));
    let list = Scroll::new(List::new(|| {
        let state_radio = RadioGroup::new(vec![
            ("Done", TodoState::Done),
            ("In progress", TodoState::Started),
            ("Pending", TodoState::Pending),
        ])
        .lens(TodoItem::state);
        Flex::row()
            .with_child(state_radio)
            .with_child(TextBox::new().lens(TodoItem::desc))
            .with_child(Checkbox::new("Remove").lens(TodoItem::to_be_removed))
            .on_click(|_: &mut EventCtx, state: &mut TodoItem, _: &Env| println!("{:?}", state))
    }))
    .lens(AppState::todo_list);
    Flex::column().with_child(header).with_child(list)
}

#[derive(Data, Clone, Lens, Debug)]
struct AppState {
    pub todo_list: Arc<Vec<TodoItem>>,
    pub new: String,
}

struct Delegate {
    windows: Vec<WindowId>,
}
impl Delegate {
    pub fn new() -> Delegate {
        Delegate {
            windows: Vec::new(),
        }
    }
}
impl AppDelegate<AppState> for Delegate {
    fn window_added(
        &mut self,
        id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window added, id: {:?}", id);
        self.windows.push(id)
    }
    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        println!("Window removed, id: {:?}", id);
        match id == self.windows[0] {
            true => match save_todo(&data.todo_list, "./todo.json".to_owned()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Caught {:?} when saving", e);
                }
            },
            _ => {}
        }
    }
}
