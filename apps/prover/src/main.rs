// apps/prover/src/main.rs
// =================================================================
// APARATO: SYSTEM INTEGRITY PROVER (THE GOLDEN TICKET)
// RESPONSABILIDAD: GENERACIÓN DE ESCENARIOS DE PRUEBA DETERMINISTAS
// USO: cargo run --bin prover -- --output ./dist/proof
// =================================================================

use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::path::PathBuf;
use uuid::Uuid; // ✅ Importación explícita para claridad

// Importaciones del Núcleo
use prospector_core_gen::address_legacy::pubkey_to_address;
use prospector_core_math::public_key::SafePublicKey;
use prospector_core_probabilistic::sharded::ShardedFilter;
use prospector_domain_strategy::brainwallet::phrase_to_private_key;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Directorio donde se guardarán los shards de prueba
    #[arg(short, long, default_value = "dist/filters_proof")]
    output: PathBuf,

    /// Prefijo para la Brainwallet (Semilla)
    #[arg(long, default_value = "GOLD")]
    prefix: String,

    /// Número objetivo dentro del rango
    #[arg(long, default_value = "777")]
    target: String,
}

fn main() -> Result<()> {
    // Inicialización de logs
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    let args = Args::parse();

    info!("🧪 INICIANDO SECUENCIA DE CERTIFICACIÓN (PROVER)");
    info!("--------------------------------------------------");

    // 1. GENERACIÓN DEL SECRETO (LA AGUJA)
    let suffix = "TEST";
    let phrase = format!("{}{}{}", args.prefix, args.target, suffix);

    info!(
        "🔑 Generando material criptográfico para frase: '{}'",
        phrase
    );

    // 2. DERIVACIÓN CRIPTOGRÁFICA (Simulando lo que haría el usuario real)
    // Usamos el mismo motor matemático que el minero para garantizar compatibilidad.
    let pk = phrase_to_private_key(&phrase);
    let pubk = SafePublicKey::from_private(&pk);

    // Generamos dirección Legacy (Uncompressed) que es el estándar de 2009-2012
    let address = pubkey_to_address(&pubk, false);
    let wif = prospector_core_gen::wif::private_to_wif(&pk, false);

    println!("\n--- 📝 ARTEFACTOS DEL OBJETIVO ---");
    println!("Address:      {}", address);
    println!("Private Key:  {}", wif);
    println!("Entropy:      SHA256(\"{}\")", phrase);
    println!("----------------------------------\n");

    // 3. GENERACIÓN DEL FILTRO SINTÉTICO (EL MAPA)
    // Creamos un entorno controlado: Filtro particionado con solo 1000 elementos
    // pero configurado idéntico a producción (4 shards).
    if args.output.exists() {
        std::fs::remove_dir_all(&args.output)?;
    }
    std::fs::create_dir_all(&args.output)?;

    info!("🧠 Construyendo ShardedFilter (4 particiones)...");
    let mut filter = ShardedFilter::new(4, 1000, 0.00001);

    // Inyectamos la aguja en el pajar vacío
    filter.add(&address);

    info!("💾 Persistiendo filtros en {:?}...", args.output);
    filter
        .save_to_dir(&args.output)
        .context("Fallo al guardar shards")?;

    // 4. GENERACIÓN DE INSTRUCCIONES DE OPERACIÓN (SQL)
    // Calculamos un rango que contenga nuestro número objetivo.
    // Si target es 777, rango 700-800.
    let target_num: u64 = args.target.parse().unwrap_or(777);
    let range_start = target_num - 50;
    let range_end = target_num + 50;

    // Generamos un ID único para el job de prueba
    let job_id = Uuid::new_v4();

    println!("\n✅ ESCENARIO LISTO. EJECUTA ESTA QUERY EN TURSO:");
    println!("==================================================");

    println!(
        r#"
    -- Inyectar trabajo de prueba
    INSERT INTO jobs (id, range_start, range_end, status, created_at)
    VALUES (
        '{}', -- UUID Generado
        '{}',
        '{}',
        'pending',
        CURRENT_TIMESTAMP
    );
    "#,
        job_id, range_start, range_end
    );

    println!("\nℹ️ CONFIGURACIÓN DEL ORQUESTADOR:");
    println!(
        "   Asegúrate de que el Minero descargue los filtros desde: {:?}",
        args.output
    );
    println!("   Estrategia: Combinatoric");
    println!("   Prefix: '{}'", args.prefix);
    println!("   Suffix: '{}'", suffix);

    Ok(())
}
