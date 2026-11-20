#!/usr/bin/env python3
"""
Validate committed error-spec-snapshot.json against actual contract error definitions. (#768)
"""
import json, re, sys, os

def check_error_snapshot():
    spec_path = 'contract-spec.json'
    snapshot_path = 'contracts/error-spec-snapshot.json'
    error_rs_path = 'neurowealth-vault/contracts/vault/src/error.rs'

    if not os.path.exists(spec_path) or not os.path.exists(snapshot_path):
        print(f'Error: Spec or snapshot missing.')
        return 1

    with open(spec_path) as f:
        spec = json.load(f)
    with open(snapshot_path) as f:
        snapshot = json.load(f)

    spec_errors = {k: v['code'] for k, v in spec.get('errors', {}).items()}

    mismatches = []
    for k, v in snapshot.items():
        if k not in spec_errors:
            mismatches.append(f'Missing in spec: {k} (expected code {v})')
        elif spec_errors[k] != v:
            mismatches.append(f'Discriminant mismatch for {k}: snapshot={v}, spec={spec_errors[k]}')

    for k, v in spec_errors.items():
        if k not in snapshot:
            mismatches.append(f'New variant in spec not in snapshot: {k} with code {v}')

    if mismatches:
        print('Error spec snapshot validation FAILED with mismatches:')
        for m in mismatches:
            print(f'  - {m}')
        return 1

    print(f'Error spec snapshot validation PASSED: {len(snapshot)} typed error variants verified.')
    return 0

if __name__ == '__main__':
    sys.exit(check_error_snapshot())
