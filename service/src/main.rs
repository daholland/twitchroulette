use scraper::scrape_streams;
use structopt::StructOpt;
use tonic::transport::Server;

use crate::service::MyTwitchRouletteService;
use crate::service::TwitchRouletteServiceServer;
use crate::service::protos;

mod database;
mod scraper;
mod service;

#[derive(StructOpt, Debug)]
#[structopt(name = "twitchroulette")]
struct TwitchRouletteOpts {
    #[structopt(
        short,
        long,
        env = "DATABASE_URL",
        default_value = "postgres://postgres:password@localhost/twitch"
    )]
    database_url: String,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Server(ServerCmd),
    Scraper(ScraperCmd),
    Util(UtilCmd),
}

#[derive(StructOpt, Debug)]
struct ServerCmd {
    #[structopt(subcommand)]
    command: ServerSubCmd,
}

#[derive(StructOpt, Debug)]
enum ServerSubCmd {
    Start {
        #[structopt(default_value = "127.0.0.1:4045")]
        address: String,
    },
}

#[derive(StructOpt, Debug)]
struct ScraperCmd {
    #[structopt(subcommand)]
    command: ScraperSubCmd,
}

#[derive(StructOpt, Debug)]
enum ScraperSubCmd {
    Run {
        #[structopt(default_value = "100")]
        pages: usize,
    },
}

#[derive(StructOpt, Debug)]
struct UtilCmd {
    #[structopt(subcommand)]
    command: UtilSubCmd,
}

#[derive(StructOpt, Debug)]
enum UtilSubCmd {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = TwitchRouletteOpts::from_args();

    match opts.command {
        Command::Server(cmd) => match cmd.command {
            ServerSubCmd::Start { address } => {
                println!("Listening on {:?}", address);

                let db_url = opts.database_url.clone();
                let twitch_service: MyTwitchRouletteService = MyTwitchRouletteService::new(db_url).await?;

                Server::builder()
                    .accept_http1(true)
                    .add_service(tonic_web::enable(TwitchRouletteServiceServer::new(twitch_service)))
                    .serve(address.parse()?)
                    .await?;

                Ok(())
            }
        },
        Command::Scraper(cmd) => match cmd.command {
            ScraperSubCmd::Run { pages } => {
                let db_url = opts.database_url.clone();
                scrape_streams(&db_url, pages).await?;

                Ok(())
            }
        },
        _ => Ok(())
    }
}
