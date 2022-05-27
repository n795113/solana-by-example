use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::Message;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!(
            "usage: {} <path to program keypair> <path to signer keypair>",
            args[0],
        );
        std::process::exit(-1);
    }
    let program_keypair_path = &args[1];
    let signer_keypair_path = &args[2];

    // Establish connection
    let url = "http://localhost:8899".to_string();
    let commitment_config = CommitmentConfig::processed();
    let connection = RpcClient::new_with_commitment(url, commitment_config);

    // Get signer's Pubkey
    let payer = read_keypair_file(signer_keypair_path).map_err(|e| {
        panic!("failed to read keypair file ({}): ({})", signer_keypair_path, e);
    }).unwrap();

    // Get program's Pubkey (Program ID)
    let program_keypair = read_keypair_file(program_keypair_path).map_err(|e| {
        panic!(
            "failed to read program keypair file ({}): ({})",
            program_keypair_path, e
        );
    }).unwrap();

    // This will fail if the program is not deployed
    let program_info = connection
        .get_account(&program_keypair.pubkey())
        .expect("Fail to find the program");

    // On Solana every data are stored in Accounts
    // There are seversal types of accounts which we will cover later
    // So programs are stored as "binary" codes in "executable" accounts
    if !program_info.executable {
        panic!(
            "program with keypair ({}) is not executable",
            program_keypair_path
        );
    }

    // Make and send a transaction
    // Take this step as you send a HTTP request in web2
    let instruction = Instruction::new_with_bytes(
        program_keypair.pubkey(),
        &[],
        vec![],
    );
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let transaction = Transaction::new(
        &[&payer],
        message, 
        connection.get_recent_blockhash().unwrap().0
    );

    connection.send_and_confirm_transaction(&transaction);
}
