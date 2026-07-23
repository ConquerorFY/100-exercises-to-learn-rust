/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
use tokio::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let msg = receiver.recv().await.unwrap() {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel(1024);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel(1024);
        let (response_sender, mut response_receiver) = mpsc::channel(1024);
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .await
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}


// EXPLANATION: 
// The core issue is that the code uses **synchronous, blocking primitives (`std::sync::mpsc`) inside an asynchronous environment (`Tokio`)**.

// ---

// ### The Sequence of Events Leading to a Deadlock

// 1. **Blocking the Async Runtime Thread:**
// Inside the test, `response_receiver.recv().unwrap()` is called synchronously. This blocks the current Tokio worker thread until it receives a message.
// 2. **Starvation or Deadlock:**
// `std::mpsc::Receiver::recv()` is a **blocking OS-level call**, not an async `.await`. When you block a Tokio worker thread with a synchronous receive call, that thread cannot execute other asynchronous tasks (like your `pong` task).
// 3. **The Buffer Trap:**
// Standard channels (`mpsc::channel()`) have unbounded or internally buffered behavior, but mixing them with synchronous blocking inside an async runtime frequently causes threads to lock up each other's execution context, resulting in a standstill where neither side can make progress.

// ---

// ### How to Rewrite it Using Tokio's Channels

// To fix this, you need to use **`tokio::sync::mpsc`** and **asynchronous `.recv().await**` methods instead of standard blocking calls.

// Here is the rewritten, non-deadlocking version:

// ```rust
// use tokio::sync::mpsc;

// pub struct Message {
//     payload: String,
//     response_channel: mpsc::Sender<Message>,
// }

// /// Replies with `pong` to any message it receives, setting up a new
// /// channel to continue communicating with the caller.
// pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
//     while let Some(msg) = receiver.recv().await {
//         println!("Pong received: {}", msg.payload);
//         // Use a tokio channel here instead
//         let (sender, new_receiver) = mpsc::channel(1);
//         msg.response_channel
//             .send(Message {
//                 payload: "pong".into(),
//                 response_channel: sender,
//             })
//             .await
//             .unwrap();
//         receiver = new_receiver;
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::{pong, Message};
//     use tokio::sync::mpsc;

//     #[tokio::test]
//     async fn ping() {
//         // Tokio channels require a buffer size (e.g., 1)
//         let (sender, receiver) = mpsc::channel(1);
//         let (response_sender, mut response_receiver) = mpsc::channel(1);
        
//         sender
//             .send(Message {
//                 payload: "pong".into(),
//                 response_channel: response_sender,
//             })
//             .await
//             .unwrap();

//         tokio::spawn(pong(receiver));

//         // Use .recv().await instead of synchronous .recv().unwrap()
//         let answer = response_receiver.recv().await.unwrap().payload;
//         assert_eq!(answer, "pong");
//     }
// }

// ```