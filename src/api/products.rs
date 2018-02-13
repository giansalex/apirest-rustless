use rustless;
use rustless::prelude::*;
use jsonway::{ObjectSerializer, ArraySerializer};
use uuid;
use url;

use super::super::model::product;
use super::super::serializers::tweet_serializer;

pub fn tweets(path: &str) -> rustless::Namespace {
    rustless::Namespace::build(path, dsl!(|products| {

        post("", dsl!(|endpoint| {
            desc("Create new product");
            params(|params| {
                params.req("product", |product| {
                    product.desc("Tweet model in JSON format");
                    product.schema(|product| {
                        product.id("http://tweet.example.com/tweet-short");
                        product.object();
                        product.properties(|props| {
                            props.insert("description", |description| {
                                description.string();
                                description.max_length(100);
                            });
                        });
                        product.required(vec![
                            "description".to_string()
                        ]);
                        product.additional_properties(false);
                    })
                })
            });
            handle(|client, params| {
                // Note that .db() is an extension methods that we created with DatabaseExt
                let cn = client.app.db();
                let tweet = params.find("tweet").unwrap();

                let tweet = tweet::Tweet::new(
                    tweet.find("author_name").unwrap().as_string().unwrap().to_string(),
                    tweet.find("content").unwrap().as_string().unwrap().to_string()
                );

                tweet.create(&*cn);
                client.json(&tweet_serializer::TweetSerializer.serialize(&tweet, true))
            })
        }));

        namespace(":product_id", dsl!(|single| {
            params(|params| {
                params.req("product_id", |product_id| {
                    product_id.desc("Product ID in UUID format");
                    product_id.schema(|schema| {
                        schema.format("uuid");
                    })
                })
            });

            get("", dsl!(|endpoint| {
                desc("Get product by ID");
                handle(|mut client, params| {
                    // Note that .db() is an extension methods that we created with DatabaseExt

                    let tweet = tweet::Tweet::find(id);

                    if tweet.is_some() {
                        client.json(&tweet_serializer::TweetSerializer.serialize(&tweet.unwrap(), true))
                    } else {
                        client.not_found();
                        client.empty()
                    }
                })
            }));
        }))

    }))
}