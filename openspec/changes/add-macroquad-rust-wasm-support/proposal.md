# Proposal

## Problem

The studio currently documents Unity, Godot, Unreal, and Cocos Creator engine
paths, but it has no first-class workflow for lightweight Rust games compiled to
WebAssembly with Macroquad.

## Goal

Make Macroquad discoverable and actionable from engine selection through local
development, native validation, WASM release builds, and static web deployment.

## Non-goals

- Do not create a sample game in this repository.
- Do not select Macroquad as the repository's active engine.
- Do not add hosting credentials or deploy a website.
- Do not require wasm-bindgen for Macroquad's standard miniquad loader path.

## Acceptance Criteria

1. `/setup-engine macroquad` has explicit Rust/WASM guidance.
2. A reusable Macroquad skill covers scaffold, architecture, validation, web
   packaging, and deployment concerns.
3. A Macroquad specialist agent is present in programming-team discovery docs.
4. Engine reference docs pin the verified Macroquad release and official build
   contract.
5. Skill and agent catalogs remain structurally valid and internally consistent.

