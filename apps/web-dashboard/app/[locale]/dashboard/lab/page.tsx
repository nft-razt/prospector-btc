import { ScenarioCreator } from "@/components/features/lab/scenario-creator";
import { ScenarioList } from "@/components/features/lab/scenario-list";
import { ManualVerifier } from "@/components/features/lab/manual-verifier"; // ✅ Import

export default function LabPage() {
  return (
    <div className="space-y-8">
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Columna Izquierda: Creación */}
        <div className="lg:col-span-1 space-y-6">
          <ScenarioCreator />
          <ManualVerifier /> {/* ✅ Nueva Herramienta */}
        </div>

        {/* Columna Derecha: Lista */}
        <div className="lg:col-span-2">
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-lg font-bold text-white font-mono uppercase tracking-widest">
              Active Experiments
            </h2>
          </div>
          <ScenarioList />
        </div>
      </div>
    </div>
  );
}
