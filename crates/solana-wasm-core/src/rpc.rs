//! Minimal Solana JSON-RPC client surface for invoice status checks.
//!
//! Transport is injected via [`HttpTransport`] so host tests can mock HTTP
//! and WASM plugins can plug in `waki` (or similar) without this crate
//! depending on any network stack.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt;

/// Transport error for JSON-RPC calls.
#[derive(Debug, Clone)]
pub struct RpcError {
    pub message: String,
}

impl RpcError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rpc error: {}", self.message)
    }
}

impl std::error::Error for RpcError {}

/// Pluggable HTTP POST JSON transport.
pub trait HttpTransport {
    fn post_json(&self, url: &str, body: &Value) -> Result<Value, RpcError>;
}

/// Thin JSON-RPC 2.0 client.
pub struct RpcClient<T: HttpTransport> {
    pub endpoint: String,
    pub http: T,
}

impl<T: HttpTransport> RpcClient<T> {
    pub fn new(endpoint: impl Into<String>, http: T) -> Self {
        Self {
            endpoint: endpoint.into(),
            http,
        }
    }

    /// Call `getSignaturesForAddress` (JSON-RPC 2.0).
    pub fn get_signatures_for_address(
        &self,
        address: &str,
        limit: u64,
    ) -> Result<Vec<SignatureInfo>, RpcError> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSignaturesForAddress",
            "params": [
                address,
                {
                    "limit": limit
                }
            ]
        });

        let resp = self.http.post_json(&self.endpoint, &body)?;

        if let Some(err) = resp.get("error") {
            let msg = err
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown rpc error");
            return Err(RpcError::new(msg));
        }

        let result = resp
            .get("result")
            .ok_or_else(|| RpcError::new("missing result field"))?;

        let list = result
            .as_array()
            .ok_or_else(|| RpcError::new("result is not an array"))?;

        let mut out = Vec::with_capacity(list.len());
        for item in list {
            out.push(SignatureInfo::from_value(item)?);
        }
        Ok(out)
    }

    /// Call `getBalance` — returns lamports for a system account.
    pub fn get_balance(&self, address: &str) -> Result<u64, RpcError> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBalance",
            "params": [address]
        });
        let resp = self.http.post_json(&self.endpoint, &body)?;
        Self::rpc_result(&resp)?;
        resp.get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_u64())
            .ok_or_else(|| RpcError::new("getBalance: missing result.value"))
    }

    /// Sum UI amounts of SPL token accounts for `owner` filtered by `mint`
    /// using `getTokenAccountsByOwner` + `jsonParsed`.
    pub fn get_token_ui_balance(&self, owner: &str, mint: &str) -> Result<String, RpcError> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTokenAccountsByOwner",
            "params": [
                owner,
                { "mint": mint },
                { "encoding": "jsonParsed" }
            ]
        });
        let resp = self.http.post_json(&self.endpoint, &body)?;
        Self::rpc_result(&resp)?;
        let arr = resp
            .get("result")
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_array())
            .ok_or_else(|| RpcError::new("getTokenAccountsByOwner: missing result.value"))?;

        let mut total: f64 = 0.0;
        let mut found = false;
        for item in arr {
            if let Some(ui) = item
                .pointer("/account/data/parsed/info/tokenAmount/uiAmount")
                .and_then(|v| v.as_f64())
            {
                total += ui;
                found = true;
            } else if let Some(s) = item
                .pointer("/account/data/parsed/info/tokenAmount/uiAmountString")
                .and_then(|v| v.as_str())
            {
                if let Ok(x) = s.parse::<f64>() {
                    total += x;
                    found = true;
                }
            }
        }
        if !found {
            return Ok("0".to_string());
        }
        // Trim trailing zeros for display
        let s = format!("{total:.6}");
        Ok(trim_trailing_zeros(&s))
    }

    fn rpc_result(resp: &Value) -> Result<(), RpcError> {
        if let Some(err) = resp.get("error") {
            let msg = err
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown rpc error");
            return Err(RpcError::new(msg));
        }
        Ok(())
    }
}

fn trim_trailing_zeros(s: &str) -> String {
    if !s.contains('.') {
        return s.to_string();
    }
    let t = s.trim_end_matches('0').trim_end_matches('.');
    if t.is_empty() {
        "0".into()
    } else {
        t.to_string()
    }
}

/// Compact signature info returned by `getSignaturesForAddress`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignatureInfo {
    pub signature: String,
    pub slot: u64,
    pub err: Option<Value>,
    pub memo: Option<String>,
    pub block_time: Option<i64>,
    pub confirmation_status: Option<String>,
}

impl SignatureInfo {
    fn from_value(v: &Value) -> Result<Self, RpcError> {
        let signature = v
            .get("signature")
            .and_then(|s| s.as_str())
            .ok_or_else(|| RpcError::new("signature missing"))?
            .to_string();
        let slot = v
            .get("slot")
            .and_then(|s| s.as_u64())
            .ok_or_else(|| RpcError::new("slot missing"))?;
        let err = v.get("err").cloned().filter(|e| !e.is_null());
        let memo = v
            .get("memo")
            .and_then(|m| if m.is_null() { None } else { m.as_str().map(|s| s.to_string()) });
        let block_time = v.get("blockTime").and_then(|t| t.as_i64());
        let confirmation_status = v
            .get("confirmationStatus")
            .and_then(|c| c.as_str())
            .map(|s| s.to_string());

        Ok(Self {
            signature,
            slot,
            err,
            memo,
            block_time,
            confirmation_status,
        })
    }

    /// True when the transaction succeeded (err is null / absent).
    pub fn is_success(&self) -> bool {
        self.err.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockHttp {
        response: RefCell<Value>,
        last_body: RefCell<Option<Value>>,
    }

    impl HttpTransport for MockHttp {
        fn post_json(&self, _url: &str, body: &Value) -> Result<Value, RpcError> {
            *self.last_body.borrow_mut() = Some(body.clone());
            Ok(self.response.borrow().clone())
        }
    }

    #[test]
    fn get_signatures_parses_result() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": [
                {
                    "signature": "Sig111",
                    "slot": 42,
                    "err": null,
                    "memo": "PIX|BRL|inv-001|x",
                    "blockTime": 1_700_000_000,
                    "confirmationStatus": "finalized"
                }
            ]
        });
        let http = MockHttp {
            response: RefCell::new(response),
            last_body: RefCell::new(None),
        };
        let client = RpcClient::new("https://api.mainnet-beta.solana.com", http);
        let sigs = client
            .get_signatures_for_address("SomeRef111111111111111111111111111", 5)
            .unwrap();
        assert_eq!(sigs.len(), 1);
        assert_eq!(sigs[0].signature, "Sig111");
        assert!(sigs[0].is_success());
        assert_eq!(sigs[0].memo.as_deref(), Some("PIX|BRL|inv-001|x"));

        let body = client.http.last_body.borrow().clone().unwrap();
        assert_eq!(body["method"], "getSignaturesForAddress");
        assert_eq!(body["params"][1]["limit"], 5);
    }

    #[test]
    fn rpc_error_propagates() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": { "code": -32600, "message": "Invalid Request" }
        });
        let http = MockHttp {
            response: RefCell::new(response),
            last_body: RefCell::new(None),
        };
        let client = RpcClient::new("http://localhost", http);
        let err = client.get_signatures_for_address("x", 1).unwrap_err();
        assert!(err.message.contains("Invalid Request"));
    }
}
