#!/bin/bash
set -exu

flatc -r -I include_test monster_test.fbs
rustfmt monster_test_generated.rs
