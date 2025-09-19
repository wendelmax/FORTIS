//! Módulo de Armazenamento Distribuído Eficiente
//! 
//! Implementa armazenamento distribuído usando DHT (Distributed Hash Table)
//! e IPFS, seguindo os princípios do Prof. Marcos Simplicio de usar
//! tecnologias apropriadas para cada problema e evitar replicação
//! completa desnecessária.

pub mod distributed_storage;
// pub mod ipfs_client;
// pub mod dht_client;
// pub mod local_cache;

pub use distributed_storage::*;
