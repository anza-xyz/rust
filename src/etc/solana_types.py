import re


class SolanaType(object):
    PUBKEY = "Pubkey"
    ACCOUNT_INFO = "AccountInfo"

PUBKEY_REGEX = re.compile(r"^(solana_program::pubkey::Pubkey)")
ACCOUNT_INFO_REGEX = re.compile(r"^(solana_program::account_info::AccountInfo)")

SOLANA_TYPE_TO_REGEX = {
    SolanaType.PUBKEY: PUBKEY_REGEX,
    SolanaType.ACCOUNT_INFO: ACCOUNT_INFO_REGEX,
}

def classify_solana_type(type):
    for ty, regex in SOLANA_TYPE_TO_REGEX.items():
        if regex.match(type.name):
            return ty
