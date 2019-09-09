/*
 * Copyright 2018-2019 TON DEV SOLUTIONS LTD.
 *
 * Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
 * this file except in compliance with the License.  You may obtain a copy of the
 * License at: https://ton.dev/licenses
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific TON DEV software governing permissions and
 * limitations under the License.
 */

use crate::interop::Interop;
use crate::{TonCrypto, TonContracts, TonResult, TonQueries};

#[derive(Default, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TonClientConfig {
    pub default_workchain: Option<i32>,
    pub base_url: Option<String>,
    pub requests_url: Option<String>,
    pub queries_url: Option<String>,
    pub subscriptions_url: Option<String>,
}

pub struct TonClient {
    context: u32,
    pub crypto: TonCrypto,
    pub contracts: TonContracts,
    pub queries: TonQueries,
}

impl TonClient {
    pub fn new(config: &TonClientConfig) -> TonClient {
        let context = Interop::create_context();
        let client = TonClient {
            context,
            crypto: TonCrypto::new(context),
            contracts: TonContracts::new(context),
            queries: TonQueries::new(context),
        };
        client.setup(config);
        client
    }

    pub fn new_with_base_url(base_url: &str) -> TonClient {
        Self::new(&TonClientConfig {
            base_url: Some(base_url.to_string()),
            requests_url: None,
            queries_url: None,
            subscriptions_url: None,
            default_workchain: Some(0)
        })
    }

    pub fn default() -> TonClient {
        Self::new(&TonClientConfig::default())
    }

    pub fn get_client_version(&self) -> String {
        Interop::json_request_no_args(self.context, "version").unwrap()
    }

    pub fn setup(&self, config: &TonClientConfig) -> TonResult<()> {
        Interop::json_request(self.context, "setup", config)
    }
}
