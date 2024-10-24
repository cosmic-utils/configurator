use std::borrow::Cow;

use cosmic::{
    iced::{alignment, Alignment, Color, Length},
    iced_widget::{pick_list, toggler},
    prelude::CollectionWidget,
    widget::{
        button, column, container, horizontal_space, mouse_area, row,
        segmented_button::Entity,
        settings::section,
        text, text_input,
        tooltip::{tooltip, Position},
    },
    Element,
};

use crate::{
    app::App,
    icon, icon_button,
    message::{AppMsg, ChangeMsg, PageMsg},
    node::{
        data_path::{DataPath, DataPathType},
        Node, NodeArray, NodeBool, NodeContainer, NodeEnum, NodeNumber, NodeObject, NodeString,
        NodeValue,
    },
    page::Page,
};

const SPACING: f32 = 10.;

pub fn view_app(app: &App) -> Element<'_, AppMsg> {
    let entity = app.nav_model.active();

    match app.nav_model.data::<Page>(entity) {
        Some(page) => {
            container(view_page(entity, page).map(move |msg| AppMsg::PageMsg(entity, msg)))
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
        None => text("no page selected").into(),
    }
}

fn view_data_path(data_path: &DataPath) -> Element<'_, PageMsg> {
    let mut elements = Vec::new();

    let get_class = |pos: Option<usize>| {
        if pos == data_path.pos {
            button::ButtonClass::Text
        } else {
            button::ButtonClass::MenuRoot
        }
    };

    elements.push(
        button::text("/".to_string())
            .on_press(PageMsg::SelectDataPath(None))
            .class(get_class(None))
            .into(),
    );

    for (pos, component) in data_path.vec.iter().enumerate() {
        elements.push(
            button::text(format!("{}", component))
                .on_press(PageMsg::SelectDataPath(Some(pos)))
                .class(get_class(Some(pos)))
                .into(),
        );
    }

    row::with_children(elements).into()
}

fn view_page(entity: Entity, page: &Page) -> Element<'_, PageMsg> {
    let data_path = page.data_path.current();

    let node = page.tree.get_at(data_path.iter()).unwrap();

    let content = match &node.node {
        Node::Bool(node_bool) => view_bool(data_path, node, node_bool),
        Node::String(node_string) => view_string(data_path, node, node_string),
        Node::Number(node_number) => view_number(data_path, node, node_number),
        Node::Object(node_object) => view_object(data_path, node, node_object),
        Node::Enum(node_enum) => view_enum(data_path, node, node_enum),
        Node::Value(node_value) => view_value(data_path, node, node_value),
        Node::Null => text("null").into(),
        Node::Array(node_array) => view_array(data_path, node, node_array),
    };

    column()
        .push(view_data_path(&page.data_path))
        .push(content)
        .spacing(10)
        .into()
}

fn no_value_defined_warning_icon<'a, M: 'a>() -> Element<'a, M> {
    tooltip(
        icon!("report24").class(cosmic::theme::Svg::custom(|e| cosmic::widget::svg::Style {
            color: Some(Color::from_rgb(236.0, 194.0, 58.0)),
        })),
        text("You need to define some values that have no default!"),
        Position::Top,
    )
    .into()
}

fn this_will_remove_all_children<'a, M: 'a>() -> Element<'a, M> {
    tooltip(
        icon!("info24"),
        text("This will remove all children"),
        Position::Top,
    )
    .into()
}

