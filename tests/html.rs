use lira::prelude::*;

#[test]
fn test_div() {
    let res = div()
        .class("my-favorite-class")
        .text("Hello, World!")
        .render();

    assert_eq!("<div class=\"my-favorite-class\">Hello, World!</div>", res);
}

#[test]
fn test_html_document() {
    let res = html()
        .lang("en")
        .child(
            head()
                .child(title().text("My HTML Page"))
                .child(link().rel(Rel::Stylesheet).href("/app.css")),
        )
        .child(
            body()
                .id("body")
                .child(div().text("Best Page"))
                .child(img().src("/icon.png")),
        )
        .render();

    assert_eq!(
        "<!DOCTYPE html><html lang=\"en\"><head><title>My HTML Page</title><link rel=\"stylesheet\" href=\"/app.css\" /></head><body id=\"body\"><div>Best Page</div><img src=\"/icon.png\" /></body></html>",
        res
    )
}

#[test]
fn test_self_closing_tags() {
    let res = img()
        .src("https://example.com/image.png")
        .class("img w-full")
        .render();
    assert_eq!(
        "<img src=\"https://example.com/image.png\" class=\"img w-full\" />",
        res
    );
}

#[test]
fn test_flags() {
    let res = div().flag("x-my-cool-flag").spellcheck().render();
    assert_eq!("<div x-my-cool-flag spellcheck></div>", res);
}

#[test]
fn test_data_attributes() {
    let res = div().data("name", "lira").data("LANG", "Rust").render();
    assert_eq!("<div data-name=\"lira\" data-lang=\"Rust\"></div>", res);
}

#[test]
fn test_empty_class_and_id() {
    let res = div().class("").id("").render();
    assert_eq!("<div class=\"\" id=\"\"></div>", res);
}

#[test]
fn test_text_html_injection() {
    let res = div().text("<script>alert('xss')</script>").render();
    assert_eq!(
        "<div>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</div>",
        res
    );
}

#[test]
fn test_text_html_injection_with_raw() {
    let res = div()
        .raw("<script>alert('this time its fine')</script>")
        .render();

    assert_eq!(
        "<div><script>alert('this time its fine')</script></div>",
        res
    );
}

#[test]
fn test_attribute_html_injection() {
    let res = div()
        .attr("title", "\" onmouseover=\"alert('xss')")
        .render();
    assert_eq!(
        "<div title=\"&quot; onmouseover=&quot;alert(&#39;xss&#39;)\"></div>",
        res
    );
}

#[test]
fn test_data_attribute_injection() {
    let res = div().data("evil", "><img src=x onerror=alert(1)>").render();

    assert_eq!(
        "<div data-evil=\"&gt;&lt;img src=x onerror=alert(1)&gt;\"></div>",
        res
    );
}

#[test]
fn test_empty_document() {
    let res = html().render();
    assert_eq!("<!DOCTYPE html><html></html>", res);
}

#[test]
fn test_weird_unicode_in_text() {
    let res = div().text("💩 < 𝛑 & \"quotes\"").render();
    assert_eq!("<div>💩 &lt; 𝛑 &amp; &quot;quotes&quot;</div>", res);
}

#[test]
fn test_data_attribute_case_normalization() {
    let res = div().data("UserName", "Admin").render();
    assert_eq!("<div data-username=\"Admin\"></div>", res);
}

#[test]
fn test_flag_with_unsafe_chars() {
    let res = div().flag("onload=alert(1)").render();
    assert_eq!("<div onload=alert(1)></div>", res);
}

#[test]
fn test_closed_node_appends_correctly() {
    // text() after child() should append inside <div>...</div>
    let res = div().child(span().text("one")).text("two").render();

    assert_eq!("<div><span>one</span>two</div>", res);
}

#[test]
fn test_open_child_close_behavior() {
    let res = div().child(span()).render();
    assert_eq!("<div><span></span></div>", res);
}

#[test]
fn test_attribute_name_lowercase() {
    let res = div().attr("DaTa-NaMe", "Bob").render();
    assert_eq!("<div data-name=\"Bob\"></div>", res);
}

#[test]
fn test_attribute_value_escaping() {
    let res = div().attr("title", "5 > 3 & 2 < 4").render();
    assert_eq!("<div title=\"5 &gt; 3 &amp; 2 &lt; 4\"></div>", res);
}

#[test]
fn test_flag_normalization() {
    let res = div().flag("DaTa-Flag").render();
    assert_eq!("<div data-flag></div>", res);
}

#[test]
fn test_void_node_has_no_closing_tag() {
    let res = img().render();
    assert!(!res.contains("</img>"));
}

#[test]
fn test_escape_unicode_preserved() {
    let mut buf = Vec::with_capacity(128);
    write_escaped(&mut buf, "💡 < λ & > 😊");

    let res = String::from_utf8(buf).expect("Invalid UTF-8");
    assert_eq!("💡 &lt; λ &amp; &gt; 😊", res);
}

#[test]
fn test_weird_attribute_name() {
    let res = div().attr("😎cool", "<bad>").render();
    assert_eq!("<div 😎cool=\"&lt;bad&gt;\"></div>", res);
}

#[test]
fn test_global_boolean_attributes() {
    let res = div()
        .draggable()
        .autofocus()
        .contenteditable()
        .hidden()
        .render();
    assert_eq!(
        "<div draggable autofocus contenteditable hidden></div>",
        res
    );
}

#[test]
fn test_global_value_attributes() {
    let res = div()
        .accesskey("k")
        .class("my-class")
        .id("my-id")
        .lang("en")
        .title("tooltip")
        .tabindex(5)
        .autocapitalize("words")
        .render();

    assert_eq!(
        "<div accesskey=\"k\" class=\"my-class\" id=\"my-id\" lang=\"en\" title=\"tooltip\" tabindex=\"5\" autocapitalize=\"words\"></div>",
        res
    );
}

