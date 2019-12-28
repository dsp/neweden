/*
 * Copyright (c) 2019. David "Tiran'Sol" Soria Parra
 * All rights reserved.
 */

use crate::types;

pub fn allows_cynos(system: &types::System) -> bool {
    let sec_class = types::SecurityClass::from(system.security.clone());
    let sys_class = types::SystemClass::from(system);
    match (sys_class, sec_class) {
        (types::SystemClass::KSpace, types::SecurityClass::Highsec) => false,
        (types::SystemClass::KSpace, types::SecurityClass::Lowsec) => true,
        (types::SystemClass::KSpace, types::SecurityClass::Nullsec) => true,
        (types::SystemClass::WSpace, _) => false,
    }
}
