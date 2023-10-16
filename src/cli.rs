//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use crate::daemon_client::DaemonClient;
use clap::Parser;
use clap::Subcommand;
use multiaddr::Multiaddr;
use tari_engine_types::parse_arg;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub(crate) struct Cli {
    #[clap(long, short = 'e', alias = "endpoint", env = "JRPC_ENDPOINT")]
    pub daemon_jrpc_endpoint: Option<String>,
    #[clap(long, short = 't', alias = "token")]
    pub auth_token: Option<String>,
    #[clap(
        long,
        alias = "template_address",
        default_value = "be232b36a4291fe9315ab4fb3f7bfca6e7342bec88f5e6ffa2e9a0e0fce4005a"
    )]
    pub template: String,
    #[clap(long, short = 'd')]
    pub dump_buckets: bool,
    #[clap(long)]
    pub dry_run: bool,
    #[clap(subcommand)]
    pub command: Command,
    #[clap(long, short = 'f', default_value = "100")]
    pub fees: u64,
}

impl Cli {
    pub fn init() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand, Clone)]
pub(crate) enum Command {
    Login(login::Command),

    CreatePool(create_pool::Command),

    MakePrediction(make_prediction::Command),

    TakeFreeCoins(take_free_coins::Command),
}

pub mod login {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use std::fs;

    #[derive(Debug, Args, Clone)]
    pub struct Command {}

    impl Command {
        pub async fn run(self, mut client: DaemonClient) {
            let token = client.login().await;
            fs::write("token.data", token).unwrap();
        }
    }
}

pub(crate) mod create_pool {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub token_symbol: String,

        pub team_1: String,

        pub team_2: String,

        pub num_players: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            template_address: TemplateAddress,
            fees: u64,
        ) {
            // let template_address= ;
            let function = "create_pool".to_string();

            client
                .submit_instruction(
                    Instruction::CallFunction {
                        template_address,
                        function,
                        args: vec![
                            parse_arg(&self.token_symbol).unwrap(),
                            parse_arg(&self.team_1).unwrap(),
                            parse_arg(&self.team_2).unwrap(),
                            parse_arg(&self.num_players).unwrap(),
                        ],
                    },
                    false,
                    false,
                    fees,
                    vec![],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod make_prediction {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub difference: String,

        pub membership: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "make_prediction".to_string();

            client
                .submit_instruction(
                    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address)
                            .unwrap(),
                        method,
                        args: vec![
                            parse_arg(&self.difference).unwrap(),
                            parse_arg(&self.membership).unwrap(),
                        ],
                    },
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod take_free_coins {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use serde_json::json;
    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_transaction::SubstateRequirement;
    use tari_utilities::hex::from_hex;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "take_free_coins".to_string();

            client
                .submit_instruction(
                    Instruction::CallMethod {
                        component_address: ComponentAddress::from_hex(&self.component_address)
                            .unwrap(),
                        method,
                        args: vec![],
                    },
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}
