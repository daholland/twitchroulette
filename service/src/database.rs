use std::fmt;

use anyhow::Result;
use rand::seq::SliceRandom;
use sqlx::postgres::PgRow;
use sqlx::{query_as, types::Uuid, PgPool, Row};

#[derive(Debug, sqlx::FromRow)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub display: String,
    pub boxart: Option<String>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.display)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Broadcaster {
    pub id: i64,
    pub login: String,
    pub display_name: String,
    pub profile_image: Option<String>,
    pub color: Option<String>,
}

impl fmt::Display for Broadcaster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.display_name)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub is_language_tag: bool,
    pub localized_name: String,
    pub name: String,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.localized_name)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Stream {
    pub id: i64,
    pub title: Option<String>,
    pub preview_image: Option<String>,
    pub broadcaster: Broadcaster,
    pub game: Game,
    pub tags: Vec<Tag>,
    pub stream_type: Option<String>,
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.id)
    }
}

pub async fn get_games(pool: &PgPool) -> Result<Vec<Game>> {
    let results = query_as::<_, Game>(
        r#"
        select distinct
          name, display, boxart, game as id
        from
          games
        left join
          streams s on games.id = s.game
        where
           s.updated_at >= NOW() - INTERVAL '5 minutes'
        order by
           name;
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(results)
}

pub async fn get_tags(pool: &PgPool) -> Result<Vec<Tag>> {
    let results = query_as::<_, Tag>(
        r#"
        select tags.id, tags.localized_name, tags.is_language_tag, tags.name
        from tags
        where tags.id in (select distinct unnest(tags) from streams where streams.updated_at >= NOW() - INTERVAL '5 minutes')
        order by tags.name;
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(results)
}

pub async fn insert_game(pool: &PgPool, game: &Game) -> Result<()> {
    sqlx::query!(
        r#"
        insert into games (id, name, display, boxart)
        values ($1, $2, $3, $4)
        on conflict (id)
        do nothing;
        "#,
        game.id,
        game.name,
        game.display,
        game.boxart,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_broadcaster(pool: &PgPool, broadcaster: &Broadcaster) -> Result<()> {
    sqlx::query!(
        r#"
        insert into broadcasters (id, login, display_name, profile_image, color)
        values ($1, $2, $3, $4, $5)
        on conflict (id)
        do nothing;
        "#,
        broadcaster.id,
        broadcaster.login,
        broadcaster.display_name,
        broadcaster.profile_image,
        broadcaster.color,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_tag(pool: &PgPool, tag: &Tag) -> Result<()> {
    sqlx::query!(
        r#"
        insert into tags (id, is_language_tag, localized_name, name)
        values ($1, $2, $3, $4)
        on conflict (id)
        do nothing;
        "#,
        tag.id,
        tag.is_language_tag,
        tag.localized_name,
        tag.name,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_stream(pool: &PgPool, stream: Stream) -> Result<()> {
    insert_game(pool, &stream.game).await?;
    insert_broadcaster(pool, &stream.broadcaster).await?;

    let mut tag_ids: Vec<Uuid> = vec![];

    for tag in &stream.tags {
        insert_tag(pool, &tag).await?;
        tag_ids.push(tag.id);
    }

    sqlx::query!(
        r#"
        insert into streams (id, title, preview_image, broadcaster, game, tags, stream_type)
        values ($1, $2, $3, $4, $5, $6::uuid[], $7)
        on conflict (id) do update
        set
            title=excluded.title,
            preview_image=excluded.preview_image,
            game=excluded.game,
            tags=excluded.tags::uuid[],
            stream_type=excluded.stream_type;
        "#,
        stream.id,
        stream.title,
        stream.preview_image,
        stream.broadcaster.id,
        stream.game.id,
        &tag_ids,
        stream.stream_type,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_stream_from_id(pool: &PgPool, id: i64) -> Result<Stream> {
    let stream = sqlx::query(
        r#"
    select
        streams.*,
        games.name as game_name,
        games.display as game_display,
        games.boxart as game_boxart,
        broadcasters.id as broadcaster_id,
        broadcasters.login as broadcaster_login,
        broadcasters.display_name as broadcaster_display_name,
        broadcasters.profile_image as broadcaster_profile_image,
        broadcasters.color as broadcaster_color,
    from streams
    inner join
        games
    on
        streams.game = games.id,
    inner join
        broadcasters
    on
        streams.broadcaster = broadcasters.id
    where
        streams.id = ?
    "#,
    )
    .bind(id)
    .map(|row: PgRow| {
        let game_name: &str = row.get("game_name");
        let game_display: &str = row.get("game_dsplay");
        let game_id: i64 = row.get("game");

        let game = Game {
            id: game_id,
            name: game_name.to_string(),
            display: game_display.to_string(),
            boxart: None,
        };

        let broadcaster_id: i64 = row.get("broadcaster_id");
        let broadcaster_login: &str = row.get("broadcaster_login");
        let broadcaster_display_name: &str = row.get("broadcaster_display_name");
        let broadcaster_profile_image: &str = row.get("broadcaster_profile_image");

        let broadcaster = Broadcaster {
            id: broadcaster_id,
            login: broadcaster_login.to_string(),
            display_name: broadcaster_display_name.to_string(),
            profile_image: Some(broadcaster_profile_image.to_string()),
            color: None,
        };

        let id: i64 = row.get("id");
        let title: &str = row.get("title");
        let preview_image: &str = row.get("preview_image");

        Stream {
            id,
            title: Some(title.to_string()),
            preview_image: Some(preview_image.to_string()),
            tags: vec![],
            stream_type: None,
            broadcaster,
            game,
        }
    })
    .fetch_one(pool)
    .await?;

    Ok(stream)
}

pub async fn get_random_stream_from_filters(
    pool: &PgPool,
    included_tag_ids: Vec<Uuid>,
    excluded_tag_ids: Vec<Uuid>,
    included_game_ids: Vec<i32>,
    excluded_game_ids: Vec<i32>,
) -> Option<Stream> {
    let mut query =
        "select streams.id from streams where streams.updated_at >= NOW() - INTERVAL '5 minutes'"
            .to_string();

    if !included_tag_ids.is_empty() {
        let included_tags: String = included_tag_ids
            .into_iter()
            .map(|tag_id| tag_id.to_hyphenated().to_string())
            .collect::<Vec<String>>()
            .join(",");

        query = format!("{} and streams.tags @> '{{{}}}'", query, included_tags);
    }

    if !excluded_tag_ids.is_empty() {
        let excluded_tags: String = excluded_tag_ids
            .into_iter()
            .map(|tag_id| tag_id.to_hyphenated().to_string())
            .collect::<Vec<String>>()
            .join(",");

        query = format!("{} and not streams.tags @> '{{{}}}'", query, excluded_tags);
    }

    if !included_game_ids.is_empty() {
        let included_games = included_game_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        query = format!(
            "{} and streams.games = ANY('{{{}}}')",
            query, included_games
        );
    }

    if !excluded_game_ids.is_empty() {
        let excluded_games = excluded_game_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        query = format!(
            "{} and not streams.games = ANY('{{{}}}')",
            query, excluded_games
        );
    }

    query = format!("{};", query);

    let stream_id_list_result: Vec<i64> = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|row| {
            let id: i64 = row.get("id");
            id
        })
        .collect();

    if stream_id_list_result.is_empty() {
        return None;
    }

    if let Some(stream_id) = stream_id_list_result.choose(&mut rand::thread_rng()) {
        return get_stream_from_id(pool, stream_id.clone()).await.ok();
    }

    return None;
}
