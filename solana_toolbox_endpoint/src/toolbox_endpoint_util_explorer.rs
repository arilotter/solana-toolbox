use std::collections::HashMap;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

use crate::toolbox_endpoint::ToolboxEndpoint;

impl ToolboxEndpoint {
    pub fn compute_explorer_address_url(
        rpc_url: &str,
        address: &Pubkey,
    ) -> String {
        ToolboxEndpoint::compute_explorer_url(
            rpc_url,
            "address",
            &address.to_string(),
            &HashMap::new(),
        )
    }

    pub fn compute_explorer_signature_url(
        rpc_url: &str,
        signature: &Signature,
    ) -> String {
        ToolboxEndpoint::compute_explorer_url(
            rpc_url,
            "tx",
            &signature.to_string(),
            &HashMap::new(),
        )
    }

    pub fn compute_explorer_simulation_url(
        rpc_url: &str,
        transaction_signatures: &[Signature],
        transaction_message_serialized: &[u8],
    ) -> String {
        let mut params = HashMap::new();
        params.insert(
            "signatures".to_string(),
            format!(
                "[{}]",
                transaction_signatures
                    .iter()
                    .map(|signature| format!("\"{}\"", signature))
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        );
        params.insert(
            "message".to_string(),
            ToolboxEndpoint::encode_base64(transaction_message_serialized),
        );
        ToolboxEndpoint::compute_explorer_url(
            rpc_url,
            "tx",
            "inspector",
            &params,
        )
    }

    fn compute_explorer_url(
        rpc_url: &str,
        category: &str,
        payload: &str,
        params: &HashMap<String, String>,
    ) -> String {
        let mut args = vec![];
        for (param_name, param_content) in params {
            args.push(format!(
                "{}={}",
                ToolboxEndpoint::encode_url(param_name),
                ToolboxEndpoint::encode_url(param_content)
            ));
        }
        match ToolboxEndpoint::get_cluster_from_url_or_moniker(rpc_url) {
            Some("mainnet-beta") => {},
            Some("devnet") => {
                args.push("cluster=devnet".to_string());
            },
            Some("testnet") => {
                args.push("cluster=testnet".to_string());
            },
            _ => {
                args.push("cluster=custom".to_string());
                args.push(format!(
                    "customUrl={}",
                    ToolboxEndpoint::encode_url(rpc_url)
                ));
            },
        };
        format!(
            "https://explorer.solana.com/{}/{}?{}",
            category,
            payload,
            args.join("&"),
        )
    }
}
