use gpui::*;
use gpui_component::{button::*, input::*, scroll::ScrollableElement, *};
use gpui_component_assets::Assets;

use std::collections::HashMap;

use crate::parse_file;

#[derive(Clone)]
struct Warnings {
    msgs: HashMap<String, Vec<String>>,
}

impl Global for Warnings {}

#[derive(Clone)]
struct Data {
    msg: String,
}

impl Global for Data {}

#[derive(Clone)]
pub struct HelloWorld {
    editor: Entity<InputState>,
}

impl HelloWorld {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("text")
                .multi_line(true)
                .line_number(true)
                .soft_wrap(false)
        });
        Self { editor }
    }
    fn handle(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
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
            // Some(text)
            entity
                .update(app, |_, context| {
                    Data::update_global(context, |state, _app| {
                        state.msg = text;
                    });
                    context.notify();
                })
                .ok()
        })
        .detach();
        // cx.notify();
    }
}

impl Render for HelloWorld {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = TitleBar::new()
            .bg(gray(100))
            .child(div().child("节目单转换工具"));

        let msg = Data::global(cx).msg.clone();
        self.editor.update(cx, |state, cx| {
            state.set_value(msg, window, cx);
        });

        let foobar = div()
            .v_flex()
            .justify_start()
            .content_stretch()
            .h_full()
            .bg(gpui_component::cyan_100())
            .child(titlebar.w_full())
            .child(
                div()
                    .bg(gpui_component::green_100())
                    .v_flex()
                    .flex_grow()
                    .content_stretch()
                    .items_center()
                    .justify_start()
                    .child(
                        div()
                            .flex()
                            .child(
                                Button::new("open")
                                    .primary()
                                    .m_16()
                                    .label("选择节目单")
                                    .on_click(cx.listener(Self::handle)),
                            )
                            .child(
                                Button::new("inspect")
                                    .primary()
                                    .m_16()
                                    .label("switch")
                                    .on_click(cx.listener(|_, _, window, cx| {
                                        window.toggle_inspector(cx);
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .content_stretch()
                            .flex_grow()
                            .bg(gpui_component::theme::pink_100())
                            .child(
                                div()
                                    .w_5_6()
                                    .pr_4()
                                    .bg(blue_100())
                                    .h(AbsoluteLength::Pixels(Pixels::from(512f32)))
                                    .child(
                                        Input::new(&self.editor)
                                            .content_stretch()
                                            .h(AbsoluteLength::Pixels(Pixels::from(4096f32)))
                                            .bg(yellow_100()),
                                    )
                                    .overflow_y_scrollbar(),
                            ),
                    )
                    .child(div().w_full().v_flex().bg(red_100()).child("foobar")),
            );
        for (name, warnings) in Warnings::global(cx).msgs.iter() {
            println!("foobar: {name}, {}", warnings.len());
        }
        foobar
    }
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
        Data::set_global(
            cx,
            Data {
                msg: String::default(),
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
                    view.update(cx, |hello, _cx| {
                        hello.editor.update(_cx, |state, cx| {
                            state.set_value("abc", window, cx);
                        });
                    });
                    // This first level on the window, should be a Root.
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
