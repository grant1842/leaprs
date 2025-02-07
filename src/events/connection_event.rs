use leap_sys::LEAP_CONNECTION_EVENT;

use crate::ServiceState;

crate::leap_ref_struct!(
    #[doc = "  \\ingroup Structs"]
    #[doc = " Received from LeapPollConnection() when a connection to the Ultraleap Tracking Service is established."]
    #[doc = " @since 3.0.0"]
    ConnectionEvent,
    LEAP_CONNECTION_EVENT
);

impl<'a> ConnectionEvent<'a> {
    #[doc = " A combination of eLeapServiceDisposition flags. @since 3.1.3"]
    pub fn flags(&self) -> ServiceState {
        ServiceState::from_bits_truncate(self.handle.flags)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn connection_event_test() {
        let mut connection =
            Connection::create(ConnectionConfig::default()).expect("Failed to connect");
        connection.open().expect("Failed to open the connection");
        let _flags = connection
            .wait_for(|e| match e {
                Event::Connection(e) => Some(e.flags()),
                _ => None,
            })
            .expect("Did not receive a connection event.");
    }
}
