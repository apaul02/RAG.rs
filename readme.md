# 🦀 RAG.rs

A blazing-fast, from-scratch Retrieval-Augmented Generation (RAG) pipeline built entirely in Rust.

Unlike off-the-shelf RAG frameworks, this project relies on a custom-built state machine for context-aware document parsing, a raw floating-point math implementation for vector search, and asynchronous API batching for high-performance embeddings. It uses the Google Gemini API for both embeddings and text generation.

## ✨ Features

* **Context-Aware Semantic Chunker:** A custom state machine that parses raw text while preserving markdown tables, lists, and attaching header contexts to every isolated chunk.
* **Async API Batching:** Utilizes `tokio` and `reqwest` to batch embedding requests, bypassing rate limits and drastically reducing network bottleneck time.
* **Raw Vector Search Engine:** Implements pure math (Cosine Similarity) to compare query vectors against the document database, returning the most semantically relevant chunks.
* **Type-Safe JSON Translation:** Uses `serde` to strictly map complex, deeply nested AI payloads into memory-safe Rust structs.
* **Generative Synthesis:** Connects the retrieved context to `gemini-3-flash-preview` to generate grounded, conversational answers.

## 🏗️ Architecture

The codebase is split into modular, responsibility-driven files:

* `src/main.rs`: The async entry point that orchestrates the pipeline.
* `src/chunker.rs`: The state-machine text parser and sliding-window chunker.
* `src/embedder.rs`: The async network client for batch-fetching vector embeddings.
* `src/search.rs`: The mathematical brain calculating Cosine Similarity and ranking `f32` vectors.
* `src/generator.rs`: The final LLM client that synthesizes context into human-readable answers.
* `src/models.rs`: The data blueprints and `serde` translation layer.

## 🚀 Getting Started

### Prerequisites

* [Rust & Cargo](https://rustup.rs/) (v1.70+)
* A Google Gemini API Key (Free tier via Google AI Studio works perfectly)

### Setup

1. **Clone the repository** (or navigate to your project folder).
2. **Set up your environment variables:**
   Create a `.env` file in the root of the project and add your API key:

   ```env
   GEMINI_API_KEY=your_actual_api_key_here
   ```

3. **Provide Data source**: Ensure you have a text file named `distributed.txt` in the root of your project. This is the knowledge base the AI will read from.

### Running The Pipeline

```bash
cargo run
```

The system will automatically:

1. Parse and chunk `distributed.txt`.
2. Fetch vector embeddings for all chunks.
3. Embed your hardcoded query.
4. Search the document via Cosine Similarity.
5. Stream the final, generated answer to your terminal.

## 📦 Dependecies

* tokio - Async runtime
* reqwest - HTTP network client
* serde and serde_json - JSON parsing
* donenvy - env variables setup
* uuid - unique id generator
