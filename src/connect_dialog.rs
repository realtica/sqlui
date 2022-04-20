pub struct ConnectDialog {
    user: String,
    password: String,
    host: String,
    port: String,
    db_name: String,
}
//
impl ConnectDialog {
    pub fn default() -> Self {
        let mut win = window::Window::default()
            .with_size(400, 320)
            .center_of_parent()
            .with_label("Connection")
            .with_align(enums::Align::Center | enums::Align::Inside);
        let mut grp = group::Group::default()
            .with_size(350, 230)
            .center_of_parent()
            .with_label("Fill the fields to connect");
        // .with_align(enums::Align::Top | enums::Align::Left);

        let mut main_flex = group::Flex::default()
            .with_size(310, 210)
            .center_of_parent();
        // main_flex.set_frame(enums::FrameType::BorderBox);
        main_flex.set_type(group::FlexType::Column);
        let mut urow = group::Flex::default().row();
        let uframe = frame::Frame::default()
            .with_label("Username:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        urow.set_size(&uframe, 120);
        let username = input::Input::default();

        urow.set_size(&username, 180);
        urow.end();
        let mut prow = group::Flex::default().row();
        let pframe = frame::Frame::default()
            .with_label("Password:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        prow.set_size(&pframe, 120);
        let password = input::SecretInput::default();

        prow.set_size(&password, 180);
        prow.end();
        let mut hrow = group::Flex::default().row();
        let hframe = frame::Frame::default()
            .with_label("Host:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        hrow.set_size(&hframe, 120);
        let host = input::Input::default();

        hrow.set_size(&host, 180);
        hrow.end();
        let mut prow = group::Flex::default().row();
        // prow.set_color(enums::Color::Blue);
        // prow.set_frame(enums::FrameType::BorderBox);
        let pframe = frame::Frame::default()
            .with_label("Port:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        prow.set_size(&pframe, 120);
        let port = input::Input::default();
        prow.set_size(&port, 80);
        prow.end();
        let mut drow = group::Flex::default().row();
        let dframe = frame::Frame::default()
            .with_label("Database:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        drow.set_size(&dframe, 120);
        let db_name = input::Input::default();
        drow.set_size(&db_name, 180);
        drow.end();
        main_flex.end();
        grp.end();
        grp.set_frame(enums::FrameType::EngravedFrame);
        let mut ok = button::Button::default()
            .with_label("Connect")
            .with_size(80, 30)
            .below_of(&grp, 5)
            .center_x(&grp);
        win.end();
        win.make_modal(true);
        win.show();
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            app::wait();
        }
        Self {
            user: username.value(),
            password: password.value(),
            host: host.value(),
            port: port.value(),
            db_name: db_name.value(),
        }
        // Self { connectForm.get_props() }
    }
}
// pub fn value(&self) -> String {
//     self.inp.value()
// }
