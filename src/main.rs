#![windows_subsystem = "windows"]
use druid::widget::{
    Button, Container, CrossAxisAlignment, Flex, List, Padding, RadioGroup, Scroll, SizedBox,
    TextBox, WidgetExt,
};
use druid::{
    AppDelegate, AppLauncher, Data, DelegateCtx, Env, EventCtx, Lens, Widget, WindowDesc, WindowId,
};
use std::sync::Arc;
mod todo_util;
use todo_util::*;
const TODO_PATH: &str = "./todo.js";

fn main() -> Result<(), druid::PlatformError> {
    let todo_list = load_or_new(TODO_PATH);
    let app_state = AppState {
        todo_list: Arc::from(todo_list),
        new: String::default(),
    };
    let main_window = WindowDesc::new(ui_builder)
        .title("todo.rs")
        .with_min_size((700., 800.));
    AppLauncher::with_window(main_window)
        .delegate(Delegate::new())
        .launch(app_state)?;
    Ok(())
}

fn ui_builder() -> impl Widget<AppState> {
    let header = Flex::row()
        .with_child(SizedBox::new(TextBox::new().lens(AppState::new)).width(300.))
        .with_child(Button::new("Add item").on_click(
            |_ctx: &mut EventCtx, state: &mut AppState, _: &Env| {
                let new_item = TodoItem::new(state.new.to_owned());
                Arc::make_mut(&mut state.todo_list).push(new_item);
                state.new.clear()
            },
        ))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_flex_spacer(0.1)
        .with_child(Button::new("Remove").on_click(
            |_ctx: &mut EventCtx, state: &mut AppState, _: &Env| {
                Arc::make_mut(&mut state.todo_list).retain(|item| item.state != "TO_BE_REMOVED");
            },
        ));
    let list = Scroll::new(List::new(|| {
        let state_radio = Flex::row()
            .with_child(RadioGroup::new(vec![
                ("Remove", "TO_BE_REMOVED".to_owned()),
                ("Done", "DONE".to_owned()),
            ]))
            .with_child(RadioGroup::new(vec![
                ("In progress", "STARTED".to_owned()),
                ("Pending", "PENDING".to_owned()),
            ]))
            .lens(TodoItem::state);
        let textbox = TextBox::new().lens(TodoItem::desc);
        Container::new(
            Flex::row()
                .with_flex_child(state_radio, 0.)
                .with_spacer(8.0)
                .with_flex_child(SizedBox::new(textbox).width(400.), 1.),
        )
        .border(druid::Color::grey(0.5), 0.2)
    }))
    .vertical()
    .lens(AppState::todo_list);
    Flex::column()
        .with_child(Padding::new((10., 10.), header))
        .with_flex_child(list, 1.)
        .expand()
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
        self.windows.push(id)
    }
    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        // FIXME: looks fragile
        match id == self.windows[0] {
            true => match save_todo(&data.todo_list, TODO_PATH) {
                Ok(_) => {}
                Err(e) => {
                    println!("Caught {:?} when saving", e);
                    /* In practice saving will fail silently, because there
                    doesn't seem to be a way to catch quit event early enough*/
                }
            },
            _ => {}
        }
    }
}
