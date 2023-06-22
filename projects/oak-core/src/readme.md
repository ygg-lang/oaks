# Performance & Incremental Parsing Guide

`oak-core` is designed with the philosophy that **explicit, manual optimization by the implementer outperforms generic, automated strategies.** While frameworks like Tree-sitter provide automatic incrementality, `oak-core` gives you the surgical tools to achieve much higher peak performance by leveraging domain knowledge of your specific grammar.

## Why oak is Fast

Oak achieves industry-leading performance through several key architectural decisions:

1.  **Surgical Incrementalism**: Unlike "all-or-nothing" incremental parsers, Oak allows you to define specific "Reuse Anchors" (`try_reuse`). This means you only pay for the complexity you actually use, and reuse hit rates can be optimized manually for your specific language structure.
2.  **Specialized `SyntaxArena`**: We use a custom bump allocator that outperforms general-purpose allocators (like `bumpalo`) by enforcing **Zero-Drop** for POD AST nodes, using **Fixed 8-byte Alignment**, and maintaining a **Thread-Local Chunk Pool** to minimize global allocator pressure.
3.  **SIMD-Accelerated Lexing**: Our lexer uses `portable_simd` to scan source text 32 bytes at a time. Operations like skipping whitespace or finding delimiters are effectively $O(N/32)$.
4.  **Separated Green/Red Trees**: By separating structural data (Green Nodes) from identity and position (Red Nodes), Oak maximizes memory sharing and cache locality during incremental updates.
5.  **Compiler-Level Hints**: We use `core_intrinsics` like `likely`/`unlikely` and `trusted_len` to guide the Rust compiler in optimizing the most critical hot paths in the parser loop.

## Common Performance Pitfalls

If you are not seeing the expected performance, check for these common traps:

1.  **Missing Reuse Anchors**: The most common mistake is not calling `try_reuse` at high-level nodes (e.g., functions, classes, blocks). Without these anchors, Oak defaults to a full re-parse.
2.  **Dirty Range Bloat**: If you merge multiple small, disconnected edits into a single large `TextEdit` span, you invalidate large sections of the tree unnecessarily. Keep your dirty ranges as tight as possible.
3.  **Lexer Bottleneck**: If your lexer re-scans the entire file on every edit, it becomes an $O(N)$ bottleneck that incremental parsing cannot solve. Use an anchor-based or stateful lexer that only re-lexes the changed portion.
4.  **Long-Distance Lookahead**: Using `try_reuse` on nodes that depend on context far outside their own span (e.g., a node whose meaning changes based on a token 1000 lines away) will cause frequent reuse failures or incorrect trees.
5.  **Arena Fragmentation**: Repeatedly re-parsing without successful reuse can lead to high memory churn. Ensure your grammar is structured to maximize `try_reuse` success at the highest possible levels.
6.  **Excessive Checkpointing**: While checkpoints are necessary, creating them for every single tiny node can add overhead. Focus checkpoints on meaningful nodes that are candidates for reuse.

## Requirements for Implementers

To achieve maximum performance and efficient incremental reuse, implementers must follow these requirements:

1.  **Explicit Reuse Anchors**: You must manually call `state.try_reuse(Kind)` at high-level grammar nodes that are likely to remain unchanged (e.g., Function definitions, Struct items, Statement blocks).
2.  **Stateless Lookahead**: Ensure that the nodes you attempt to reuse do not depend on long-distance context (lookahead) that exceeds the node's boundaries. If a node's meaning changes based on tokens far outside of it, incremental reuse may lead to incorrect trees.
3.  **Strict Checkpointing**: Always use `state.checkpoint()` and `state.finish_at()` to wrap nodes. This ensures the internal tree sink maintains a clean stack, which is critical for the `try_reuse` logic to correctly skip tokens.
4.  **Token Synchronization**: Your Lexer should produce tokens with accurate spans. Incremental parsing relies on mapping the new source offsets back to the old tree's coordinate system.

## Optimization Strategies

### 1. Granular Reuse
Instead of trying to reuse the entire source file, break your grammar into independent components. For example, in Rust:
- Try reuse at the **Item** level (Functions, Structs).
- Try reuse at the **Statement** level inside blocks.
- Try reuse at the **Expression** level for large literals or complex sub-trees.

### 2. Anchor-based Lexing
Implement a lexer that can "resync." When an edit occurs, find the nearest stable anchor (like a newline or a specific keyword) and only re-lex from that point until the token stream aligns with the previous generation.

### 3. Multi-segment Dirty Tracking
Currently, a single `dirty_range` is used. For advanced scenarios, implement a multi-segment tracker that allows the parser to "jump" over multiple edited areas while reusing the stable code in between.

### 4. Memory Locality (Node Promotion)
When a node is reused from the `Old Arena`, it stays there. If you have many generations of edits, your tree might be scattered across multiple arenas. Periodically performing a "Deep Clone" of reused nodes into the `Active Arena` can improve cache locality for subsequent traversals.

### 5. Bypass Validation (Unsafe Reuse)
For specific, high-frequency nodes that you know are strictly isolated, you can implement a "Fast Path" that reuses nodes based purely on their Kind and Length, skipping the expensive token-by-token validation.
