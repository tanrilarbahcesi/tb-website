fn main() {
    let layout = std::fs::read_to_string("pages/layout.html").unwrap();
    for file in std::fs::read_dir("pages/content").unwrap() {
        let content = std::fs::read_to_string(file.as_ref().unwrap().path().to_str().unwrap()).unwrap();
        let page = content_reader(content);
        let result_page = layout
            .replace("{{title}}", &page.title)
            .replace("{{description}}", &page.description)
            .replace("{{keywords}}", &page.keywords)
            .replace("{{content}}", &page.content);
        std::fs::write(format!("release/{}", file.unwrap().path().file_name().unwrap().to_str().unwrap()), result_page).unwrap();
    }
    println!("Done!");
}

struct Page {
    title: String,
    description: String,
    keywords: String,
    content: String
}

fn content_reader(content: String) -> Page {
    Page {
        title: content.lines().next().unwrap().to_string(),
        description: content.lines().skip(1).next().unwrap().to_string(),
        keywords: content.lines().skip(2).next().unwrap().to_string(),
        content: content.lines().skip(3).collect()
    }
}