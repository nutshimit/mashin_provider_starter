#[mashin_sdk::provider(github_url = "https://github.com/nutshimit/mashin_provider_starter")]
mod my_provider {
	use mashin_sdk::{
		ProviderBuilder, ProviderDefault, ProviderState, ResourceDefault, ResourceDiff, Result,
	};

	#[mashin::provider]
	pub struct Provider;

	#[mashin::config]
	/// This is my provider config
	pub struct Config {
		/// This is my key
		my_key: Option<String>,
	}

	#[mashin::state]
	pub struct State {
		my_state: Option<[u8; 4]>,
	}

	#[mashin::builder]
	impl ProviderBuilder for Provider {
		async fn build(&mut self) -> mashin_sdk::Result<()> {
			log!(trace, "New provider {:?}", self.config());

			let default_state = State { my_state: Some(*b"test") };
			self.state().put(default_state);

			Ok(())
		}
	}

	impl Drop for Provider {
		fn drop(&mut self) {
			log!(trace, "Provider dropped")
		}
	}

	#[mashin::resource_config]
	pub struct MyResourceConfig {
		my_key: Option<String>,
	}

	#[mashin::resource(name = "my_module:my_resource", config = MyResourceConfig)]
	pub struct MyResource {
		my_key: Option<String>,
		#[sensitive]
		my_sensitive_key: Option<String>,
	}

	#[mashin::calls]
	impl mashin_sdk::Resource for MyResource {
		async fn get(&mut self, _provider_state: &ProviderState) -> Result<()> {
			self.my_key = Some("woot".to_string());
			self.my_sensitive_key = Some("my_password!!!!&#*^#*&".to_string());
			Ok(())
		}
		async fn create(&mut self, _provider_state: &ProviderState) -> Result<()> {
			self.get(_provider_state).await
		}
		async fn delete(&mut self, _provider_state: &ProviderState) -> Result<()> {
			todo!()
		}
		async fn update(
			&mut self,
			_provider_state: &ProviderState,
			_diff: &ResourceDiff,
		) -> Result<()> {
			todo!()
		}
	}
}
