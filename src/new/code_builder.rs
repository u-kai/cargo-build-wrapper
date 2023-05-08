use super::{
    fn_builder::{FnBuilder, MainBuilder},
    struct_builder::StructBuilder,
};

#[derive(Debug)]
pub struct MainRsBuilder {
    depends: Vec<String>,
    inner: MainBuilder,
    fn_builders: Vec<FnBuilder>,
    struct_builders: Vec<StructBuilder>,
}

impl MainRsBuilder {
    pub fn new() -> Self {
        Self {
            depends: Vec::new(),
            fn_builders: Vec::new(),
            inner: MainBuilder::new(),
            struct_builders: Vec::new(),
        }
    }
    pub fn main_return(self, retu: impl Into<String>) -> Self {
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner.retu(retu),
            struct_builders: self.struct_builders,
        }
    }
    pub fn main_attr(self, attr: impl Into<String>) -> Self {
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner.attr(attr),
            struct_builders: self.struct_builders,
        }
    }
    pub fn async_main_mode(self) -> Self {
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner.async_mode(),
            struct_builders: self.struct_builders,
        }
    }
    pub fn add_main_line(self, line: &str) -> Self {
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner.add_line(line),
            struct_builders: self.struct_builders,
        }
    }
    pub fn add_main_inner_comment(self, comment: &str) -> Self {
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner.add_inner_comment(comment),
            struct_builders: self.struct_builders,
        }
    }
    pub fn add_main_outer_comment(self, comment: &str) -> Self {
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner.add_outer_comment(comment),
            struct_builders: self.struct_builders,
        }
    }
    pub fn add_fn_builder(mut self, fn_builder: FnBuilder) -> Self {
        self.fn_builders.push(fn_builder);
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner,
            struct_builders: self.struct_builders,
        }
    }
    pub fn add_depend(mut self, depend: &str) -> Self {
        self.depends.push(depend.into());
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner,
            struct_builders: self.struct_builders,
        }
    }
    pub fn add_struct_builder(mut self, struct_builder: StructBuilder) -> Self {
        self.struct_builders.push(struct_builder);
        Self {
            depends: self.depends,
            fn_builders: self.fn_builders,
            inner: self.inner,
            struct_builders: self.struct_builders,
        }
    }
    pub fn build(self) -> String {
        let depend = self.depends.join("\n");
        let fns = self
            .fn_builders
            .into_iter()
            .map(|fn_builder| fn_builder.build())
            .collect::<Vec<String>>()
            .join("\n");

        let structs = self
            .struct_builders
            .into_iter()
            .map(|fn_builder| fn_builder.build())
            .collect::<Vec<String>>()
            .join("\n");
        let mut result = self.inner.build();
        if depend.len() > 0 {
            result = format!("{}\n\n{}", depend, result)
        }
        if structs.len() > 0 {
            result = format!("{}\n\n{}", result, structs)
        }
        if fns.len() > 0 {
            result = format!("{}\n\n{}", result, fns)
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use crate::new::{fn_builder::FnBuilder, struct_builder::StructBuilder};

    use super::*;
    #[test]
    fn mainファイルをactix_webのプロジェクトにする() {
        let hello_fn = FnBuilder::new("hello")
            .async_mode()
            .attr(r#"actix_web::get("/")"#)
            .add_line(r#"HttpResponse::Ok().body("Hello world!")"#)
            .retu("impl Responder");
        let result = MainRsBuilder::new()
            .async_main_mode()
            .main_attr("actix_web::main")
            .add_depend("use actix_web::{App, HttpResponse, HttpServer, Responder};")
            .main_return("std::io::Result<()>")
            .add_main_line(r#"HttpServer::new(|| App::new().service(hello))"#)
            .add_main_line(r#"  .bind(("127.0.0.1",8080))?"#)
            .add_main_line(r#"  .run()"#)
            .add_main_line(r#"  .await"#)
            .add_fn_builder(hello_fn)
            .build();

        assert_eq!(
            result,
            r#"use actix_web::{App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
      .bind(("127.0.0.1",8080))?
      .run()
      .await
}

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}"#
        );
    }
    #[test]
    fn mainファイルのmain関数よりも後方に構造体を追加できその後方に関数を追加できる() {
        let struct_builder = StructBuilder::new("Test").add_field("a", "i32");
        let fn_builder = FnBuilder::new("test").add_line("let a = 1;");
        let result = MainRsBuilder::new()
            .add_struct_builder(struct_builder)
            .add_fn_builder(fn_builder)
            .build();
        assert_eq!(
            result,
            r#"fn main() {
}

struct Test {
    a: i32,
}

fn test() {
    let a = 1;
}"#
        );
    }
    #[test]
    fn mainファイルのmain関数よりも後方に構造体を追加できる() {
        let struct_builder = StructBuilder::new("Test").add_field("a", "i32");
        let result = MainRsBuilder::new()
            .add_struct_builder(struct_builder)
            .build();
        assert_eq!(
            result,
            r#"fn main() {
}

struct Test {
    a: i32,
}"#
        );
    }
    #[test]
    fn mainファイルのmain関数よりも後方に関数を追加できる() {
        let fn_builder = FnBuilder::new("test").add_line("let a = 1;");
        let fn_builder_2 = FnBuilder::new("test2").add_line("let a = 2;");
        let result = MainRsBuilder::new()
            .add_fn_builder(fn_builder)
            .add_fn_builder(fn_builder_2)
            .build();
        assert_eq!(
            result,
            r#"fn main() {
}

fn test() {
    let a = 1;
}
fn test2() {
    let a = 2;
}"#
        );
    }
    #[test]
    fn mainファイルに依存関係を追加できる() {
        let result = MainRsBuilder::new()
            .add_depend("use std::io::Result;")
            .add_depend("use std::io::Result;")
            .build();
        assert_eq!(
            result,
            r#"use std::io::Result;
use std::io::Result;

fn main() {
}"#
        );
    }
    #[test]
    fn main関数の中身を追加できる() {
        let result = MainRsBuilder::new()
            .add_main_line("let a = 1;")
            .add_main_line("let b = 2;")
            .build();
        assert_eq!(
            result,
            r#"fn main() {
    let a = 1;
    let b = 2;
}"#
        );
    }

    #[test]
    fn 空のasync_main関数を作成する() {
        let result = MainRsBuilder::new().async_main_mode().build();
        assert_eq!(result, "async fn main() {\n}");
    }
    #[test]
    fn 空のmain関数を作成する() {
        let result = MainRsBuilder::new().build();
        assert_eq!(result, "fn main() {\n}");
    }
}
