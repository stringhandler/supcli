//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use multiaddr::Multiaddr;
use reqwest;
use serde_json::json;
use serde_json::Value;
use std::str::FromStr;
use tari_engine_types::instruction::Instruction;
use tari_transaction::SubstateRequirement;
use tari_wallet_daemon_client::types::AuthLoginRequest;
use tari_wallet_daemon_client::types::CallInstructionRequest;
use tari_wallet_daemon_client::types::TransactionSubmitRequest;
use tari_wallet_daemon_client::ComponentAddressOrName;
use tari_wallet_daemon_client::WalletDaemonClient;

pub struct DaemonClient {
    endpoint: String,
    auth_token: Option<String>,
    last_id: usize,
}

impl DaemonClient {
    pub(crate) fn new(endpoint: String, auth_token: Option<String>) -> Self {
        Self {
            endpoint,
            auth_token,
            last_id: 0,
        }
    }

    pub async fn login(&mut self) -> String {
        let mut client =
            WalletDaemonClient::connect(&self.endpoint, self.auth_token.clone()).unwrap();
        let r = client
            .auth_request(&AuthLoginRequest {
                permissions: vec!["Admin".to_string()],
                duration: None,
            })
            .await
            .unwrap();

        dbg!(&r);

        r.auth_token
    }

    pub async fn submit_instruction(
        &mut self,
        instruction: Instruction,
        dump_buckets: bool,
        is_dry_run: bool,
        fees: u64,
        other_inputs: Vec<SubstateRequirement>,
    ) {
        self.submit_instructions(
            vec![instruction],
            dump_buckets,
            is_dry_run,
            fees,
            other_inputs,
        )
        .await;
    }

    pub async fn submit_instructions(
        &mut self,
        instructions: Vec<Instruction>,
        dump_buckets: bool,
        is_dry_run: bool,
        fees: u64,
        other_inputs: Vec<SubstateRequirement>,
    ) {
        let mut client =
            WalletDaemonClient::connect(&self.endpoint, self.auth_token.clone()).unwrap();
        //let r = client.list_keys().await;

        //dbg!(r);

        let tx = CallInstructionRequest {
            instructions,
            fee_account: ComponentAddressOrName::Name("TestAccount_0".to_string()),
            dump_outputs_into: if dump_buckets {
                Some(ComponentAddressOrName::Name("TestAccount_0".to_string()))
            } else {
                None
            },
            fee: fees,
            inputs: other_inputs,
            override_inputs: None,
            new_outputs: None,
            specific_non_fungible_outputs: vec![],
            new_resources: vec![],
            new_non_fungible_outputs: vec![],
            new_non_fungible_index_outputs: vec![],
            is_dry_run,
            proof_ids: vec![],
        };

        let r2 = client.submit_instruction(tx).await.unwrap();

        dbg!(r2);

        //"dump_outputs_into": "TestAccount_0",
    }

    //  {
    //    "instruction": instruction,
    //  "fee_account": self.last_account_name,
    //     "dump_outputs_into": self.last_account_name,
    //     "fee": 1000,
    // },
    //
}
