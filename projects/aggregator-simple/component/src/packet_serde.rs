use crate::bindings::wavs::aggregator::aggregator::{EnvelopeSignature, Packet};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializablePacket {
    pub service_name: String,
    pub workflow_id: String,
    pub envelope_event_id: Vec<u8>,
    pub envelope_ordering: Vec<u8>,
    pub envelope_payload: Vec<u8>,
    pub signature_data: Vec<u8>,
}

impl From<Packet> for SerializablePacket {
    fn from(packet: Packet) -> Self {
        let signature_data = match packet.signature {
            EnvelopeSignature::Secp256k1(sig) => sig.signature_data,
        };

        SerializablePacket {
            service_name: packet.service.name,
            workflow_id: packet.workflow_id,
            envelope_event_id: packet.envelope.event_id,
            envelope_ordering: packet.envelope.ordering,
            envelope_payload: packet.envelope.payload,
            signature_data,
        }
    }
}
