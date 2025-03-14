pub fn print_keys() {
    println!(
        "lido_calculator:           {}",
        lido_calculator_lib::program::ID
    );
    println!(
        "lido_calculator state:     {}",
        lido_calculator_lib::program::LIDO_CALCULATOR_STATE_ID
    );

    println!(
        "spl_calculator:            {}",
        spl_calculator_lib::program::ID
    );
    println!(
        "spl_calculator state:      {}",
        spl_calculator_lib::program::SPL_CALCULATOR_STATE_ID
    );

    println!(
        "marinade_calculator:       {}",
        marinade_calculator_lib::program::ID
    );
    println!(
        "marinade_calculator state: {}",
        marinade_calculator_lib::program::MARINADE_CALCULATOR_STATE_ID
    );

    println!(
        "wsol_calculator:           {}",
        wsol_calculator_lib::program::ID
    );

    println!("");

    println!("stsol:                     {}", lido_keys::stsol::ID);
    println!("msol:                      {}", marinade_keys::msol::ID);
    println!("wsol:                      {}", wsol_keys::wsol::ID);

    println!("");
    println!("lido_program:              {}", lido_keys::lido_program::ID);
    println!(
        "lido_program_progdata:     {}",
        lido_keys::lido_program_progdata::ID
    );
    println!("lido_state:                {}", lido_keys::lido_state::ID);

    println!("");

    println!(
        "marinade_program:          {}",
        marinade_keys::marinade_program::ID
    );
    println!(
        "marinade_program_progdata: {}",
        marinade_keys::marinade_program_progdata::ID
    );
    println!(
        "marinade_state:            {}",
        marinade_keys::marinade_state::ID
    );

    println!("");

    println!(
        "spl_stake_pool_program:    {}",
        spl_stake_pool_keys::spl_stake_pool_program::ID
    );
    println!(
        "spl_stake_pool_program_progdata: {}",
        spl_stake_pool_keys::spl_stake_pool_program_progdata::ID
    );

    println!("");

    println!("");
    println!(
        "sanctum_spl_stake_pool_program: {}",
        sanctum_spl_stake_pool_keys::sanctum_spl_stake_pool_program::ID
    );
    println!(
        "sanctum_spl_stake_pool_program_progdata: {}",
        sanctum_spl_stake_pool_keys::sanctum_spl_stake_pool_program_progdata::ID
    );

    println!("");
    println!(
        "sanctum_spl_multi_stake_pool_program: {}",
        sanctum_spl_multi_stake_pool_keys::sanctum_spl_multi_stake_pool_program::ID
    );
    println!(
        "sanctum_spl_multi_stake_pool_program_progdata: {}",
        sanctum_spl_multi_stake_pool_keys::sanctum_spl_multi_stake_pool_program_progdata::ID
    );

    //  # jitosol:                   J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn
    //  # jito_stake_pool:           Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb
}

#[cfg(test)]
mod test_print_keys {
    use super::print_keys;

    #[test]
    fn test_print_keys() {
        print_keys();
    }
}