fn node_list<'a>(
    name: DataPathType,
    inner_node: &'a NodeContainer,
    data_path: &'a [DataPathType],
) -> Element<'a, PageMsg> {
    fn append_data_path(data_path: &[DataPathType], field: &DataPathType) -> Vec<DataPathType> {
        let mut new_vec = Vec::with_capacity(data_path.len() + 1);
        new_vec.extend_from_slice(data_path);
        new_vec.push(field.clone());
        new_vec
    }

    let name_cloned = name.clone();

    mouse_area(
        row()
            .align_y(Alignment::Center)
            .push(text(format!("{}", name)))
            .push_maybe(
                if inner_node.removable
                    && let DataPathType::Name(name) = &name
                {
                    Some(
                        button::text("edit key")
                            .on_press(PageMsg::DialogRenameKey(data_path.to_vec(), name.clone())),
                    )
                } else {
                    None
                },
            )
            .push(horizontal_space())
            .push_maybe(match &inner_node.node {
                Node::Null => Some(Element::from(text("null"))),
                Node::Bool(node_bool) => Some(
                    toggler(node_bool.value.unwrap_or_default())
                        .on_toggle(move |value| {
                            PageMsg::ChangeMsg(
                                append_data_path(data_path, &name),
                                ChangeMsg::ChangeBool(value),
                            )
                        })
                        .into(),
                ),

                Node::Enum(node_enum) => {
                    #[derive(Eq, Clone)]
                    struct Key<'a> {
                        pub pos: usize,
                        pub value: Cow<'a, str>,
                    }

                    impl PartialEq for Key<'_> {
                        fn eq(&self, other: &Self) -> bool {
                            self.pos == other.pos
                        }
                    }

                    #[allow(clippy::to_string_trait_impl)]
                    impl ToString for Key<'_> {
                        fn to_string(&self) -> String {
                            self.value.to_string()
                        }
                    }

                    Some(
                        row()
                            .push_maybe(node_enum.value.map(|pos| {
                                text(
                                    node_enum.nodes[pos]
                                        .name()
                                        .unwrap_or(Cow::Owned(pos.to_string())),
                                )
                            }))
                            .push(pick_list(
                                node_enum
                                    .nodes
                                    .iter()
                                    .enumerate()
                                    .map(|(pos, node)| Key {
                                        pos,
                                        value: node.name().unwrap_or(Cow::Owned(pos.to_string())),
                                    })
                                    .collect::<Vec<_>>(),
                                node_enum.value.map(|pos| Key {
                                    pos,
                                    value: Cow::Borrowed(""),
                                }),
                                move |key| {
                                    PageMsg::ChangeMsg(
                                        append_data_path(data_path, &name),
                                        ChangeMsg::ChangeEnum(key.pos),
                                    )
                                },
                            ))
                            .align_y(alignment::Vertical::Center)
                            .into(),
                    )
                }

                _ => None,
            })
            .push_maybe(if !inner_node.is_valid() {
                Some(no_value_defined_warning_icon())
            } else {
                None
            })
            .push_maybe(if inner_node.removable {
                Some(icon_button!("close24").on_press(PageMsg::ChangeMsg(
                    data_path.to_vec(),
                    ChangeMsg::Remove(name_cloned.clone()),
                )))
            } else {
                None
            }),
    )
    .on_press(PageMsg::OpenDataPath(name_cloned))
    .into()
}

fn view_object<'a>(
    data_path: &'a [DataPathType],
    node: &'a NodeContainer,
    node_object: &'a NodeObject,
) -> Element<'a, PageMsg> {
    column()
        .push_maybe(
            node.desc
                .as_ref()
                .map(|desc| section().title("Description").add(text(desc))),
        )
        .push(
            section()
                .title("Values")
                .extend(node_object.nodes.iter().map(|(name, inner_node)| {
                    node_list(DataPathType::Name(name.clone()), inner_node, data_path)
                })),
        )
        .push_maybe(node_object.template.as_ref().map(|_| {
            icon_button!("add24").on_press(PageMsg::DialogAddNewNodeToObject(data_path.to_vec()))
        }))
        .push_maybe(node.default.as_ref().map(|default| {
            section().title("Default").add(
                row()
                    .push(horizontal_space())
                    .push(
                        // xxx: the on_press need to be lazy
                        button::text("reset to default").on_press(PageMsg::ChangeMsg(
                            data_path.to_vec(),
                            ChangeMsg::ApplyDefault,
                        )),
                    )
                    .push(this_will_remove_all_children()),
            )
        }))
        .spacing(SPACING)
        .into()
}

