#[derive(Debug)]
struct Test {
    #[allow(dead_code)]
    my_state: Option<[u8; 4]>,
}

mashin_sdk::construct_provider!(
    /// This is my provider documentation
    /// Which support multiline
    mashin_provider_starter,
    config = {
        version: u64,
    },
    resources = [my_resource],
    state = |state, _config| {
        let mut state = state.lock();
        let new_state = Test { my_state: Some(*b"test")  };
        state.put(new_state);
    },
    on_drop = |_provider| {
        log!(trace, "my provider dropped");
    }
);

#[mashin_sdk::resource]
pub mod my_resource {
    use mashin_sdk::{
        ext::parking_lot::Mutex, ProviderState, ResourceDefault, ResourceDiff, Result,
    };
    use std::sync::Arc;

    /// This is my resource config
    #[mashin::config]
    pub struct Config {
        /// My config key
        my_key: Option<String>,
    }

    /// This is my resource documentation
    #[mashin::resource]
    pub struct Resource {
        /// My resource key
        my_key: Option<String>,
        #[sensitive]
        my_sensitive_key: Option<String>,
    }

    #[mashin::calls]
    impl mashin_sdk::Resource for Resource {
        async fn get(&mut self, _provider_state: Arc<Mutex<ProviderState>>) -> Result<()> {
            self.set_my_sensitive_key(Some("my-password-not-exported-to-ts".to_string()));
            self.set_my_key(Some("my-value".to_string()));
            Ok(())
        }

        async fn create(&mut self, provider_state: Arc<Mutex<ProviderState>>) -> Result<()> {
            self.get(provider_state).await
        }

        async fn delete(&mut self, provider_state: Arc<Mutex<ProviderState>>) -> Result<()> {
            self.get(provider_state).await
        }

        async fn update(
            &mut self,
            provider_state: Arc<Mutex<ProviderState>>,
            _diff: &ResourceDiff,
        ) -> Result<()> {
            self.get(provider_state).await
        }
    }
}
