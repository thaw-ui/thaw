# Development guide

### Code style

It is recommended to use the Rust style instead of the functional style in the newly added reactive code.

```rust
RwSignal::new(12) // ✅
create_rw_signal(12) // 🙅

Memo::new(|_| {}) // ✅
create_memo(|_| {}) // 🙅

Effect::new(|_| {}) // ✅
create_effect(|_| {}) // 🙅
```