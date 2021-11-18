use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tonic;

pub use protos::twitch_roulette_service_server::TwitchRouletteService;
pub use protos::twitch_roulette_service_server::TwitchRouletteServiceServer;
use tonic::Response;

use crate::database::get_games;
use crate::database::get_tags;

pub mod protos {
    tonic::include_proto!("twitchroulette.v1");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("twitchroulette_descriptor");
}

#[derive(Debug)]
pub struct MyTwitchRouletteService {
    db_pool: Pool<Postgres>,
}

impl MyTwitchRouletteService {
    pub async fn new(database_address: String) -> Result<Self> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_address)
            .await?;

        Ok(Self { db_pool })
    }
}

#[tonic::async_trait]
impl TwitchRouletteService for MyTwitchRouletteService {
    async fn get_random_stream(
        &self,
        request: tonic::Request<protos::GetRandomStreamRequest>,
    ) -> Result<tonic::Response<protos::GetRandomStreamResponse>, tonic::Status> {
        todo!()
    }

    async fn get_stream_tags(
        &self,
        _request: tonic::Request<protos::GetStreamTagsRequest>,
    ) -> Result<tonic::Response<protos::GetStreamTagsResponse>, tonic::Status> {
        let tags: Vec<_> = get_tags(&self.db_pool).await.unwrap().into_iter().map(|db_tag| {
            protos::StreamTag {
                id: db_tag.id.to_string(),
                name: db_tag.localized_name,
            }
        }).collect();

        let response = protos::GetStreamTagsResponse {
            tags,
        };

        Ok(Response::new(response))
    }

    async fn get_stream_games(
        &self,
        _request: tonic::Request<protos::GetStreamGamesRequest>,
    ) -> Result<tonic::Response<protos::GetStreamGamesResponse>, tonic::Status> {
        let games: Vec<_> = get_games(&self.db_pool).await.unwrap().into_iter().map(|db_game| {
            protos::StreamGame {
                id: db_game.id.to_string(),
                name: db_game.name,
                image_url: db_game.boxart.unwrap_or("https://placekitten.com/40/56".to_string()),
            }
        }).collect();

        let response = protos::GetStreamGamesResponse {
            games,
        };

        Ok(Response::new(response))
    }
}
