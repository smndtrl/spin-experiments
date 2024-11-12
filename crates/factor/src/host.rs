use std::ops::Add;

use spin_core::wasmtime::component::Resource;
use world::{async_trait, smndtrl::experiments::{self as experiments, data::{Counter, Error}}};

use crate::InstanceState;


#[async_trait]
impl experiments::data::Host for InstanceState {
    fn convert_error(
        &mut self,
        err: Error,
    ) -> anyhow::Result<Error> {
        Ok(err)
    }
}

#[async_trait]
impl experiments::data::HostCounter for InstanceState {
    async fn new(&mut self, amount: u8) -> Result<Resource<Counter>, anyhow::Error> {
        self.counters
            .push(amount)
            .map(Resource::new_own)
            .map_err(|_| Error::Other("no more space for counters".into()).into())
    }

    // async fn add(&mut self, resource: Resource<Counter>, amount: u8) -> Result<Resource<Counter>, Error> {
    //     self.counters
    //         .get(resource).unwrap()
    //         .add(amount)
    // }

    // async fn dec(&mut self, resource: Resource<Counter>, amount: u8) -> Result<Resource<Counter>, Error> {
    //     todo!()
    // }

    async fn add(&mut self, resource: Resource<Counter>, amount: u8) -> Result<u8, anyhow::Error> {
        Ok(self.counters
            .get(resource.rep()).unwrap()
            .saturating_add(amount))
    }

    async fn dec(&mut self, resource: Resource<Counter>, amount: u8) -> Result<u8, anyhow::Error>  {
        Ok(self.counters
            .get(resource.rep()).unwrap()
            .saturating_sub(amount))
    }

    async fn drop(&mut self, resource: Resource<Counter>) -> Result<(), anyhow::Error> {
        todo!()
    }
}