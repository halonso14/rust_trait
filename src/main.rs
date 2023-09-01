mod common;
mod external_module;
mod module_a;
mod module_b;

use std::sync::Arc;

use async_trait::async_trait;
use tokio::task;

use crate::{common::TestCase, module_a::do_a};

trait ISyncTrait {
    // async fn do_something(&self);
    fn do_something(&self);
}

#[async_trait]
trait IAsyncTrait: Send + Sync {
    async fn do_something(&self);
}

#[derive(Clone)]
struct AsyncTraitWithClone;

#[async_trait]
impl IAsyncTrait for AsyncTraitWithClone {
    async fn do_something(&self) {
        println!("TestCase::Success => {:?}", do_a(TestCase::Success));
        println!(
            "TestCase::FailFromTop1 => {:?}",
            do_a(TestCase::FailFromTop1)
        );
        println!(
            "TestCase::FailFromTop2 => {:?}",
            do_a(TestCase::FailFromTop2)
        );
        println!(
            "TestCase::FailFromMiddle1 => {:?}",
            do_a(TestCase::FailFromMiddle1)
        );
        println!(
            "TestCase::FailFromMiddle2 => {:?}",
            do_a(TestCase::FailFromMiddle2)
        );
        println!(
            "TestCase::FailFromBottom1 => {:?}",
            do_a(TestCase::FailFromBottom1)
        );
        println!(
            "TestCase::FailFromBottom2 => {:?}",
            do_a(TestCase::FailFromBottom2)
        );
        println!("AsyncTraitWithClone is doing something asynchronously");
    }
}

struct AsyncTraitWithoutClone;

#[async_trait]
impl IAsyncTrait for AsyncTraitWithoutClone {
    async fn do_something(&self) {
        println!("AsyncTraitWithoutClone is doing something asynchronously");
    }
}

#[derive(Clone)]
struct StructWithClone {
    // async_trait_with_clone: Box<dyn IAsyncTrait>,
    async_trait_with_clone: Arc<dyn IAsyncTrait>,
}

impl StructWithClone {
    fn new(async_trait_with_clone: Arc<dyn IAsyncTrait>) -> Self {
        Self {
            async_trait_with_clone,
        }
    }

    async fn perform_action(&self) {
        self.async_trait_with_clone.do_something().await;
    }
}

struct StructWithoutClone {
    async_trait_without_clone: Box<dyn IAsyncTrait>,
}

impl StructWithoutClone {
    fn new(async_trait_without_clone: Box<dyn IAsyncTrait>) -> Self {
        Self {
            async_trait_without_clone,
        }
    }

    async fn perform_action(&self) {
        self.async_trait_without_clone.do_something().await;
    }
}

#[tokio::main]
async fn main() {
    let async_trait_struct_with_clone = AsyncTraitWithClone;
    let async_trait_struct_without_clone = AsyncTraitWithoutClone;

    let arc_struct: Arc<dyn IAsyncTrait> = Arc::new(async_trait_struct_with_clone);
    let box_struct: Box<dyn IAsyncTrait> = Box::new(async_trait_struct_without_clone);

    let struct_requires_clone = StructWithClone::new(arc_struct);
    let struct_does_not_requires_clone = StructWithoutClone::new(box_struct);

    let task1 = task::spawn(async move {
        struct_requires_clone.clone().perform_action().await;
    });

    let task2 = task::spawn(async move {
        struct_does_not_requires_clone.perform_action().await;
    });

    task1.await.unwrap();
    task2.await.unwrap();
}