#[test]
fn test_href_attribute() {
    let res = link().href("style.css").rel(Rel::Stylesheet).render();
    assert_eq!("<link href=\"style.css\" rel=\"stylesheet\" />", res);

    let res = base().href("/").target(Target::Blank).render();
    assert_eq!("<base href=\"/\" target=\"_blank\" />", res);
}

#[test]
fn test_rel_variants() {
    let res = link().rel(Rel::Alternate).render();
    assert_eq!("<link rel=\"alternate\" />", res);
}

#[test]
fn test_src_attribute() {
    let res = img().src("image.png").render();
    assert_eq!("<img src=\"image.png\" />", res);
}

#[test]
fn test_input_types() {
    let res = input().input_type(InputType::Text).render();
    assert_eq!("<input type=\"text\" />", res);

    let res = input().input_type(InputType::Email).render();
    assert_eq!("<input type=\"email\" />", res);
}

#[test]
fn test_target_variants() {
    let res = base().target(Target::Self_).render();
    assert_eq!("<base target=\"_self\" />", res);

    let res = base().target(Target::Top).render();
    assert_eq!("<base target=\"_top\" />", res);
}

#[test]
fn test_form_attributes() {
    let res = form()
        .action("/submit")
        .method(FormMethod::Post)
        .accept_charset("UTF-8")
        .autocomplete(true)
        .render();
    assert_eq!(
        "<form action=\"/submit\" method=\"POST\" accept-charset=\"UTF-8\" autocomplete=\"on\"></form>",
        res
    );
}

#[test]
fn test_input_attributes() {
    let res = input()
        .name("username")
        .value("bob")
        .checked()
        .accept("image/*")
        .capture()
        .autocomplete(false)
        .render();

    assert_eq!(
        "<input name=\"username\" value=\"bob\" checked accept=\"image/*\" capture autocomplete=\"off\" />",
        res
    );
}

#[test]
fn test_button_attributes() {
    let res = button()
        .type_("submit")
        .name("btn")
        .value("Go")
        .disabled()
        .render();

    assert_eq!(
        "<button type=\"submit\" name=\"btn\" value=\"Go\" disabled></button>",
        res
    );
}

#[test]
fn test_textarea_attributes() {
    let res = textarea()
        .name("msg")
        .value("hello")
        .rows(4)
        .cols(20)
        .disabled()
        .autocomplete(true)
        .render();

    assert_eq!(
        "<textarea name=\"msg\" value=\"hello\" rows=\"4\" cols=\"20\" disabled autocomplete=\"on\"></textarea>",
        res
    );
}

#[test]
fn test_select_and_option() {
    let res = select()
        .name("choices")
        .disabled()
        .multiple()
        .autocomplete(false)
        .child(option().value("1").selected().text("One"))
        .child(option().value("2").text("Two"))
        .render();

    assert_eq!(
        "<select name=\"choices\" disabled multiple autocomplete=\"off\"><option value=\"1\" selected>One</option><option value=\"2\">Two</option></select>",
        res
    );
}

#[test]
fn test_details_and_dialog_open() {
    let res = details().open().child(summary().text("Click me")).render();
    assert_eq!("<details open><summary>Click me</summary></details>", res);

    let res = dialog().open().text("Hello").render();
    assert_eq!("<dialog open>Hello</dialog>", res);
}

#[test]
fn test_hr_void_element() {
    let res = hr().render();
    assert_eq!("<hr />", res);
}

#[test]
fn test_javascript_script() {
    let res = script()
        .type_("module")
        .raw("import Something from 'library';\nSomething.doSomething();\n")
        .render();
    assert_eq!(
        "<script type=\"module\">import Something from 'library';\nSomething.doSomething();\n</script>",
        res
    );
}

#[test]
fn test_manually_closing_element() {
    let mut elem = div().class("root").close();

    for i in [1, 2, 3, 4, 5] {
        elem = elem.child(div().text(i.to_string()));
    }

    assert_eq!(
        "<div class=\"root\"><div>1</div><div>2</div><div>3</div><div>4</div><div>5</div></div>",
        elem.render()
    );
}

#[test]
fn test_children_func() {
    let res = div()
        .class("root")
        .children([1, 2, 3, 4, 5], |item| div().text(item.to_string()))
        .render();

    assert_eq!(
        "<div class=\"root\"><div>1</div><div>2</div><div>3</div><div>4</div><div>5</div></div>",
        res
    );
}

#[test]
fn test_map() {
    let res = div()
        .class("root")
        .children([1, 2, 3, 4, 5], |item| {
            div()
                .map(|n| {
                    if item % 2 == 0 {
                        n.class("even")
                    } else {
                        n.class("odd")
                    }
                })
                .text(item.to_string())
        })
        .render();

    assert_eq!(
        "<div class=\"root\"><div class=\"odd\">1</div><div class=\"even\">2</div><div class=\"odd\">3</div><div class=\"even\">4</div><div class=\"odd\">5</div></div>",
        res
    );
}

#[test]
fn test_map_when() {
    let res = div()
        .class("root")
        .children([1, 2, 3, 4, 5], |item| {
            div()
                .map_when(item % 2 == 0, |n| n.class("even"))
                .text(item.to_string())
        })
        .render();

    assert_eq!(
        "<div class=\"root\"><div>1</div><div class=\"even\">2</div><div>3</div><div class=\"even\">4</div><div>5</div></div>",
        res
    );
}
