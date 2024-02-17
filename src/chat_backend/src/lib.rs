use std::ops::DerefMut;

use connections::{Message, MESSAGES};

pub mod connections;

#[ic_cdk::update]
fn message_update(message: connections::Message)
    {
        MESSAGES.with(|a|{
            let mut x = a.clone().deref_mut().to_vec();
            x.push(message);
        });
    }
#[ic_cdk::query]
fn message_query(message: connections::Message) -> connections::Message
    {
        message
    }
