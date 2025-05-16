use jup_ag_sdk::JupiterClient;

#[cfg(test)]
mod tests {
    #[test]
    pub fn test() {}
}

fn test_jupiter_client_creation() {
    let base_url = "https://lite-api.jup.ag";
    let client = JupiterClient::new(base_url);

    // assert_eq!(client.base_url, base_url);
}
