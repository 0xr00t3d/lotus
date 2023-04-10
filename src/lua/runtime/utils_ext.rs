use crate::{
    lua::{
        model::LuaRunTime,
        parsing::{
            html::{css_selector, html_parse, html_search},
            text::ResponseMatcher,
        },
        threads::{LuaThreader, ParamScan},
    },
    BAR,
};
use std::sync::{Arc, Mutex};

pub trait UtilsEXT {
    fn add_threadsfunc(&self);
    fn add_matchingfunc(&self);
    fn add_printfunc(&self);
}

impl UtilsEXT for LuaRunTime<'_> {
    fn add_printfunc(&self) {
        self.lua
            .globals()
            .set(
                "println",
                self.lua
                    .create_function(move |_, msg: String| {
                        {
                            BAR.lock().unwrap().println(&msg)
                        };
                        Ok(())
                    })
                    .unwrap(),
            )
            .unwrap();
    }
    fn add_matchingfunc(&self) {
        self.lua
            .globals()
            .set(
                "generate_css_selector",
                self.lua
                    .create_function(|_, payload: String| Ok(css_selector(&payload)))
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "html_parse",
                self.lua
                    .create_function(|_, (html, payload): (String, String)| {
                        Ok(html_parse(&html, &payload))
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "html_search",
                self.lua
                    .create_function(|_, (html, pattern): (String, String)| {
                        Ok(html_search(&html, &pattern))
                    })
                    .unwrap(),
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "Matcher",
                ResponseMatcher {
                    ignore_whitespace: false,
                    case_insensitive: false,
                    multi_line: false,
                    octal: true,
                    unicode: true,
                    dot_matches_new_line: false,
                },
            )
            .unwrap();

        self.lua
            .globals()
            .set(
                "str_startswith",
                self.lua
                    .create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.starts_with(&str_two))
                    })
                    .unwrap(),
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "str_contains",
                self.lua
                    .create_function(|_, (str_one, str_two): (String, String)| {
                        Ok(str_one.contains(&str_two))
                    })
                    .unwrap(),
            )
            .unwrap();
    }

    fn add_threadsfunc(&self) {
        // ProgressBar
        self.lua
            .globals()
            .set(
                "ParamScan",
                ParamScan {
                    finds: Arc::new(Mutex::new(false)),
                    accept_nil: Arc::new(Mutex::new(false)),
                },
            )
            .unwrap();
        self.lua
            .globals()
            .set(
                "LuaThreader",
                LuaThreader {
                    stop: Arc::new(Mutex::new(false)),
                },
            )
            .unwrap();
    }
}
