use serde::{Serialize, de::DeserializeOwned};
use std::future::Future;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use uuid::Uuid;
pub struct Action {
    #[allow(unused)]
    pub session_id: Uuid,
    handle: JoinHandle<anyhow::Result<()>>,
    sender: mpsc::Sender<serde_json::Value>,
}

impl Action {
    pub fn new<F, Fut>(
        session_id: Uuid,
        receiver: mpsc::Sender<serde_json::Value>,
        close_listener: oneshot::Receiver<()>,
        task_func: F,
    ) -> Self
    where
        F: FnOnce(
                mpsc::Receiver<serde_json::Value>,
                mpsc::Sender<serde_json::Value>,
                oneshot::Receiver<()>,
            ) -> Fut
            + Send
            + 'static,
        Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
    {
        let (inbound_sender, inbound_receiver) =
            mpsc::channel::<serde_json::Value>(32);
        let fut = task_func(inbound_receiver, receiver, close_listener);
        let handle = tokio::spawn(async move { fut.await });
        Action {
            session_id,
            handle,
            sender: inbound_sender,
        }
    }

    pub async fn resume(&self, msg: serde_json::Value) -> anyhow::Result<()> {
        self.sender.send(msg).await?;
        Ok(())
    }

    pub fn abort(&self) {
        self.handle.abort();
    }
}

#[sealed::sealed]
pub trait InitAction<Input, Output>
where
    Input: 'static + Send + Sync + DeserializeOwned,
    Output: 'static + Send + Sync + Serialize + DeserializeOwned,
{
    fn init_action(
        self,
        session_id: Uuid,
        receiver: mpsc::Sender<Output>,
    ) -> (Action, oneshot::Sender<()>);
}

/// Example usage:
/// ```rust
/// use rmcs_actions_service::service::action::OnceShot;
/// let action = OnceShot(|| async { Ok("done".to_string()) });
/// ```
#[allow(dead_code)]
pub struct OnceShot<F>(pub F);

// For once-shot active actions
#[sealed::sealed]
impl<F, Input, Output, OutputFut> InitAction<Input, Output> for OnceShot<F>
where
    F: FnOnce() -> OutputFut + Send + 'static,
    OutputFut: Future<Output = anyhow::Result<Output>> + Send + 'static,
    Input: 'static + Send + Sync + DeserializeOwned,
    Output: 'static + Send + Sync + Serialize + DeserializeOwned,
{
    fn init_action(
        self,
        session_id: Uuid,
        receiver: mpsc::Sender<Output>,
    ) -> (Action, oneshot::Sender<()>) {
        let (close_sender, close_listener) = oneshot::channel::<()>();
        let (json_sender, mut json_receiver) = mpsc::channel::<serde_json::Value>(32);
        
        // Spawn output converter task: JSON -> Typed Output
        tokio::spawn(async move {
            while let Some(json_value) = json_receiver.recv().await {
                match serde_json::from_value::<Output>(json_value) {
                    Ok(typed_output) => {
                        if receiver.send(typed_output).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to deserialize output: {}", e);
                        break;
                    }
                }
            }
        });
        
        (  
            Action::new(
                session_id,
                json_sender,
                close_listener,
                move |_inbound_receiver, outbound_sender, _close_listener| {
                    let fut = self.0();
                    async move {
                        let result = fut.await?;
                        let json_value = serde_json::to_value(result)?;
                        outbound_sender.send(json_value).await?;
                        Ok(())
                    }
                },
            ),
            close_sender,
        )
    }
}

/// Example usage:
/// ```rust
/// use rmcs_actions_service::service::action::Responsive;
/// let action = Responsive(|input: String| async move { Ok(format!("echo: {}", input)) });
/// ```
#[allow(dead_code)]
pub struct Responsive<F>(pub F);

