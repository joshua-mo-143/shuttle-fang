use async_trait::async_trait;
use serde::Serialize;
use shuttle_service::{
    database::{SharedEngine, Type as DatabaseType},
    DbInput, DbOutput, Factory, ResourceBuilder, Type,
};
use fang::{NoTls, AsyncQueue};

#[derive(Serialize)]
pub struct Postgres {
    config: DbInput,
}

impl Postgres {
    pub fn local_uri(mut self, local_uri: impl ToString) -> Self {
		self.config.local_uri = Some(local_uri.to_string());

		self
        }
}

fn get_connection_string(db_output: &DbOutput) -> String {
    match db_output {
        DbOutput::Info(ref info) => info.connection_string_private(),
        DbOutput::Local(ref local) => local.clone(),
    }
}

#[async_trait]
impl ResourceBuilder<AsyncQueue<NoTls>> for Postgres {
    const TYPE: Type = Type::Database(DatabaseType::Shared(SharedEngine::Postgres));

    type Config = DbInput;
    type Output = DbOutput;

    fn new() -> Self {
	Self {
		config: Default::default()
	}
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn output(
        self,
        factory: &mut dyn Factory,
    ) -> Result<Self::Output, shuttle_service::Error> {
        let db_output = if let Some(local_uri) = self.config.local_uri {
            DbOutput::Local(local_uri)
        } else {
            let conn_data = factory
                .get_db_connection(DatabaseType::Shared(SharedEngine::Postgres))
                .await?;
            DbOutput::Info(conn_data)
        };

        Ok(db_output)
    }

    async fn build(db_output: &Self::Output) -> Result<AsyncQueue<NoTls>, shuttle_service::Error> {
    let conn_string = get_connection_string(db_output);
        
    let mut queue = AsyncQueue::builder().uri(conn_string).max_pool_size(5u32).build();

    let _ = queue.connect(NoTls).await.map_err(|err| shuttle_service::Error::Custom(err.into()));

    Ok(queue)
    }
}
