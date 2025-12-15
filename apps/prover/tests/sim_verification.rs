// apps/prover/tests/sim_verification.rs
// =================================================================
// APARATO: PROOF SIMULATOR
// OBJETIVO: Verificar que la aguja generada es detectable por el filtro
// =================================================================

use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;

#[test]
fn test_golden_ticket_detectability() {
    // 1. Setup del Escenario
    let phrase = "GOLD777TEST";

    // 2. Derivación
    let pk = phrase_to_private_key(phrase);
    let pubk = SafePublicKey::from_private(&pk);
    let address = pubkey_to_address(&pubk, false);

    // 3. Creación del Filtro
    let mut filter = ShardedFilter::new(4, 100, 0.00001);

    // 4. Inyección
    filter.add(&address);

    // 5. Verificación (Assert)
    // El filtro DEBE retornar true para esta dirección.
    assert!(
        filter.contains(&address),
        "El filtro falló en detectar la dirección conocida (False Negative)"
    );

    // 6. Control Negativo
    // Una dirección diferente NO debe ser detectada (salvo colisión infinitesimal)
    let fake_phrase = "SILVER888TEST";
    let fake_pk = phrase_to_private_key(fake_phrase);
    let fake_pubk = SafePublicKey::from_private(&fake_pk);
    let fake_address = pubkey_to_address(&fake_pubk, false);

    assert!(
        !filter.contains(&fake_address),
        "El filtro detectó un falso positivo obvio"
    );
}
