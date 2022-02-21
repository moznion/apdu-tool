use ts_102_221::class::{ClassTypeForStandardLogicalChannels, new_standard_class, SecureMessagingIndicationForStandardLogicalChannels};
use ts_102_221::command_apdu::new_command_apdu;
use ts_102_221::instruction::{SelectFile, UpdateBinary};
use ts_131_102::access_technology_identifier::new_access_technology_identifier;
use ts_131_102::access_technology_identifier::RadioAccessNetworkArchitecture::{EUTRAN, UTRAN};
use ts_131_102::binaryable::Binaryable;
use ts_131_102::identifier_providable::IdentifierProvidable;
use ts_131_102::operator_controlled_plmn_selector_with_access_technology::new_operator_controlled_plmn_selector_with_access_technology;
use ts_131_102::plmn::new_plmn;

fn main() { // TODO be CLI tool
    let plmns_info = [
        (new_plmn(310, 410).unwrap(), new_access_technology_identifier(Vec::from([UTRAN, EUTRAN]), Vec::new()))
    ];
    let oplmnwact = new_operator_controlled_plmn_selector_with_access_technology(&plmns_info);

    let identifier = oplmnwact.get_identifier();
    let class = new_standard_class(ClassTypeForStandardLogicalChannels::ISOIEC7816_4, SecureMessagingIndicationForStandardLogicalChannels::NoSM, 0).unwrap();
    let instruction = SelectFile{};
    let apdu = new_command_apdu(&class, &instruction, 0x00, 0x04, Option::None, Option::Some(&identifier));
    for b in apdu.to_bytes().unwrap() {
        print!("{:02x}", b);
    }
    println!();

    let oplmnwact_bytes = oplmnwact.to_bytes();
    let class = new_standard_class(ClassTypeForStandardLogicalChannels::ISOIEC7816_4, SecureMessagingIndicationForStandardLogicalChannels::NoSM, 0).unwrap();
    let instruction = UpdateBinary{};
    let apdu = new_command_apdu(&class, &instruction, 0x00, 0x05, Option::None, Option::Some(oplmnwact_bytes.as_slice()));
    for b in apdu.to_bytes().unwrap() {
        print!("{:02x}", b);
    }
    println!();
}
