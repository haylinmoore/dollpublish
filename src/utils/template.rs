use crate::models::post::Post;
use handlebars::Handlebars;
use serde::Serialize;
use std::{collections::HashMap, fs, path::PathBuf};

const DEFAULT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="https://cdn.simplecss.org/simple.min.css">
    <title>{{author}} - {{title}}</title>
    <meta name="author" content="{{author}}">
    {{#if description}}
    <meta name="description" content="{{description}}">
    {{/if}}
</head>
<body>
    <article>
        <header>
            <h1>{{title}}</h1>
        </header>
        <div class="content">
            {{{content}}}
        </div>
        <hr>
        <p class="author">By {{author}}</p>
    </article>
</body>
</html>"#;

#[derive(Clone)]
pub struct Templates {
    engine: Handlebars<'static>,
}

#[derive(Serialize)]
struct TemplateData<'a> {
    title: &'a str,
    author: &'a str,
    content: &'a str,
    description: Option<&'a str>,
    metadata: &'a HashMap<String, serde_json::Value>,
    path: &'a str,
}

impl Templates {
    pub fn new() -> Self {
        Self {
            engine: Handlebars::new(),
        }
    }

    pub fn render(&self, data_dir: &PathBuf, username: &str, post: &Post, content: &str) -> String {
        // Check for user template
        let template = if let Ok(user_template) =
            fs::read_to_string(data_dir.join(username).join("template.html"))
        {
            user_template
        } else {
            DEFAULT_TEMPLATE.to_string()
        };

        // Extract description from metadata if it exists
        let description = post
            .metadata
            .extra
            .get("description")
            .and_then(|v| v.as_str());

        let template_data = TemplateData {
            title: &post.name,
            author: username,
            content,
            description,
            metadata: &post.metadata.extra,
            path: &post.path,
        };

        self.engine
            .render_template(&template, &template_data)
            .unwrap_or_else(|_| {
                // Fallback to default template if user template fails
                self.engine
                    .render_template(DEFAULT_TEMPLATE, &template_data)
                    .unwrap_or_else(|_| "Template rendering failed".to_string())
            })
    }
}
