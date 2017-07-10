// Tests for HTML spec.

extern crate pulldown_cmark;

#[test]
fn html_test_1() {
    let original = r##"Little header

<script type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<p>Little header</p>
<script type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_2() {
    let original = r##"Little header

<script
type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<p>Little header</p>
<script
type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_3() {
    let original = r##"Little header

<?
<div></div>
<p>Useless</p>
?>"##;
    let expected = r##"<p>Little header</p>
<?
<div></div>
<p>Useless</p>
?>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_4() {
    let original = r##"Little header

<!--
<div></div>
<p>Useless</p>
-->"##;
    let expected = r##"<p>Little header</p>
<!--
<div></div>
<p>Useless</p>
-->"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_5() {
    let original = r##"Little header

<![CDATA[
<div></div>
<p>Useless</p>
]]>"##;
    let expected = r##"<p>Little header</p>
<![CDATA[
<div></div>
<p>Useless</p>
]]>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_6() {
    let original = r##"Little header

<!X
Some things are here...
>"##;
    let expected = r##"<p>Little header</p>
<!X
Some things are here...
>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_7() {
    let original = r##"Little header
-----------

<script>
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<h2>Little header</h2>
<script>
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    use pulldown_cmark::{Parser, html};

    let mut s = String::new();

    let p = Parser::new(&original);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_9() {
    let original = r##"/u/spez /r/pics reddit@reddit.com www.reddit.com ~~strikethrough~~ ~underlined~"##;

    let expected = r##"<p><a href="/u/spez" />/u/spez</a> <a href="/r/pics" />/r/pics</a> reddit@reddit.com www.reddit.com <del>strikethrough</del> <u>underlined</u></p>
"##;

    use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

    let mut s = String::new();

    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    opts.insert(OPTION_ENABLE_FOOTNOTES);

    let p = Parser::new_ext(&original, opts);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_10() {
    let original = r##"~~strikethrough~~"##;

    let expected = r##"<p><del>strikethrough</del></p>
"##;

    use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

    let mut s = String::new();

    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    opts.insert(OPTION_ENABLE_FOOTNOTES);

    let p = Parser::new_ext(&original, opts);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

#[test]
fn html_test_11() {
    let original = r##"~underlined~"##;

    let expected = r##"<p><u>underlined</u></p>
"##;

    use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

    let mut s = String::new();

    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    opts.insert(OPTION_ENABLE_FOOTNOTES);

    let p = Parser::new_ext(&original, opts);
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}
