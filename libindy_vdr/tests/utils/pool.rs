use std::env;

use futures_executor::block_on;

use indy_vdr::common::error::VdrResult;
use indy_vdr::config::PoolConfig;
use indy_vdr::ledger::RequestBuilder;
use indy_vdr::pool::helpers::{perform_ledger_action, perform_ledger_request};
use indy_vdr::pool::{
    NodeReplies, Pool, PoolBuilder, PoolTransactions, PreparedRequest, RequestResult, SharedPool,
};

pub fn default_transactions() -> Vec<String> {
    let test_pool_ip = env::var("TEST_POOL_IP").unwrap_or("127.0.0.1".to_string());

    vec![
        format!(
            r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","blskey_pop":"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}"#,
            test_pool_ip, test_pool_ip
        ),
        format!(
            r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","blskey_pop":"Qr658mWZ2YC8JXGXwMDQTzuZCWF7NK9EwxphGmcBvCh6ybUuLxbG65nsX4JvD4SPNtkJ2w9ug1yLTj6fgmuDg41TgECXjLCij3RMsV8CwewBVgVN67wsA45DFWvqvLtu4rjNnE9JbdFTc1Z4WCPA3Xan44K1HoHAq9EVeaRYs8zoF5","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}"#,
            test_pool_ip, test_pool_ip
        ),
        format!(
            r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","blskey_pop":"QwDeb2CkNSx6r8QC8vGQK3GRv7Yndn84TGNijX8YXHPiagXajyfTjoR87rXUu4G4QLk2cF8NNyqWiYMus1623dELWwx57rLCFqGh7N4ZRbGDRP4fnVcaKg1BcUxQ866Ven4gw8y4N56S5HzxXNBZtLYmhGHvDtk6PFkFwCvxYrNYjh","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}"#,
            test_pool_ip, test_pool_ip
        ),
        format!(
            r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","blskey_pop":"RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#,
            test_pool_ip, test_pool_ip
        ),
    ]
}

pub struct TestPool {
    pub pool: SharedPool,
}

impl TestPool {
    pub fn new() -> TestPool {
        let pool_transactions =
            PoolTransactions::from_json_transactions(default_transactions()).unwrap();

        let pool = PoolBuilder::new(PoolConfig::default(), pool_transactions)
            .into_shared()
            .unwrap();

        TestPool { pool }
    }

    pub fn transactions(&self) -> Vec<String> {
        self.pool.get_transactions().encode_json().unwrap()
    }

    pub fn request_builder(&self) -> RequestBuilder {
        self.pool.get_request_builder()
    }

    pub fn send_request(&self, prepared_request: &PreparedRequest) -> Result<String, String> {
        block_on(async {
            let (request_result, _meta) = perform_ledger_request(&self.pool, prepared_request)
                .await
                .unwrap();

            match request_result {
                RequestResult::Reply(message) => Ok(message),
                RequestResult::Failed(err) => Err(err.extra().unwrap_or_default()),
            }
        })
    }

    pub fn send_full_request(
        &self,
        prepared_request: &PreparedRequest,
        node_aliases: Option<Vec<String>>,
        timeout: Option<i64>,
    ) -> VdrResult<NodeReplies<String>> {
        block_on(async {
            let (request_result, _meta) = perform_ledger_action(
                &self.pool,
                prepared_request.req_id.clone(),
                prepared_request.req_json.to_string(),
                node_aliases,
                timeout,
            )
            .await
            .unwrap();
            match request_result {
                RequestResult::Reply(replies) => Ok(replies),
                RequestResult::Failed(err) => Err(err),
            }
        })
    }

    pub fn send_request_with_retries(
        &self,
        prepared_request: &PreparedRequest,
        previous_response: &str,
    ) -> Result<String, String> {
        Self::_submit_retry(
            Self::extract_seq_no_from_reply(previous_response).unwrap(),
            || self.send_request(prepared_request),
        )
    }

    pub fn extract_seq_no_from_reply(reply: &str) -> Result<u64, &'static str> {
        let reply: serde_json::Value =
            serde_json::from_str(reply).map_err(|_| "Cannot deserialize transaction Response")?;

        let seq_no = reply["result"]["seqNo"]
            .as_u64()
            .or_else(|| reply["result"]["txnMetadata"]["seqNo"].as_u64())
            .ok_or("Missed seqNo in reply")?;

        Ok(seq_no)
    }

    const SUBMIT_RETRY_CNT: usize = 3;

    fn _submit_retry<F>(minimal_timestamp: u64, submit_action: F) -> Result<String, String>
    where
        F: Fn() -> Result<String, String>,
    {
        let mut i = 0;
        let action_result = loop {
            let action_result = submit_action()?;

            let retry = Self::extract_seq_no_from_reply(&action_result)
                .map(|received_timestamp| received_timestamp < minimal_timestamp)
                .unwrap_or(true);

            if retry && i < Self::SUBMIT_RETRY_CNT {
                ::std::thread::sleep(::std::time::Duration::from_secs(2));
                i += 1;
            } else {
                break action_result;
            }
        };
        Ok(action_result)
    }
}
