#!/usr/bin/env just --justfile

alias s := start
alias d := development
alias f := format
alias l := lint

# Start the freaking project
start:
	deno run --allow-all mod.ts --config deno.json

# Start the local development
development:
	deno run --watch --allow-all mod.ts --config deno.json

# Formatting source codes
format:
	deno fmt --config deno.json

# Check for eslint errors
lint:
	deno lint --config deno.json
