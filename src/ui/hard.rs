use gpui::*;
use gpui_component::{button::*, input::*, *};
use gpui_component_assets::Assets;

use std::collections::HashMap;

use crate::parse_file;

#[derive(Clone)]
struct Warnings {
    msgs: HashMap<String, Vec<String>>,
}

impl Global for Warnings {}

#[derive(Clone)]
pub struct HelloWorld {
    editor: Entity<InputState>,
    display_text: String,
}

impl HelloWorld {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("text")
                .multi_line(true)
                .line_number(true)
        });
        Self {
            editor,
            display_text: String::default(),
        }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = TitleBar::new()
            .bg(gray(100))
            .child(div().child("节目单转换工具"));
        println!("text: {}", self.display_text);
        let foobar = div().child(titlebar).child(
            div()
                .v_flex()
                .gap_2()
                .size_full()
                .items_center()
                .justify_center()
                .child(
                    Button::new("open")
                        .primary()
                        .m_16()
                        .label("选择节目单")
                        .on_click(cx.listener(|_, _, _, cx| {
                            let destination_prompt = cx.prompt_for_paths(PathPromptOptions {
                                files: true,
                                directories: false,
                                multiple: false,
                                prompt: None,
                            });
                            cx.spawn(async move |entity, app| {
                                let mut paths = destination_prompt.await.ok()?.ok()??;
                                let destination_file = paths.pop()?;
                                let text = parse_file(&destination_file).ok()?;
                                entity
                                    .update(app, |hello, context| {
                                        hello.display_text = text;
                                        context.notify();
                                    })
                                    .ok()
                            })
                            .detach();
                        })),
                )
                .child(Input::new(&self.editor).flex_1()),
        );
        for (name, warnings) in Warnings::global(cx).msgs.iter() {
            println!("foobar: {name}, {}", warnings.len());
        }
        foobar.child(self.display_text.clone())
    }
}

fn draw(app: &mut App, window: &mut Window, handle: &Entity<HelloWorld>) {
    app.update_entity(handle, |this, cx| {
        let text = this.display_text.clone();
        this.editor.update(cx, |state, cx| {
            state.set_value(text, window, cx);
        })
    });
}

pub fn hello() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        Warnings::set_global(
            cx,
            Warnings {
                msgs: HashMap::default(),
            },
        );
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: None,
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_decorations: Some(WindowDecorations::Client),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|context| HelloWorld::new(window, context));
                    draw(cx, window, &view);
                    // This first level on the window, should be a Root.
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