fn view_array<'a>(
    data_path: &'a [DataPathType],
    node: &'a NodeContainer,
    node_array: &'a NodeArray,
) -> Element<'a, PageMsg> {
    column()
        .push_maybe(
            node.desc
                .as_ref()
                .map(|desc| section().title("Description").add(text(desc))),
        )
        .push(
            section().title("Values").extend(
                node_array
                    .values
                    .as_ref()
                    .map_or(&[] as &[NodeContainer], |v| v.as_slice())
                    .iter()
                    .enumerate()
                    .map(|(pos, inner_node)| {
                        node_list(DataPathType::Indice(pos), inner_node, data_path)
                    }),
            ),
        )
        .push(icon_button!("add24").on_press(PageMsg::ChangeMsg(
            data_path.to_vec(),
            ChangeMsg::AddNewNodeToArray,
        )))
        .push_maybe(node.default.as_ref().map(|default| {
            section().title("Default").add(
                row()
                    .push(horizontal_space())
                    .push(
                        // xxx: the on_press need to be lazy
                        button::text("reset to default").on_press(PageMsg::ChangeMsg(
                            data_path.to_vec(),
                            ChangeMsg::ApplyDefault,
                        )),
                    )
                    .push(this_will_remove_all_children()),
            )
        }))
        .spacing(SPACING)
        .into()
}

fn view_enum<'a>(
    data_path: &'a [DataPathType],
    node: &'a NodeContainer,
    node_enum: &'a NodeEnum,
) -> Element<'a, PageMsg> {
    column()
        .push_maybe(
            node.desc
                .as_ref()
                .map(|desc| section().title("Description").add(text(desc))),
        )
        .push(
            section()
                .title("Values")
                .extend(node_enum.nodes.iter().enumerate().map(|(pos, inner_node)| {
                    container(cosmic::widget::radio(
                        {
                            let is_active = if let Some(active_pos) = node_enum.value
                                && active_pos == pos
                            {
                                Some(())
                            } else {
                                None
                            };

                            row()
                                .push(text(
                                    inner_node.name().unwrap_or(Cow::Owned(pos.to_string())),
                                ))
                                .push(horizontal_space())
                                .push_maybe(is_active.map(|_| {
                                    button::text("modify")
                                        .on_press(PageMsg::OpenDataPath(DataPathType::Indice(pos)))
                                }))
                                .push_maybe(is_active.and_then(|_| {
                                    if !inner_node.is_valid() {
                                        Some(no_value_defined_warning_icon())
                                    } else {
                                        None
                                    }
                                }))
                                .align_y(Alignment::Center)
                        },
                        pos,
                        node_enum.value,
                        |pos| PageMsg::ChangeMsg(data_path.to_vec(), ChangeMsg::ChangeEnum(pos)),
                    ))
                    .padding(5)
                })),
        )
        .push_maybe(node.default.as_ref().map(|default| {
            section()
                .title("Default")
                .add_maybe(default.clone().into_string().map(|default| {
                    container(
                        row()
                            .push(text("Default value"))
                            .push(horizontal_space())
                            .push(text(default)),
                    )
                    .padding(10)
                }))
                .add(
                    row()
                        .push(horizontal_space())
                        .push(
                            // xxx: the on_press need to be lazy
                            button::text("reset to default").on_press(PageMsg::ChangeMsg(
                                data_path.to_vec(),
                                ChangeMsg::ApplyDefault,
                            )),
                        )
                        .push(this_will_remove_all_children()),
                )
        }))
        .spacing(SPACING)
        .into()
}

fn view_bool<'a>(
    data_path: &'a [DataPathType],
    node: &'a NodeContainer,
    node_bool: &'a NodeBool,
) -> Element<'a, PageMsg> {
    column()
        .push_maybe(
            node.desc
                .as_ref()
                .map(|desc| section().title("Description").add(text(desc))),
        )
        .push(
            section().title("Value").add(
                row()
                    .push(text("Current value"))
                    .push(horizontal_space())
                    .push(
                        toggler(node_bool.value.unwrap_or_default()).on_toggle(move |value| {
                            PageMsg::ChangeMsg(data_path.to_vec(), ChangeMsg::ChangeBool(value))
                        }),
                    )
                    .push_maybe(if node_bool.value.is_none() {
                        Some(no_value_defined_warning_icon())
                    } else {
                        None
                    }),
            ),
        )
        .push_maybe(
            node.default
                .as_ref()
                .and_then(|v| v.to_bool())
                .map(|default| {
                    section()
                        .title("Default")
                        .add(
                            row()
                                .push(text("Default value"))
                                .push(horizontal_space())
                                .push(toggler(default)),
                        )
                        .add(row().push(horizontal_space()).push(
                            // xxx: the on_press need to be lazy
                            button::text("reset to default").on_press(PageMsg::ChangeMsg(
                                data_path.to_vec(),
                                ChangeMsg::ApplyDefault,
                            )),
                        ))
                }),
        )
        .spacing(SPACING)
        .into()
}

