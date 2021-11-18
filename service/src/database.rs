use std::fmt;

use anyhow::Result;
use sqlx::{PgPool, query_as, types::{
        chrono::{DateTime, Utc},
        Uuid,
    }};

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

#[derive(Debug)]
pub struct Cursor {
    id: String,
    offset: i64,
    limit: i64,
    query: Option<String>,
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
