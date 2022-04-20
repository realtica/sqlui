use fltk::{draw::measure, prelude::*, *};
use fltk_table::{SmartTable, TableOpts};
use fltk_theme::{widget_themes, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use std::cell::RefCell;
use std::rc::Rc;
mod mysql_manager;
use self::mysql_manager::MySqlManager;
use log::{debug, info, trace, warn, LevelFilter};
use mysql::*;
extern crate once_cell;
use once_cell::unsync::{Lazy, OnceCell};
use serde_derive::Deserialize;
use std::fs;
#[derive(Debug, Deserialize, Clone)]
struct Config {
    endpoints: Vec<Endpoint>,
}
#[derive(Debug, Deserialize, Clone)]
struct Endpoint {
    connection_string: String,
    name: String,
}
#[tokio::main]
async fn main() -> Result<()> {
    // simple_logging::log_to_file("test.log", LevelFilter::Info);
    // let connection_string = "mysql://root:@k3y3d-1n@@localhost:3316/keke6196_keyed_in_pe";
    let toml_str: String = fs::read_to_string("endpoints.toml")?.parse()?;
    println!("{}", toml_str);
    let config: Config = toml::from_str(&toml_str).unwrap();
    let config = Rc::new(RefCell::new(config));
    // info!("{}", urls);
    println!("{:#?}", config);
    let db_manager: MySqlManager = Default::default();
    let db_manager = Rc::new(RefCell::new(db_manager));
    let app = app::App::default();
    app::set_font_size(18);
    let widget_theme = WidgetTheme::new(ThemeType::Metro);
    widget_theme.apply();
    // let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    // widget_scheme.apply();
    let mut wind = window::Window::default().with_size(800, 600);
    let mut main_flex = group::Flex::default().size_of_parent();
    main_flex.set_type(group::FlexType::Row);
    let mut conn_flex = group::Flex::default().size_of_parent();
    conn_flex.set_type(group::FlexType::Column);
    let mut db_name = frame::Frame::default().with_label("Select a database");
    conn_flex.set_size(&mut db_name, 30);
    let mut choice_endpoint = menu::Choice::default();
    // choice_endpoint.add_choice("Select the Endpoint to connect");
    // choice_endpoint.set_value(0);
    for e in &config.borrow_mut().endpoints {
        println!("{}", e.name);
        choice_endpoint.add_choice(e.name.as_str());
    }
    conn_flex.set_size(&mut choice_endpoint, 30);
    // let mut btn = button::Button::default().with_label("Connect");
    // conn_flex.set_size(&mut btn, 30);
    let mut hold_browser = browser::HoldBrowser::default();
    hold_browser.set_column_widths(&[280, 20]);
    let hold_browser = Rc::new(RefCell::new(hold_browser));
    conn_flex.end();
    main_flex.set_size(&mut conn_flex, 350);
    let mut detail_flex = group::Flex::default().size_of_parent();
    detail_flex.set_type(group::FlexType::Column);
    let mut controls_flex = group::Flex::default().size_of_parent();
    controls_flex.set_type(group::FlexType::Row);
    controls_flex.set_margin(5);
    let mut btn_prev = button::Button::default().with_label("@#<-");
    // btn_prev.set_label_type(enums::LabelType::Icon);
    btn_prev.set_label_size(32);
    btn_prev.deactivate();
    controls_flex.set_size(&mut btn_prev, 50);
    let btn_prev = Rc::new(RefCell::new(btn_prev));
    let mut btn_next = button::Button::default().with_label("@#->");
    btn_next.set_label_size(32);
    btn_next.deactivate();
    controls_flex.set_size(&mut btn_next, 50);
    let btn_next = Rc::new(RefCell::new(btn_next));
    let mut page_label = frame::Frame::default().with_label("page");
    controls_flex.set_size(&mut page_label, 50);
    let page_label = Rc::new(RefCell::new(page_label));
    let mut filter = misc::InputChoice::default();
    filter.input().set_label_size(10);
    controls_flex.set_size(&mut filter, 500);
    let filter = Rc::new(RefCell::new(filter));
    let mut btn_filter = button::Button::default().with_label("@search");
    btn_filter.set_label_size(32);
    controls_flex.set_size(&mut btn_filter, 50);
    let btn_filter = Rc::new(RefCell::new(btn_filter));
    let mut btn_clear = button::Button::default().with_label("X");
    btn_clear.set_label_size(32);
    controls_flex.set_size(&mut btn_clear, 50);
    controls_flex.end();
    let table = SmartTable::default_fill().with_opts(TableOpts {
        rows: 0,
        cols: 0,
        editable: false,
        ..Default::default()
    });
    let table = Rc::new(RefCell::new(table));
    detail_flex.set_size(&mut controls_flex, 50);
    detail_flex.end();
    main_flex.end();
    wind.make_resizable(true);
    wind.end();
    wind.show();
    choice_endpoint.set_callback({
        let hold_browser = hold_browser.clone();
        let table = table.clone();
        let db_manager = db_manager.clone();
        move |c| {
            let connection_string =
                &config.borrow_mut().endpoints[(c.value()) as usize].connection_string;
            println!("conn={:#?}", connection_string);
            hold_browser.borrow_mut().clear();
            table.borrow_mut().clear();
            let split_iterator = connection_string.split("/");
            let split: Vec<&str> = split_iterator.collect();
            db_manager.borrow_mut().db_name = split[split.len() - 1].to_string();
            // db_manager.borrow_mut().db_name = String::from("keke6196_keyed_in_pe");
            if *connection_string != db_manager.borrow_mut().connection_string {
                println!("Changing Endpoint..");
                let pool_cell: OnceCell<Pool> = OnceCell::new();
                db_manager.borrow_mut().pool = pool_cell;
            }
            db_manager.borrow_mut().connection_string = String::from(connection_string);
            let tables: Vec<String> = db_manager.borrow_mut().get_tables_from_db();
            for table_name in tables {
                hold_browser
                    .borrow_mut()
                    .add(format!("{}\t", table_name).as_str());
            }
        }
    });
    hold_browser.borrow_mut().set_callback({
        let table = table.clone();
        let db_manager = db_manager.clone();
        let btn_prev = btn_prev.clone();
        let btn_next = btn_next.clone();
        let page_label = page_label.clone();
        let filter = filter.clone();
        move |h| {
            if h.value() != 0 {
                filter.borrow_mut().set_value("");
                db_manager.borrow_mut().select_table(
                    h.selected_text()
                        .unwrap()
                        .split("\t")
                        .next()
                        .unwrap()
                        .to_string(),
                );
                navigate(table.clone(), db_manager.clone());
                let number_of_pages = db_manager.borrow_mut().number_of_pages;
                let page = db_manager.borrow_mut().page;
                check_controls(
                    page_label.clone(),
                    btn_prev.clone(),
                    btn_next.clone(),
                    number_of_pages,
                    page,
                );
                let number_of_rows = db_manager.borrow_mut().number_of_rows;
                let table_name = &db_manager.borrow_mut().table_selected;
                h.set_text(
                    h.value(),
                    format!("{}\t@r@s{}", table_name, number_of_rows).as_str(),
                );
            }
        }
    });
    btn_next.borrow_mut().set_callback({
        let db_manager = db_manager.clone();
        let table = table.clone();
        let btn_prev = btn_prev.clone();
        let btn_next = btn_next.clone();
        let page_label = page_label.clone();
        move |_| {
            println!("Button Next pressed!");
            db_manager.borrow_mut().next();
            navigate(table.clone(), db_manager.clone());
            let number_of_pages = db_manager.borrow_mut().number_of_pages;
            let page = db_manager.borrow_mut().page;
            check_controls(
                page_label.clone(),
                btn_prev.clone(),
                btn_next.clone(),
                number_of_pages,
                page,
            );
        }
    });
    btn_prev.borrow_mut().set_callback({
        let db_manager = db_manager.clone();
        let table = table.clone();
        let btn_prev = btn_prev.clone();
        let btn_next = btn_next.clone();
        let page_label = page_label.clone();
        move |_| {
            println!("Button Prev pressed!");
            db_manager.borrow_mut().prev();
            navigate(table.clone(), db_manager.clone());
            let number_of_pages = db_manager.borrow_mut().number_of_pages;
            let page = db_manager.borrow_mut().page;
            check_controls(
                page_label.clone(),
                btn_prev.clone(),
                btn_next.clone(),
                number_of_pages,
                page,
            );
        }
    });
    btn_filter.borrow_mut().set_callback({
        let table = table.clone();
        let db_manager = db_manager.clone();
        let btn_prev = btn_prev.clone();
        let btn_next = btn_next.clone();
        let page_label = page_label.clone();
        let hold_browser = hold_browser.clone();
        let filter = filter.clone();
        move |_| {
            if hold_browser.borrow_mut().value() != 0 {
                db_manager.borrow_mut().filter = filter.borrow_mut().value().clone().unwrap();
                db_manager.borrow_mut().init_values();
                navigate(table.clone(), db_manager.clone());
                let number_of_pages = db_manager.borrow_mut().number_of_pages;
                let page = db_manager.borrow_mut().page;
                check_controls(
                    page_label.clone(),
                    btn_prev.clone(),
                    btn_next.clone(),
                    number_of_pages,
                    page,
                );
            }
        }
    });
    btn_clear.set_callback({
        let hold_browser = hold_browser.clone();
        let filter = filter.clone();
        move |_| {
            hold_browser.borrow_mut().do_callback();
            filter.borrow_mut().set_value("");
        }
    });
    // TODO:  make a fuzzy search for filter's items
   // filter.borrow_mut().set_callback({
    //     move |f| {
    //         if f.menu_button().changed() {
    //             println!("picked..");
    //         } else {
    //             println!("typed..");
    //         }
    //     }
    // });
    wind.set_callback(move |_| {
        if app::event() == enums::Event::Close {
            app.quit();
        }
    });
    app.run().unwrap();
    Ok(())
}

fn check_controls(
    page_label: Rc<RefCell<frame::Frame>>,
    btn_prev: Rc<RefCell<button::Button>>,
    btn_next: Rc<RefCell<button::Button>>,
    number_of_pages: u32,
    page: u32,
) {
    let label = format!("{}/{}", page, number_of_pages);
    page_label.borrow_mut().set_label(&label);
    if page < number_of_pages {
        btn_next.borrow_mut().activate();
    } else {
        btn_next.borrow_mut().deactivate();
    }
    if page > 1 {
        btn_prev.borrow_mut().activate();
    } else {
        btn_prev.borrow_mut().deactivate();
    }
}

fn navigate(table: Rc<RefCell<SmartTable>>, db_manager: Rc<RefCell<MySqlManager>>) {
    let (columns, size_types) = db_manager.borrow_mut().get_columns_from_table();
    let rows = db_manager.borrow_mut().get_rows_from_table();
    // db_name.set_label(h.selected_text().unwrap().as_str());
    table.borrow_mut().set_opts(TableOpts {
        cell_align: enums::Align::Left,
        editable: false,
        rows: rows.len() as i32,
        cols: columns.len() as i32,
        cell_padding: 5,
        ..Default::default()
    });
    for (i, col) in columns.iter().enumerate() {
        table.borrow_mut().set_col_header_value(i as i32, col);
        let (text_width, _) = measure(col, true);
        let col_size: i32;
        if size_types[i] > text_width {
            col_size = size_types[i];
        } else {
            col_size = text_width;
        }
        table.borrow_mut().set_col_width(i as i32, col_size);
    }
    for (i, row) in rows.iter().enumerate() {
        for (j, column) in row.columns_ref().iter().enumerate() {
            let column_value = &row[column.name_str().as_ref()];
            let val = match from_value_opt::<String>(column_value.clone()) {
                Ok(s) => s,
                Err(FromValueError(_v)) => String::from("null"),
            };
            table.borrow_mut().set_cell_value(i as i32, j as i32, &val);
        }
    }
    table.borrow_mut().redraw();
}
