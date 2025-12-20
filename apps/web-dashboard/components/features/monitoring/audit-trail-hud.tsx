/**
 * =================================================================
 * APARATO: AUDIT TRAIL STRATEGIC HUD (V40.0)
 * CLASIFICACIÓN: FEATURE UI (ESTRATO L5)
 * RESPONSABILIDAD: VISUALIZACIÓN DE HUELLA FORENSE EN TIEMPO REAL
 * =================================================================
 */

export function AuditTrailHUD() {
  const { audit_history, is_connected } = useNeuralLink();

  // Motor de formateo para la Tesis (Hashes a unidades científicas)
  const formatEffort = (hashes: string) => {
    const n = BigInt(hashes);
    if (n > 1_000_000_000_000n) return `${(Number(n) / 1e12).toFixed(2)} TH`;
    if (n > 1_000_000_000n) return `${(Number(n) / 1e9).toFixed(2)} GH`;
    return `${(Number(n) / 1e6).toFixed(2)} MH`;
  };

  return (
    <div className="bg-black/80 border border-zinc-800 rounded-2xl overflow-hidden shadow-2xl flex flex-col h-full">
      <header className="p-4 border-b border-white/5 bg-white/2 flex justify-between items-center">
        <div className="flex items-center gap-3">
          <ShieldCheck className="w-5 h-5 text-blue-500" />
          <h2 className="text-xs font-black text-white uppercase tracking-[0.3em] font-mono">
            Immutable Audit Ledger // Stratum L4
          </h2>
        </div>
        <div className="flex items-center gap-2 px-3 py-1 bg-blue-500/10 rounded-full border border-blue-500/20">
          <div className="w-1.5 h-1.5 rounded-full bg-blue-500 animate-pulse" />
          <span className="text-[9px] font-bold text-blue-400 font-mono">NEURAL_LINK_ACTIVE</span>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto custom-scrollbar font-mono">
        <table className="w-full text-left border-collapse">
          <thead className="sticky top-0 bg-[#050505] z-10 shadow-sm">
            <tr className="text-[8px] font-black text-zinc-600 uppercase border-b border-zinc-800">
              <th className="p-4">Mission ID</th>
              <th className="p-4 text-center">Volume (Effort)</th>
              <th className="p-4">Forensic Footprint</th>
              <th className="p-4 text-right">Status</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-white/5">
            {audit_history.map((report) => (
              <tr key={report.job_mission_identifier} className="hover:bg-white/2 transition-colors group">
                <td className="p-4 text-[10px] text-blue-400 font-bold">
                  {report.job_mission_identifier.substring(0, 8).toUpperCase()}
                </td>
                <td className="p-4 text-center text-zinc-300 text-[10px] font-black">
                  {formatEffort(report.computational_effort_volume)}
                </td>
                <td className="p-4">
                  <div className="flex items-center gap-2 bg-black/40 border border-white/5 px-2 py-1 rounded w-fit group-hover:border-blue-500/30">
                    <Fingerprint className="w-3 h-3 text-zinc-700" />
                    <span className="text-[9px] text-zinc-500 truncate max-w-[120px]">
                      0x{report.audit_footprint_checkpoint}
                    </span>
                  </div>
                </td>
                <td className="p-4 text-right">
                  <span className="px-2 py-0.5 bg-emerald-500/10 border border-emerald-500/20 text-emerald-500 rounded text-[8px] font-black uppercase">
                    {report.final_mission_status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
