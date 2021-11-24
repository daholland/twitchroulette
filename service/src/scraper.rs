use std::collections::HashMap;

use anyhow::{anyhow, Result};
use futures::{StreamExt, stream::FuturesUnordered};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct StreamBroadcaster {
    id: String,
    login: String,
    displayName: String,
    profileImageURL: Option<String>,
    primaryColorHex: Option<String>,
    __typename: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct StreamGame {
    id: String,
    name: String,
    displayName: String,
    boxArtURL: String,
    __typename: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct StreamTag {
    id: String,
    isLanguageTag: bool,
    localizedName: String,
    tagName: String,
    __typename: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct StreamEdgeNode {
    id: String,
    title: Option<String>,
    viewersCount: usize,
    previewImageURL: String,
    broadcaster: StreamBroadcaster,
    game: Option<StreamGame>,
    tags: Option<Vec<StreamTag>>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct StreamEdge {
    cursor: String,
    node: StreamEdgeNode,
    trackingId: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct StreamEdgesResponse {
    edges: Vec<StreamEdge>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct StreamDataResponse {
    streams: StreamEdgesResponse,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct StreamResponse {
    data: Option<StreamDataResponse>,
    errors: Option<Value>,
}

async fn get_stream_edges(cursor: Option<String>) -> Result<Vec<StreamEdge>> {
    let payload = json!([{
        "operationName": "BrowsePage_Popular",
        "variables": {
            "limit": 30,
            "platformType": "all",
            "options": {
                "includeRestricted": ["SUB_ONLY_LIVE"],
                "sort": "VIEWER_COUNT_ASC",
                "tags": [],
                "recommendationsContext": {"platform": "web"},
                "requestID": "JIRA-VXP-2397",
            },
            "cursor": cursor,
            "sortTypeIsRecency": false,
        },
        "extensions": {
            "persistedQuery": {
                "version": 1,
                "sha256Hash": "4de7f2166105c1a034ba40251f55593b90500f69cf44c8735db4f62ad2760c39",
            }
        },
    }]);

    let response = reqwest::Client::new()
        .post("https://gql.twitch.tv/gql")
        .header("Client-Id", "kimne78kx3ncx6brgo4mv6wki5h1ko")
        .json(&payload)
        .send()
        .await?;

    let responses = response
        .json::<Vec<StreamResponse>>()
        .await?
        .first()
        .unwrap()
        .clone();

    match responses.errors {
        Some(_) => return Err(anyhow!("got error in response")),
        _ => (),
    }

    let mut edges = vec![];

    match responses.data {
        Some(data) => {
            edges = data.streams.edges;
        }
        _ => (),
    }

    Ok(edges)
}

async fn fetch_streams(pages: usize) -> Result<Vec<StreamEdge>> {
    let mut edges: Vec<StreamEdge> = vec![];
    let mut cursor: Option<String> = None;
    let mut streams_indexed: HashMap<String, StreamEdge> = HashMap::new();

    for page in 1..=pages {
        println!("fetching page {:?} / {:?}", page, pages);

        let results = get_stream_edges(cursor.clone()).await;

        match results {
            Ok(mut result) => {
                cursor = Some(result.last().unwrap().cursor.clone());

                for edge in &result {
                    let id = edge.node.broadcaster.id.to_string();
                    streams_indexed.insert(id, edge.clone());
                }

                edges.append(&mut result);
            }
            _ => {
                cursor = None;
                continue;
            }
        };
    }

    let edge_list = streams_indexed.values().cloned().collect();

    Ok(edge_list)
}

async fn insert_streams(
    pool: Pool<Postgres>,
    edges: Vec<StreamEdge>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut futures = FuturesUnordered::new();

    edges.iter().map(|edge| edge.node.clone()).for_each(|node| {
        let db_broadcaster = crate::database::Broadcaster {
            id: node.broadcaster.id.parse::<i64>().unwrap(),
            login: node.broadcaster.login,
            display_name: node.broadcaster.displayName,
            profile_image: node.broadcaster.profileImageURL,
            color: node.broadcaster.primaryColorHex,
        };

        let db_game = match node.game {
            Some(game) => crate::database::Game {
                id: game.id.parse::<i64>().unwrap(),
                name: game.name,
                display: game.displayName,
                boxart: Some(game.boxArtURL),
            },
            None => crate::database::Game {
                id: 0,
                name: "Unknown".to_string(),
                display: "Unknown".to_string(),
                boxart: None,
            },
        };

        let db_tags = match node.tags {
            Some(tags) => {
                tags.iter().map(|tag| {
                    let id = Uuid::parse_str(&tag.id).unwrap();
                    crate::database::Tag {
                        id,
                        is_language_tag: tag.isLanguageTag,
                        localized_name: tag.localizedName.clone(),
                        name: tag.tagName.clone(),
                    }
                }).collect()
            },
            None => vec![],
        };

        let stream = crate::database::Stream {
            id: node.id.parse::<i64>().unwrap(),
            title: node.title,
            preview_image: Some(node.previewImageURL),
            broadcaster: db_broadcaster,
            game: db_game,
            tags: db_tags,
            stream_type: None,
        };

        let future = crate::database::insert_stream(&pool, stream);
        futures.push(future);
    });

    while let Some(_) = futures.next().await {}

    Ok(())
}

pub async fn scrape_streams(db_url: &str, pages: usize) -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    let streams = fetch_streams(pages).await?;

    insert_streams(pool, streams).await
}
