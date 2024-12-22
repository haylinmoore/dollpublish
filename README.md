# DollPublish

A simple and elegant platform for publishing markdown documents directly from Obsidian.

## Getting Started

1. Message ```_haylin``` on Discord to get your API key
2. Install the Obsidian plugin
3. Start publishing!

Your posts will be available at ```https://estrogen.coffee/<username>/<id>/```
The ```<id>``` will appear at the top of your Obsidian document after your first publish.

## Obsidian Integration

1. Install the [MoonServer Obsidian Plugin](https://github.com/Dzoukr/MoonServerObsidianPlugin)
2. Configure the plugin:
   - Server URL: ```https://estrogen.coffee/_moon/```
   - Place your API key in either the API-key or API-secret field
3. Start publishing directly from Obsidian!

## Customizing Your Pages

DollPublish uses Handlebars templates for rendering your published pages. You can customize how your content looks by uploading your own template.

To upload a new template:
```bash
curl -X PUT \
     -H "api-key: your-api-key" \
     --data-binary @template.html \
     https://estrogen.coffee/_files/template.html
```

### Template Variables

```rust
struct TemplateData<'a> {
    title: &'a str,
    author: &'a str,
    content: &'a str,
    description: Option<&'a str>,
    metadata: &'a HashMap<String, serde_json::Value>,
    path: &'a str,
}
```

### Example Template

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{author}} - {{title}}</title>
    {{#if description}}
    <meta name="description" content="{{description}}">
    {{/if}}
</head>
<body>
    <div>
        <h1>{{title}}</h1>
        <div>
            {{{content}}}
        </div>
        <hr>
        <p>By {{author}}</p>
    </div>
</body>
</html>
```

Note: Use triple braces ```{{{content}}}``` for the content variable to ensure proper HTML rendering.