fn view_string<'a>(
    data_path: &'a [DataPathType],
    node: &'a NodeContainer,
    node_string: &'a NodeString,
) -> Element<'a, PageMsg> {
    column()
        .push_maybe(
            node.desc
                .as_ref()
                .map(|desc| section().title("Description").add(text(desc))),
        )
        .push(
            section().title("Value").add(
                row()
                    .push(text("Current value"))
                    .push(horizontal_space())
                    .push(
                        text_input("value", node_string.value.as_ref().map_or("", |v| v)).on_input(
                            move |value| {
                                PageMsg::ChangeMsg(
                                    data_path.to_vec(),
                                    ChangeMsg::ChangeString(value),
                                )
                            },
                        ),
                    )
                    .push_maybe(if node_string.value.is_none() {
                        Some(no_value_defined_warning_icon())
                    } else {
                        None
                    }),
            ),
        )
        .push_maybe(
            node.default
                .as_ref()
                .and_then(|v| v.as_str())
                .map(|default| {
                    section()
                        .title("Default")
                        .add(
                            row()
                                .push(text("Default value"))
                                .push(horizontal_space())
                                .push(text(default)),
                        )
                        .add(row().push(horizontal_space()).push(
                            // xxx: the on_press need to be lazy
                            button::text("reset to default").on_press(PageMsg::ChangeMsg(
                                data_path.to_vec(),
                                ChangeMsg::ApplyDefault,
                            )),
                        ))
                }),
        )
        .spacing(SPACING)
        .into()
}

fn view_number<'a>(
    data_path: &'a [DataPathType],
    node: &'a NodeContainer,
    node_number: &'a NodeNumber,
) -> Element<'a, PageMsg> {
    column()
        .push_maybe(
            node.desc
                .as_ref()
                .map(|desc| section().title("Description").add(text(desc))),
        )
        .push(
            section().title("Value").add(
                row()
                    .push(text("Current value"))
                    .push(horizontal_space())
                    .push(
                        text_input("value", &node_number.value_string).on_input(move |value| {
                            PageMsg::ChangeMsg(data_path.to_vec(), ChangeMsg::ChangeNumber(value))
                        }),
                    )
                    .push_maybe(if node_number.value.is_none() {
                        Some(no_value_defined_warning_icon())
                    } else if node_number
                        .try_parse_from_str(&node_number.value_string)
                        .is_err()
                    {
                        Some(
                            tooltip(
                                icon!("report24"),
                                text("This value is incorrect."),
                                Position::Top,
                            )
                            .into(),
                        )
                    } else {
                        None
                    }),
            ),
        )
        .push_maybe(
            node.default
                .as_ref()
                .and_then(|v| v.to_num())
                .and_then(|v| node_number.try_from_figment_num(v).ok())
                .map(|default| {
                    section()
                        .title("Default")
                        .add(
                            row()
                                .push(text("Default value"))
                                .push(horizontal_space())
                                .push(text(default.to_string())),
                        )
                        .add(row().push(horizontal_space()).push(
                            // xxx: the on_press need to be lazy
                            button::text("reset to default").on_press(PageMsg::ChangeMsg(
                                data_path.to_vec(),
                                ChangeMsg::ApplyDefault,
                            )),
                        ))
                }),
        )
        .spacing(SPACING)
        .into()
}

fn view_value<'a>(
    data_path: &'a [DataPathType],
    node: &'a NodeContainer,
    node_value: &'a NodeValue,
) -> Element<'a, PageMsg> {
    column()
        .push_maybe(
            node.desc
                .as_ref()
                .map(|desc| section().title("Description").add(text(desc))),
        )
        .push(
            section()
                .title("Value")
                .add(text(format!("{:?}", node_value.value))),
        )
        .spacing(SPACING)
        .into()
}
