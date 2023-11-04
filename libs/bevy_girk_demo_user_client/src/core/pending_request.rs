//local shortcuts

//third-party shortcuts

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct PendingRequest(bevy_simplenet::RequestSignal);

impl PendingRequest
{
    pub(crate) fn new(new_req: bevy_simplenet::RequestSignal) -> Self
    {
        Self(new_req)
    }
}

impl std::ops::Deref for PendingRequest
{
    type Target = bevy_simplenet::RequestSignal;
    fn deref(&self) -> &bevy_simplenet::RequestSignal { &self.0 }
}

//-------------------------------------------------------------------------------------------------------------------
