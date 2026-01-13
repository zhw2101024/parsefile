use gpui::*;
use gpui_component::{button::*, input::*, *};
use gpui_component_assets::Assets;

use crate::parse_file;

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
        });
        Self { editor }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = TitleBar::new()
            .bg(gray(100))
            .child(div().child("节目单转换工具"));
        div().child(titlebar).child(
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
                        .on_click(cx.listener(|_, _, window, cx| {
                            let destination_prompt = cx.prompt_for_paths(PathPromptOptions {
                                files: true,
                                directories: false,
                                multiple: false,
                                prompt: None,
                            });
                            window
                                .spawn(cx, async move |_| {
                                    let mut paths = destination_prompt.await.ok()?.ok()??;
                                    let destination_file = paths.pop()?;
                                    match parse_file(&destination_file) {
                                        Ok(number) => {
                                            println!("passed {} records", number);
                                        }
                                        Err(e) => println!("{}", e),
                                    }
                                    Some(())
                                })
                                .detach();
                        })),
                )
                .child(Input::new(&self.editor).flex_1()),
        )
    }
}

fn draw(app: &mut App, window: &mut Window, handle: &Entity<HelloWorld>) {
    app.update_entity(handle, |this, cx| {
        this.editor.update(cx, |state, cx| {
            state.set_value("foobar", window, cx);
        })
    });
}

pub fn hello() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
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
