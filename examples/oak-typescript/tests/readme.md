# 🧪 TypeScript Parser Test Suite

> ⚠️ **Tests No Longer Maintained**
> 
> This test directory is no longer maintained. All TypeScript parser test cases are now maintained downstream:
> https://github.com/nyar-vm/rusty-typescript/tree/dev

## 📝 Frequently Asked Questions

### Why aren't these tests maintained here anymore?

**Complexity challenges made maintenance difficult:**

- **Evolving TypeScript Syntax**: TypeScript's language specification changes frequently with new features, making it hard to maintain comprehensive test coverage across versions
- **Cross-Platform Compatibility**: Ensuring tests work consistently across different environments adds significant complexity
- **Integration Requirements**: Testing TypeScript parsing requires integration with multiple toolchains and frameworks
- **Scalability**: As the test suite grows, maintaining and running tests becomes increasingly resource-intensive
- **Focus Dilution**: Maintaining extensive tests alongside core parser development impacts the iteration speed of core features

### Why is downstream maintenance better?

**Benefits of the new approach:**

- **Specialized Focus**: The `rusty-typescript` repository is dedicated to TypeScript parsing, allowing for more focused and comprehensive test coverage
- **Unified Ecosystem**: All test cases are centralized in one location, providing a consistent testing environment and reducing duplication
- **Better Integration**: Tests are directly integrated with the parser implementation, enabling more efficient feedback loops
- **Community Collaboration**: A dedicated repository fosters community contributions and shared maintenance efforts
- **Scalable Infrastructure**: Purpose-built testing infrastructure better handles the growing complexity of TypeScript's feature set

### Where can I find the current tests?

All TypeScript parser test cases are now maintained at:
https://github.com/nyar-vm/rusty-typescript/tree/dev
