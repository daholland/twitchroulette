use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tonic;

pub use protos::twitch_roulette_service_server::TwitchRouletteService;
pub use protos::twitch_roulette_service_server::TwitchRouletteServiceServer;
use tonic::Response;
use uuid::Uuid;

use crate::database::get_tags;
use crate::database::{get_games, get_random_stream_from_filters};

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
        let args = request.into_inner();

        let included_tags = args
            .included_tag_ids
            .into_iter()
            .flat_map(|id| Uuid::parse_str(&id))
            .collect();

        let excluded_tags = args
            .excluded_tag_ids
            .into_iter()
            .flat_map(|id| Uuid::parse_str(&id))
            .collect();

        let included_games = args
            .included_game_ids
            .into_iter()
            .filter_map(|id| id.parse().ok())
            .collect();

        let excluded_games = args
            .excluded_game_ids
            .into_iter()
            .filter_map(|id| id.parse().ok())
            .collect();

        let db_stream = get_random_stream_from_filters(
            &self.db_pool,
            included_tags,
            excluded_tags,
            included_games,
            excluded_games,
        )
        .await;

        let stream = match db_stream {
            Some(db_stream) => {
                let stream_user = protos::StreamUser {
                    id: db_stream.broadcaster.id.to_string(),
                    display_name: db_stream.broadcaster.display_name,
                    login_name: db_stream.broadcaster.login,
                    image_url: db_stream.broadcaster.profile_image.unwrap_or_default(),
                    ..Default::default()
                };

                let stream_game = protos::StreamGame {
                    id: db_stream.game.id.to_string(),
                    name: db_stream.game.name,
                    image_url: db_stream.game.boxart.unwrap_or_default(),
                    ..Default::default()
                };

                Some(protos::Stream {
                    id: db_stream.id.to_string(),
                    user: Some(stream_user),
                    game: Some(stream_game),
                    title: db_stream.title.unwrap_or_default(),
                    ..Default::default()
                })
            }
            _ => None,
        };

        let response = protos::GetRandomStreamResponse {
            stream,
            ..Default::default()
        };

        Ok(Response::new(response))
    }

    async fn get_stream_tags(
        &self,
        _request: tonic::Request<protos::GetStreamTagsRequest>,
    ) -> Result<tonic::Response<protos::GetStreamTagsResponse>, tonic::Status> {
        let tags: Vec<_> = get_tags(&self.db_pool)
            .await
            .unwrap()
            .into_iter()
            .map(|db_tag| protos::StreamTag {
                id: db_tag.id.to_string(),
                name: db_tag.localized_name,
            })
            .collect();

        let response = protos::GetStreamTagsResponse { tags };

        Ok(Response::new(response))
    }

    async fn get_stream_games(
        &self,
        _request: tonic::Request<protos::GetStreamGamesRequest>,
    ) -> Result<tonic::Response<protos::GetStreamGamesResponse>, tonic::Status> {
        let games: Vec<_> = get_games(&self.db_pool)
            .await
            .unwrap()
            .into_iter()
            .map(|db_game| protos::StreamGame {
                id: db_game.id.to_string(),
                name: db_game.name,
                image_url: db_game
                    .boxart
                    .unwrap_or("https://placekitten.com/40/56".to_string()),
            })
            .collect();

        let response = protos::GetStreamGamesResponse { games };

        Ok(Response::new(response))
    }
}
