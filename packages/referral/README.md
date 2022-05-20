## SingleSidedReferral

Referral controller with standard specs,

- `set_ref`
  - Set referral of a specific address.
- `ref_chains`
  - Get a referral chain of a specific address. Default depth is 3.
- `ref_of`
  - Get a referrer of a specific address.
- `has_ref`
  - Get a boolean that state a specific address has a referrer or not.

## IndexedReferral

Referral controller extended with referred address indexer, i.e. specific address can query list of addresses that address have referred.

- `all_referred_of`
  - Get all referral of a specific address.

