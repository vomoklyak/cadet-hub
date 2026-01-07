use crate::view::auth_layout::AuthLayout;
use crate::view::cadet_course_list_view::CadetCourseListView;
use crate::view::cadet_list_view::CadetListView;
use crate::view::header_layout::HeaderLayout;
use crate::view::home_view::Home;
use crate::view::login_view::LoginView;
use crate::view::user_list_view::UserListView;
use dioxus::prelude::*;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Debug)]
pub(crate) enum Route {
    #[layout(AuthLayout)]
        #[layout(HeaderLayout)]
            #[route("/")]
            Home {},
            #[route("/cadets")]
            CadetListView {},
            #[route("/cadet-courses")]
            CadetCourseListView {},
            #[route("/users")]
            UserListView {},
        #[end_layout]
    #[end_layout]

    #[route("/logins")]
    LoginView {},
}