// For simple responsive actions
#[sealed::sealed]
impl<F, Input, Output, OutputFut> InitAction<Input, Output> for Responsive<F>
where
    F: Fn(Input) -> OutputFut + Send + 'static,
    OutputFut: Future<Output = anyhow::Result<Output>> + Send + 'static,
    Input: 'static + Send + Sync + DeserializeOwned,
    Output: 'static + Send + Sync + Serialize + DeserializeOwned,
{
    fn init_action(
        self,
        session_id: Uuid,
        receiver: mpsc::Sender<Output>,
    ) -> (Action, oneshot::Sender<()>) {
        let (close_sender, close_listener) = oneshot::channel::<()>();
        let (json_sender, mut json_receiver) = mpsc::channel::<serde_json::Value>(32);
        
        // Spawn output converter task: JSON -> Typed Output
        tokio::spawn(async move {
            while let Some(json_value) = json_receiver.recv().await {
                match serde_json::from_value::<Output>(json_value) {
                    Ok(typed_output) => {
                        if receiver.send(typed_output).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to deserialize output: {}", e);
                        break;
                    }
                }
            }
        });
        
        (
            Action::new(
                session_id,
                json_sender,
                close_listener,
                move |mut inbound_receiver,
                      outbound_sender,
                      _close_listener| {
                    let f = self.0;
                    async move {
                        while let Some(json_input) = inbound_receiver.recv().await {
                            let input: Input = serde_json::from_value(json_input)?;
                            let result = f(input).await?;
                            let json_output = serde_json::to_value(result)?;
                            outbound_sender.send(json_output).await?;
                        }
                        Ok(())
                    }
                },
            ),
            close_sender,
        )
    }
}

pub struct Streaming<F>(pub F);

// For streaming actions
#[sealed::sealed]
impl<F, Input, Output, OutputFut> InitAction<Input, Output> for Streaming<F>
where
    F: Fn(
            mpsc::Receiver<Input>,
            mpsc::Sender<Output>,
            oneshot::Receiver<()>,
        ) -> OutputFut
        + Send
        + 'static,
    OutputFut: Future<Output = anyhow::Result<()>> + Send + 'static,
    Input: 'static + Send + Sync + DeserializeOwned,
    Output: 'static + Send + Sync + Serialize + DeserializeOwned,
{
    fn init_action(
        self,
        session_id: Uuid,
        receiver: mpsc::Sender<Output>,
    ) -> (Action, oneshot::Sender<()>) {
        let (close_sender, close_listener) = oneshot::channel::<()>();
        let (typed_in_sender, typed_in_receiver) = mpsc::channel::<Input>(32);
        let (typed_out_sender, mut typed_out_receiver) = mpsc::channel::<Output>(32);
        let (json_out_sender, mut json_out_receiver) = mpsc::channel::<serde_json::Value>(32);
        
        let json_out_sender_clone = json_out_sender.clone();
        
        // Spawn output converter task: Typed Output -> JSON + External Typed receiver
        tokio::spawn(async move {
            while let Some(typed_output) = typed_out_receiver.recv().await {
                match serde_json::to_value(&typed_output) {
                    Ok(json_value) => {
                        // Send typed output to external receiver
                        if receiver.send(typed_output).await.is_err() {
                            break;
                        }
                        // Send JSON for internal Action output
                        if json_out_sender_clone.send(json_value).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to serialize output: {}", e);
                        break;
                    }
                }
            }
        });
        
        (
            Action::new(
                session_id,
                json_out_sender,
                close_listener,
                move |mut json_in_receiver, json_outbound_sender, user_close_listener| {
                    // Spawn input converter task: JSON -> Typed Input
                    tokio::spawn(async move {
                        while let Some(json_value) = json_in_receiver.recv().await {
                            match serde_json::from_value::<Input>(json_value) {
                                Ok(typed_input) => {
                                    if typed_in_sender.send(typed_input).await.is_err() {
                                        break;
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to deserialize input: {}", e);
                                    break;
                                }
                            }
                        }
                    });
                    
                    // Forward JSON outputs to the outbound sender provided by Action
                    tokio::spawn(async move {
                        while let Some(json_value) = json_out_receiver.recv().await {
                            if json_outbound_sender.send(json_value).await.is_err() {
                                break;
                            }
                        }
                    });
                    
                    let fut = self.0(
                        typed_in_receiver,
                        typed_out_sender,
                        user_close_listener,
                    );
                    async move { fut.await }
                },
            ),
            close_sender,
        )
    }
}
