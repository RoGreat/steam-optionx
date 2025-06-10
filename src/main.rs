use slint::{Model, ModelExt, ModelRc, SharedString, StandardListViewItem, VecModel};
use std::rc::Rc;
use webbrowser;

slint::slint! {
    import { VerticalBox, ListView, HorizontalBox, LineEdit, Button, StyleMetrics } from "std-widgets.slint";

    export global HyperlinkClicked {
        pure callback link-clicked(string);
    }

    export component Hyperlink inherits Text {
        in property<string> url;

        area := TouchArea {
            clicked => { HyperlinkClicked.link-clicked(root.url) }
        }
    }

    export component TableViewPage inherits Window {
        property <int> sort-index: -1;
        property <bool> sort-ascending;
        callback file-dialog;
        callback reload-file;

        HorizontalBox {
            VerticalLayout {
                spacing: StyleMetrics.layout-spacing;

                VerticalLayout {
                    Text {
                        text: @tr("Open 'Steam/userdata/XXXXXXXX/config/localconfig.vdf':");
                    }

                    GridLayout {
                        HorizontalLayout {
                            spacing: StyleMetrics.layout-spacing;

                            file_path := LineEdit {
                                accepted => { root.reload-file() }
                            }

                            Button {
                                text: "Open file...";
                                clicked => { root.file-dialog() }
                            }
                        }
                    }
                }

                VerticalLayout {
                    Text {
                        text: @tr("Filter by game:");
                    }

                    filter-edit := LineEdit {
                        placeholder-text: @tr("Enter game...");
                    }

                    Hyperlink {
                        text: @tr("Game");
                        url: @tr("https://example.com");
                    }
                }
            }
        }
    }
}

fn main() {
    let app = TableViewPage::new().unwrap();

    app.global::<HyperlinkClicked>().on_link_clicked(|url| {
        let _ = webbrowser::open(url.as_str());
    });

    app.run().unwrap();
}
