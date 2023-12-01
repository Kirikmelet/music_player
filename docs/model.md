# Application Model

This application should apply an actor/DOM like model.

example implementation

```rust
use async_trait::async_trait;
use ratatui::Frame;

#[async_trait]
pub trait Page {
    type Message;
    type State;
    async fn update(&mut self, msg: Self::Message) -> Result<(), u8>;
    fn render(&mut self, frame: &mut Frame);
    fn set_state(&mut self, new_state: Self::State);
    fn get_state(&self) -> Self::State;
}

```

This would conform somewhat with ELM architecture.
As the only glaring misconformity is that the update method
is *mutable*.