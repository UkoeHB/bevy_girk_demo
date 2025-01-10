use bevy_cobweb::prelude::*;
use bevy_simplenet::RequestSignal;

//-------------------------------------------------------------------------------------------------------------------

#[derive(ReactComponent, Debug, Clone)]
pub(crate) struct PendingRequest(RequestSignal);

impl PendingRequest
{
    pub(crate) fn new(new_req: RequestSignal) -> Self
    {
        Self(new_req)
    }
}

impl Deref for PendingRequest
{
    type Target = RequestSignal;
    fn deref(&self) -> &RequestSignal
    {
        &self.0
    }
}

//-------------------------------------------------------------------------------------------------------------------
