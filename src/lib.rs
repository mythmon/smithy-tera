extern crate smithy;
extern crate tera;
extern crate yaml_rust;

use std::collections::HashMap;

use tera::{Context, Tera, Template};
use smithy::{SmithyPlugin, Document, SmithyError};
use yaml_rust::Yaml;

pub struct SmithyTera;

impl SmithyPlugin for SmithyTera {
    fn process(&self, documents: Vec<Document>) -> Result<Vec<Document>, SmithyError> {
        // Load templates
        let mut templates = HashMap::new();
        let mut not_templates = vec![];

        for doc in documents {
            match doc.path.extension().map(|ext| ext.to_str().unwrap()) {
                Some("tmpl") => {
                    let path = doc.path.into_os_string().into_string().unwrap();
                    templates.insert(path.clone(), Template::new(&path, &doc.body));
                },
                _ => {
                    not_templates.push(doc);
                }
            }
        }

        let tera = Tera { templates: templates };

        for doc in not_templates.iter_mut() {
            if let Yaml::String(ref template_name) = doc.metadata["template"] {
                let mut context = Context::new();
                fill_context(&mut context, &doc.metadata);
                doc.body = tera.render(template_name, context).unwrap();
            }
        }

        Ok(not_templates)
    }
}

fn fill_context(context: &mut Context, metadata: &Yaml) {
    if let Some(metadata) = metadata.as_hash() {
        for (key, val) in metadata {
            let key = key.as_str().unwrap();
            match *val {
                Yaml::String(ref v) => context.add(key, v),
                _ => panic!("idk"),
            }
        }
    } else {
        panic!("Can't fill context from non-hash metadata");
    }
}
