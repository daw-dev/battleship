use rumqttc::AsyncClient;
use std::pin::Pin;

type Callback = dyn FnOnce(AsyncClient) -> Pin<Box<dyn Future<Output = ()> + Send>> + 'static + Send;

pub struct Callbacks {
    list: Vec<Box<Callback>>,
}

impl Callbacks {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn nothing() -> Self {
        Self {
            list: Vec::with_capacity(0),
        }
    }

    pub fn add_callback<F, Fut>(&mut self, callback: F)
    where
        F: FnOnce(AsyncClient) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.list.push(Box::new(move |client| {
            Box::pin(async move { callback(client).await })
        }));
    }

    pub async fn execute_all(self, client: AsyncClient) {
        for callback in self.list.into_iter() {
            callback(client.clone()).await;
        }
    }
}
