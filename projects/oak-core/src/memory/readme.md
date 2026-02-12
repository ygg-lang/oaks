# Syntax Arena

Specialized high-performance bump allocator for AST nodes.

This module implements a `SyntaxArena`, which is a bump allocator optimized for
short-lived, POD (Plain Old Data) objects like AST nodes. It minimizes allocation
overhead by using a chunk-based strategy and a thread-local pool for recycling memory.

## Architecture & Memory Layout

The arena allocates memory in "chunks" (defaulting to 64KB).
- **Current Chunk:** The active chunk where new allocations are "bumped" from a pointer.
- **Full Chunks:** A list of previously exhausted chunks that will be recycled together.
- **Chunk Pool:** A thread-local storage that keeps up to 64 standard-sized chunks to
  avoid hitting the global allocator for every new arena.

## Design Rationale: Why not `bumpalo`?

While general-purpose bump allocators like `bumpalo` are excellent, they are designed to
handle arbitrary alignment, `Drop` checking, and varied allocation patterns.
Our `SyntaxArena` is intentionally specialized for AST nodes with the following advantages:

1. **Zero-Drop Enforcement:** We assume all allocated types are POD and do not require
   dropping. This eliminates the need for a "drop list" or any drop-tracking overhead.
2. **Fixed Alignment:** Optimized for a constant 8-byte alignment (sufficient for 64-bit
   pointers and primitives), simplifying pointer arithmetic and reducing branching.
3. **Thread-Local Pooling:** We reuse memory chunks via a thread-local pool (`CHUNK_POOL`)
   to minimize the overhead of hitting the global allocator between parser runs.
4. **Minimal Overhead:** Focused purely on the needs of our high-performance parser by
   stripping away features not required for AST construction.

## Safety Invariants

- All types allocated in the arena **must not** implement `Drop` (or their `Drop`
  implementation will be ignored).
- The arena is not `Sync` or `Send` in a way that allows cross-thread allocation,
  though the underlying chunks are managed safely via thread-local storage.
- Memory is only reclaimed when the entire `SyntaxArena` is dropped.
