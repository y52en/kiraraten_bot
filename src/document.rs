use scraper;

#[allow(dead_code)]
fn p<S:std::fmt::Debug>(s:S){
    println!("{:#?}",s);
}

#[derive(Debug)]
#[allow(non_camel_case_types,dead_code)]
pub enum Get {
    id,
    inner_html,
    outer_html,
    class
}

pub fn scraping<S: AsRef<str>>(html: &scraper::Html, sel: S, get: Get) -> Vec<String> {
    let sel = sel.as_ref();
    let selector = compile_selector(sel);
    let dom_ls = html.select(&selector);


    let mut output = Vec::new();
    for elm in dom_ls {
        let st = match &get {
            Get::id => elm.value().id().unwrap().to_string(),
            Get::inner_html => elm.inner_html(),
            Get::outer_html => elm.outer_html(),
            Get::class => elm.value().classes().next().unwrap().to_string(),
        };
        output.push(st);
    }
    output
}

#[inline(always)]
fn compile_selector(selector: &str) -> scraper::Selector {
    scraper::Selector::parse(&selector).unwrap()
}

pub fn parse_html<S: AsRef<str>>(html: S) -> scraper::Html {
    let html = html.as_ref();
    scraper::Html::parse_document(html)
}

trait Elm {
    fn outer_html(self) -> String;
}

impl Elm for scraper::ElementRef<'_> {
    fn outer_html(self) -> String {
        return self.html();
    }
}
