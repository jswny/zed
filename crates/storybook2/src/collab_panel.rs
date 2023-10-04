use crate::theme::{theme, Theme};
use gpui3::{
    div, img, svg, view, AppContext, ArcCow, Context, Element, IntoAnyElement, ParentElement,
    ScrollState, StyleHelpers, View, ViewContext, WindowContext,
};

pub struct CollabPanel {
    scroll_state: ScrollState,
}

pub fn collab_panel<S: 'static>(cx: &mut WindowContext) -> View<CollabPanel, S> {
    view(cx.entity(|cx| CollabPanel::new(cx)), CollabPanel::render)
}

impl CollabPanel {
    fn new(_: &mut AppContext) -> Self {
        CollabPanel {
            scroll_state: ScrollState::default(),
        }
    }
}

impl CollabPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<State = Self> {
        let theme = theme(cx);

        // Panel
        div()
            .w_64()
            .h_full()
            .flex()
            .flex_col()
            .font("Courier")
            .text_color(theme.middle.base.default.foreground)
            .border_color(theme.middle.base.default.border)
            .border()
            .fill(theme.middle.base.default.background)
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll(self.scroll_state.clone())
                    // List Container
                    .child(
                        div()
                            .fill(theme.lowest.base.default.background)
                            .pb_1()
                            .border_color(theme.lowest.base.default.border)
                            .border_b()
                            //:: https://tailwindcss.com/docs/hover-focus-and-other-states#styling-based-on-parent-state
                            // .group()
                            // List Section Header
                            .child(self.list_section_header("#CRDB 🗃️", true, theme))
                            // List Item Large
                            .child(self.list_item(
                                "http://github.com/maxbrunsfeld.png?s=50",
                                "maxbrunsfeld",
                                theme,
                            )),
                    )
                    .child(
                        div()
                            .py_2()
                            .flex()
                            .flex_col()
                            .child(self.list_section_header("CHANNELS", true, theme)),
                    )
                    .child(
                        div()
                            .py_2()
                            .flex()
                            .flex_col()
                            .child(self.list_section_header("CONTACTS", true, theme))
                            .children(
                                std::iter::repeat_with(|| {
                                    vec![
                                        self.list_item(
                                            "http://github.com/as-cii.png?s=50",
                                            "as-cii",
                                            theme,
                                        ),
                                        self.list_item(
                                            "http://github.com/nathansobo.png?s=50",
                                            "nathansobo",
                                            theme,
                                        ),
                                        self.list_item(
                                            "http://github.com/maxbrunsfeld.png?s=50",
                                            "maxbrunsfeld",
                                            theme,
                                        ),
                                    ]
                                })
                                .take(10)
                                .flatten(),
                            ),
                    ),
            )
            .child(
                div()
                    .h_7()
                    .px_2()
                    .border_t()
                    .border_color(theme.middle.variant.default.border)
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.middle.variant.default.foreground)
                            .child("Find..."),
                    ),
            )
    }

    fn list_section_header(
        &self,
        label: impl IntoAnyElement<Self>,
        expanded: bool,
        theme: &Theme,
    ) -> impl Element<State = Self> {
        div()
            .h_7()
            .px_2()
            .flex()
            .justify_between()
            .items_center()
            .child(div().flex().gap_1().text_sm().child(label))
            .child(
                div().flex().h_full().gap_1().items_center().child(
                    svg()
                        .path(if expanded {
                            "icons/radix/caret-down.svg"
                        } else {
                            "icons/radix/caret-up.svg"
                        })
                        .w_3p5()
                        .h_3p5()
                        .fill(theme.middle.variant.default.foreground),
                ),
            )
    }

    fn list_item(
        &self,
        avatar_uri: impl Into<ArcCow<'static, str>>,
        label: impl IntoAnyElement<Self>,
        theme: &Theme,
    ) -> impl Element<State = Self> {
        div()
            .h_7()
            .px_2()
            .flex()
            .items_center()
            // .hover()
            // .fill(theme.lowest.variant.hovered.background)
            // .active()
            // .fill(theme.lowest.variant.pressed.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_sm()
                    .child(
                        img()
                            .uri(avatar_uri)
                            .size_3p5()
                            .rounded_full()
                            .fill(theme.middle.positive.default.foreground),
                    )
                    .child(label),
            )
    }
}
