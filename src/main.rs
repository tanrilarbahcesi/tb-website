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
        std::fs::write(format!("release/{}", file.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap()), result_page).unwrap();        
    }
    generate_sitemap();
    println!("Done!");
}

struct Page {
    title: String,
    description: String,
    keywords: String,
    content: String
}

fn generate_sitemap() {
    let lastmod = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00").to_string();
    let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">"#);
    for file in std::fs::read_dir("pages/content").unwrap() {
        let filepath = file.unwrap().path();
        let filename = filepath.file_name().unwrap().to_str().unwrap();
        let priority = if filename == "index.html" { "1.0" } else { "0.8" };
        sitemap.push_str(format!("
        <url>
            <loc>https://tanrilarbahcesi.com/{}</loc>
            <lastmod>{}</lastmod>
            <priority>{}</priority>
        </url>
        ", filename, lastmod, priority).as_str());
    }
    sitemap.push_str(r#"
    </urlset>"#);
    std::fs::write(format!("release/sitemap.xml"), sitemap).unwrap();
}

fn content_reader(content: String) -> Page {
    Page {
        title: content.lines().next().unwrap().to_string(),
        description: content.lines().skip(1).next().unwrap().to_string(),
        keywords: content.lines().skip(2).next().unwrap().to_string(),
        content: content.lines().skip(3).collect()
    }
}
