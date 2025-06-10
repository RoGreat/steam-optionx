use slint::{Model, ModelExt, ModelRc, SharedString, StandardListViewItem, VecModel};
use std::rc::Rc;

slint::slint! {
    import { Button, HorizontalBox, VerticalBox, StandardTableView, GroupBox, StyleMetrics, LineEdit, GridBox } from "std-widgets.slint";

    export global TableViewPageAdapter  {
        in property <[[StandardListViewItem]]> row_data: [
            [ { text: "Item 1.1" }, { text: "Item 1.2" }, { text: "Item 1.3" }, { text: "Item 1.4" }, ],
            [ { text: "Item 2.1" }, { text: "Item 2.2" }, { text: "Item 2.3" }, { text: "Item 2.4" }, ],
            [ { text: "Item 3.1" }, { text: "Item 3.2" }, { text: "Item 3.3" }, { text: "Item 3.4" }, ],
            [ { text: "Item 4.1" }, { text: "Item 4.2" }, { text: "Item 4.3" }, { text: "Item 4.4" }, ],
            [ { text: "Item 5.1" }, { text: "Item 5.2" }, { text: "Item 5.3" }, { text: "Item 5.4" }, ],
            [ { text: "Item 6.1" }, { text: "Item 6.2" }, { text: "Item 6.3" }, { text: "Item 6.4" }, ],
        ];

        pure callback filter_sort_model([[StandardListViewItem]], string, int, bool) -> [[StandardListViewItem]];
        filter_sort_model(row-data, filter, sort-index, sort-ascending) => { return row-data; }
    }

    export component TableViewPage inherits Window {
        property <int> sort-index: -1;
        property <bool> sort-ascending;
        callback file-dialog;
        callback reload-file;

        HorizontalBox {
            vertical-stretch: 1;

            GroupBox {
                vertical-stretch: 0;

                VerticalLayout {
                    spacing: StyleMetrics.layout-spacing;

                    VerticalLayout {
                        Text {
                            text: @tr("Open 'Steam/userdata/XXXXXXXX/config/localconfig.vdf':");
                        }

                        GridLayout {
                            HorizontalLayout {
                                spacing: 5px;

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
                    }

                    StandardTableView {
                        sort-ascending(index) => {
                            root.sort-index = index;
                            root.sort-ascending = true;
                        }

                        sort-descending(index) => {
                            root.sort-index = index;
                            root.sort-ascending = false;
                        }

                        columns: [
                            { title: @tr("Game") },
                            { title: @tr("Launch Option") },
                            { title: @tr("Header 3") },
                            { title: @tr("Header 4") },
                        ];
                        rows: TableViewPageAdapter.filter_sort_model(TableViewPageAdapter.row_data, filter-edit.text, root.sort-index, root.sort-ascending);
                    }
                }
           }
        }
    }
}

fn main() {
    let app = TableViewPage::new().unwrap();

    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

    for r in 1..101 {
        let items = Rc::new(VecModel::default());

        for c in 1..5 {
            items.push(slint::format!("Item {r}.{c}").into());
        }

        row_data.push(items.into());
    }

    app.global::<TableViewPageAdapter>()
        .set_row_data(row_data.clone().into());
    app.global::<TableViewPageAdapter>()
        .on_filter_sort_model(filter_sort_model);

    app.run().unwrap();
}

fn filter_sort_model(
    source_model: ModelRc<ModelRc<StandardListViewItem>>,
    filter: SharedString,
    sort_index: i32,
    sort_ascending: bool,
) -> ModelRc<ModelRc<StandardListViewItem>> {
    let mut model = source_model.clone();

    if !filter.is_empty() {
        let filter = filter.to_lowercase();

        // filter by first row
        model = Rc::new(source_model.clone().filter(move |e| {
            e.row_data(0)
                .unwrap()
                .text
                .to_lowercase()
                .contains(filter.as_str())
        }))
        .into();
    }

    if sort_index >= 0 {
        model = Rc::new(model.clone().sort_by(move |r_a, r_b| {
            let c_a = r_a.row_data(sort_index as usize).unwrap();
            let c_b = r_b.row_data(sort_index as usize).unwrap();

            if sort_ascending {
                c_a.text.cmp(&c_b.text)
            } else {
                c_b.text.cmp(&c_a.text)
            }
        }))
        .into();
    }
    model
}

fn reload_file() {}

fn file_dialog() {}
