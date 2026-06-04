# HashMap-Based Graph Implementation in Rust

A flexible, generic graph data structure implemented using HashMaps for efficient lookups and edge management.

## Features

- **Generic ID Types**: Works with any ID type that implements `Clone`, `Hash`, and `Eq`
- **Debug Support**: Derives the `Debug` trait for easy console output during testing
- **Consistent Design**: Uses the same ID type for both nodes and edges
- **Clone-Based Architecture**: Uses `Clone` instead of `Copy` for flexibility with complex ID types (e.g., `String`)

## Design Overview

This implementation uses a map-based approach where both nodes and edges are stored in HashMap collections, providing:

- **O(1)** average-time complexity for node/edge lookups
- Flexible ID management without strict memory constraints
- Clean separation between node and edge storage

## ID Trait Requirements

The graph expects ID types to implement three core traits:

| Trait | Purpose |
|-------|---------|
| `Clone` | Allows IDs to be duplicated without requiring the stricter `Copy` trait |
| `Hash` | Enables HashMap key functionality |
| `Eq` | Ensures proper equality comparisons for ID lookups |

> **Note**: Using `Clone` instead of `Copy` allows the graph to work with larger data types like `String` without performance penalties or ownership issues.

## Core Design Principles

### 1. Unified ID System
Edge IDs and Node IDs share the same data type, maintaining consistency across the graph structure.

### 2. Debug Support
The entire graph layout can be printed to the console for testing and visualization purposes.

### 3. Clean Architecture
The file focuses solely on implementing the main graph system using a map-based approach.

## Basic Usage Example

```rust
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct NodeId(String);

#[derive(Debug)]
struct Graph<ID> 
where 
    ID: Clone + Hash + Eq,
{
    nodes: HashMap<ID, NodeData>,
    edges: HashMap<ID, EdgeData<ID>>,
}

struct NodeData {
    // Node-specific fields
}

struct EdgeData<ID> {
    from: ID,
    to: ID,
    // Edge-specific fields
}

impl<ID> Graph<ID> 
where 
    ID: Clone + Hash + Eq,
{
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    
    // Additional graph methods...
}
