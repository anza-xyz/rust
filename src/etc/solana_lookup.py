from solana_providers import *
from solana_types import SolanaType, classify_solana_type


def summary_lookup(valobj, dict):
    # type: (SBValue, dict) -> str
    """Returns the summary provider for the given value"""
    solana_type = classify_solana_type(valobj.GetType())
    if solana_type == SolanaType.PUBKEY:
        return PubkeySummaryProvider(valobj, dict)
    if solana_type == SolanaType.ACCOUNT_INFO:
        return AccountInfoSummaryProvider(valobj, dict)
